use actix_files::{Files, NamedFile};
use actix_web::dev::{fn_service, ServiceRequest, ServiceResponse};
use actix_web::{
    get, middleware::Logger, web, web::Json, App, HttpResponse, HttpServer, Responder, Result,
};
use log::info;
use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct LeaderboardUser {
    name: String,
    username: String,
    star_count: i32,
}

#[get("/leaderboard")]
async fn leaderboard() -> impl Responder {
    let mut data = ["Miles", "Aidan", "Alex", "Evan", "Meher", "Kevin", "Reema"]
        .iter()
        .map(|name| LeaderboardUser {
            name: name.to_string(),
            username: name.to_string().to_lowercase(),
            star_count: thread_rng().gen_range(0..20),
        })
        .collect::<Vec<LeaderboardUser>>();

    data.sort_unstable_by(|a, b| b.star_count.cmp(&a.star_count));

    Json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api").service(leaderboard))
            .service(
                Files::new("/", "./dist/")
                    .index_file("index.html")
                    .default_handler(fn_service(|req: ServiceRequest| async {
                        let (req, _) = req.into_parts();
                        let file = NamedFile::open_async("./dist/index.html").await?;
                        let res = file.into_response(&req);
                        Ok(ServiceResponse::new(req, res))
                    })),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
