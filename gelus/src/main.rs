use acm::models::{Submission, User, forms::FirstTimeCompletionsForm};
use chrono::{Utc, NaiveDateTime};
use futures::future::join_all;
use clap::Parser;

use serenity::{
    framework::standard::{CommandResult, StandardFramework, macros::{command, group}},
    model::{Timestamp, channel::Message, id::ChannelId},
    prelude::*,
    utils::MessageBuilder,
    async_trait,
    http::Http,
};
use tokio::{
    task,
    time::{self, Duration},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The client secret of the discord bot
    #[clap(short, long)]
    token: String,

    /// The ID of the channel to send updates in
    #[clap(short, long)]
    channel_id: u64
}

#[group]
#[commands(start)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    let args = Args::parse();
    let token = args.token;

    tracing::info!("Starintg client.");

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    tracing::info!("Starting client.");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        tracing::error!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn start(ctx: &Context, _msg: &Message) -> CommandResult {
    let http = ctx.http.clone();

    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(10));


        let mut prev_time = Utc::now().naive_local();

        loop {
            interval.tick().await;
            check_new_messages(&http, prev_time).await;
            prev_time = Utc::now().naive_local();
        }
    });

    Ok(())
}

async fn check_new_messages(http: &Http, prev_time: NaiveDateTime) {
    tracing::info!("Checking new messages");

    let client = reqwest::Client::new();
    let res = client.get("http://localhost:8081/submissions/new-completions")
        .query(&FirstTimeCompletionsForm { since: Some(prev_time) })
        .send()
        .await;

    let res = match res {
        Ok(res) => res,
        Err(why) => {
            tracing::error!("Error fetching new submissions from server: {why:?}");
            return;
        }
    };

    let data = res
        .json::<Vec<(User, Submission)>>()
        .await
        .expect("Did not serialize properly");

    let args = Args::parse();
    let channel_id = args.channel_id;

    join_all(data.into_iter().map(|(user, submission)| {
        send_update_message(http, channel_id, user, submission)
    })).await;
}

async fn send_update_message(http: &Http, channel_id: u64, user: User, submission: Submission) {
    let code = MessageBuilder::new()
        .push_codeblock_safe(submission.code, Some("cpp"))
        .build();

    let err = ChannelId(channel_id).send_message(http, |m| {
            m.content(&format!("{} just completed a problem!", user.name)).embed(|e| {
                e.title(&format!("Problem {}", submission.problem_id))
                    .description(code)
                    .fields(vec![
                        ("Runtime", format!("{}ms", submission.runtime), true),
                        ("Username", user.username, true),
                    ])
                    .timestamp(Timestamp::now())
            })
        })
        .await;

    if let Err(why) = err {
        tracing::error!("Error occured sending message: {why:?}");
    }
}
