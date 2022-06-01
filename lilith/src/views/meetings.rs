use acm::models::{Activity, Meeting};
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

    // update time whenever prop is changed
    {
        let duration = duration.clone();
        let event_time = props.event_time.clone();

        use_effect_with_deps(
            move |_| {
                duration.set(event_time.signed_duration_since(Utc::now().naive_local()));
                || ()
            },
            props.event_time,
        );
    }

    // rerender the component once per second
    {
        let duration = duration.clone();
        let event_time = props.event_time.clone();
        use_effect(move || {
            let timeout = Timeout::new(1000, move || {
                duration.set(event_time.signed_duration_since(Utc::now().naive_local()));
            });

            || {
                timeout.cancel();
            }
        });
    }

    let seconds = (duration.num_seconds() % 60).abs();
    let minutes = (duration.num_minutes() % 60).abs();
    let hours = (duration.num_hours() % 24).abs();
    let days = duration.num_days().abs();

    html! {
        <div class="countdown">
            if duration.num_seconds() < 0 {
                <h3>{ "Started..." }</h3>
            } else {
                <h3>{ "Starts in..." }</h3>
            }

            if days != 0 {
                <span>
                    { days } { " days " }
                </span>
            }

            if days == 0 && hours != 0 {
                <span>
                    { hours } { " hours " }
                </span>
            }

            if minutes != 0 {
                <span>
                    { minutes } { " minutes " }
                </span>
            }

            <span>
                { " and " } { seconds } { " seconds " }
            </span>

            if duration.num_seconds() < 0 {
                <span>{ "ago." }</span>
            }
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
                        <span class="subtitle">{ m.meeting_time.format("%A, %B %-d @ %-I:%M %p") }</span>
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
    meeting_id: i64,
}

#[function_component]
fn Activities(props: &ActivitiesProps) -> HtmlResult {
    let id = props.meeting_id;

    let activities = use_future(|| async move {
        Request::get(&format!("/api/meetings/{}/activities", id))
            .send()
            .await?
            .json::<Vec<Activity>>()
            .await
    })?;

    // this is pretty bad but idk how else to do this
    let vec_ref = vec![];
    let activities = (*activities).as_ref().unwrap_or(&vec_ref);

    if activities.is_empty() {
        return Ok(html! {});
    }

    let activities_html = activities
        .iter()
        .map(|activity| {
            html! {
                <div class="padded card">
                    <h3>{ &activity.title }</h3>

                    <span class="subtitle">{ &activity.description }</span>
                </div>
            }
        })
        .collect::<Html>();

    Ok(html! {
        <>
            <h2>{ "Activities" }</h2>

            <div class="activities-wrapper">
                { activities_html }
            </div>
        </>
    })
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
        "/api/meetings/next".to_string()
    };

    let meeting = use_future_with_deps(
        |_| async move { Request::get(&url).send().await?.json::<Meeting>().await },
        props.id,
    )?;

    let meeting_html = match &*meeting {
        Ok(meeting) => html! {
            <>
                <h1>{ &meeting.title }</h1>
                <span class="subtitle">{ meeting.meeting_time.format("%A, %B %-d @ %-I:%M %p") }</span>

                <Countdown event_time={ meeting.meeting_time } />

                <Suspense>
                    <Activities meeting_id={meeting.id} />
                </Suspense>
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
