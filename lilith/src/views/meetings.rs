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
struct CountdownNumberProps {
    number: i64,
    description: &'static str,
}

#[function_component]
fn CountdownNumber(props: &CountdownNumberProps) -> Html {
    html! {
        <div class="flex flex-col items-center w-14">
            <span class="text-3xl font-bold">{ props.number }</span>
            <span class="text-sm font-bold">{ props.description }</span>
        </div>
    }
}

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
        <div class="flex gap-4 justify-center">
            <CountdownNumber number={days} description="days" />
            <CountdownNumber number={hours} description="hours" />
            <CountdownNumber number={minutes} description="minutes" />
            <CountdownNumber number={seconds} description="seconds" />
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
                    <Link<Route> to={Route::Meeting { id: m.id }}
                                 classes="border-b last:border-0 border-neutral-300 p-2 sm:first:rounded-t-md sm:last:rounded-b-md hover:bg-neutral-100 transition-colors">
                        <h3 class="font-bold">{ &m.title }</h3>
                        <span class="text-neutral-600 text-sm">{ m.meeting_time.format("%A, %B %-d @ %-I:%M %p") }</span>
                    </Link<Route>>
                }
            })
            .collect::<Html>(),
        Err(_) => html! {},
    };

    Ok(html! {
        <div class="sm:px-2 flex flex-col gap-2">
            <h2 class="text-2xl font-bold px-2 sm:p-0">{ "Schedule" }</h2>

            <div class="bg-white sm:rounded-md border-y sm:border border-neutral-300 flex flex-col">
                { meetings_list }
            </div>

            if is_officer(&*session) {
                <Link<Route> to={Route::MeetingEditorNew} classes="text-center rounded-full bg-green-700 hover:bg-green-500 transition-colors text-green-50 py-2 text-sm">{ "Add" }</Link<Route>>
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
                <div class="bg-white ring-1 sm:rounded-md ring-neutral-300 p-2">
                    <h3 class="text-lg font-bold">{ &activity.title }</h3>

                    <span class="text-neutral-500">{ &activity.description }</span>
                </div>
            }
        })
        .collect::<Html>();

    Ok(html! {
        <>
            <h2 class="font-bold text-xl px-2 sm:p-0">{ "Activities" }</h2>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
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
                <div class="px-2 sm:p-0">
                    <h1 class="text-2xl font-bold">{ &meeting.title }</h1>
                    <span class="text-neutral-600 text-sm">{ meeting.meeting_time.format("%A, %B %-d @ %-I:%M %p") }</span>
                </div>

                <Countdown event_time={ meeting.meeting_time } />

                <Suspense>
                    <Activities meeting_id={meeting.id} />
                </Suspense>
            </>
        },
        Err(_) => html! {
            "not found"
        },
    };

    Ok(html! {
        <div class="sm:px-2 flex flex-col gap-2">
            { meeting_html }
        </div>
    })
}

#[function_component]
pub fn MeetingsView(props: &MeetingViewProps) -> Html {
    let id = props.id;

    html! {
        <>
            <Navbar />

            <div class="grid grid-rows-[min-content_1fr] md:grid-cols-[1fr_300px] md:grid-rows-1 max-w-screen-lg mx-auto gap-2 my-2">
                <Suspense fallback={html!{ <div></div> }}>
                    <MeetingView {id} />
                </Suspense>

                <Suspense>
                    <ScheduleList />
                </Suspense>
            </div>
        </>
    }
}
