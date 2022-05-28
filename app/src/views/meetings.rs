use acm::models::{Activity, Meeting, MeetingActivities};
use chrono::{NaiveDateTime, Utc};
use gloo_net::http::Request;
use gloo_timers::callback::Timeout;
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with_deps, Suspense};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{components::Navbar, helpers::is_officer, Route, State};

#[derive(PartialEq, Properties)]
struct CountdownProps {
    event_time: NaiveDateTime,
}

#[function_component]
fn Countdown(props: &CountdownProps) -> Html {
    let duration = use_state(|| {
        props
            .event_time
            .signed_duration_since(Utc::now().naive_local())
    });

    // rerender the component once per second
    {
        let duration = duration.clone();
        let event_time = props.event_time.clone();
        use_effect(move || {
            Timeout::new(1000, move || {
                duration.set(event_time.signed_duration_since(Utc::now().naive_local()));
            })
            .forget();

            || ()
        });
    }

    let seconds = duration.num_seconds() % 60;
    let minutes = duration.num_minutes() % 60;
    let hours = duration.num_hours() % 24;
    let days = duration.num_days();

    html! {
        <div>
            <h3>{ "The next meeting will start in..." }</h3>

            <span>
                { days } { " days " }
            </span>
            <span>
                { hours } { " hours " }
            </span>
            <span>
                { minutes } { " minutes " }
            </span>
            <span>
                { seconds } { " seconds " }
            </span>
        </div>
    }
}

#[function_component]
fn ScheduleList() -> HtmlResult {
    let session = use_selector(|state: &State| state.session.clone());

    let meetings = use_future(|| async move {
        Request::get("/api/meetings")
            .send()
            .await?
            .json::<Vec<Meeting>>()
            .await
    })?;

    let meetings_list = match &*meetings {
        Ok(list) => list
            .iter()
            .map(|m| {
                html! {
                    <Link<Route> to={Route::Meeting { id: m.id }} classes="padded schedule-item">
                        <h3>{ &m.title }</h3>
                        <span>{ m.meeting_time }</span>
                    </Link<Route>>
                }
            })
            .collect::<Html>(),
        Err(_) => html! {},
    };

    Ok(html! {
        <div class="schedule">
            <h2>
                { "Schedule" }
            </h2>

            <div class="card schedule-list">
                { meetings_list }
            </div>

            if is_officer(&*session) {
                <Link<Route> to={Route::MeetingEditorNew} classes="button green">{ "Add" }</Link<Route>>
            }
        </div>
    })
}

#[derive(PartialEq, Properties)]
struct ActivitiesProps {
    activities: Vec<Activity>,
}

#[function_component]
fn Activities(props: &ActivitiesProps) -> Html {
    if props.activities.is_empty() {
        return html! {};
    }

    let activities_html = props
        .activities
        .iter()
        .map(|activity| {
            html! {
                <div class="padded card">
                    <h2>{ &activity.title }</h2>

                    <span>{ &activity.description }</span>
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <>
            <h2>{ "Events" }</h2>

            <div class="activities-wrapper">
                { activities_html }
            </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct MeetingViewProps {
    #[prop_or_default]
    pub id: Option<i64>,
}

#[function_component]
fn MeetingView(props: &MeetingViewProps) -> HtmlResult {
    let id = props.id;

    let url = if let Some(id) = id {
        let mut url = "/api/meetings/".to_string();
        url.push_str(&id.to_string());
        url
    } else {
        "/api/next-meeting".to_string()
    };

    let meeting_activities = use_future_with_deps(
        |_| async move {
            Request::get(&url)
                .send()
                .await?
                .json::<MeetingActivities>()
                .await
        },
        props.id,
    )?;

    let meeting_html = match &*meeting_activities {
        Ok(meeting_activities) => html! {
            <>
                <h1>{ &meeting_activities.meeting.title }</h1>
                <Countdown event_time={meeting_activities.meeting.meeting_time} />

                <Activities activities={meeting_activities.activities.clone()} />
            </>
        },
        Err(_) => html! {},
    };

    Ok(html! {
        <div class="meetings-view-content">
            { meeting_html }

        </div>
    })
}

#[function_component]
pub fn MeetingsView(props: &MeetingViewProps) -> Html {
    let id = props.id;

    html! {
        <div class="container">
            <Navbar />

            <div class="meetings-view-wrapper">
                <Suspense fallback={html!{ <div></div> }}>
                    <MeetingView {id} />
                </Suspense>

                <Suspense>
                    <ScheduleList />
                </Suspense>
            </div>
        </div>
    }
}
