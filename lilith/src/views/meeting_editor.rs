use acm::models::{Activity, ActivityType};
use chrono::NaiveDateTime;
use gloo_net::http::Request;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{api_url, components::Navbar, state::State, Route};

#[derive(PartialEq, Properties)]
pub struct ActivityEntryProps {
    id: i64,
    index: usize,
    activity: Activity,
}

#[function_component]
fn ActivityEntry(props: &ActivityEntryProps) -> Html {
    let dispatch = Dispatch::<State>::new();
    let id = props.id;
    let index = props.index;

    let update_title = dispatch.reduce_mut_callback_with(move |state, e: InputEvent| {
        let title = e.target_unchecked_into::<HtmlInputElement>().value();
        state.meeting_editor.entry(id).or_insert_with(Default::default).activities[index].title = title;
    });

    let update_description = dispatch.reduce_mut_callback_with(move |state, e: InputEvent| {
        let description = e.target_unchecked_into::<HtmlTextAreaElement>().value();
        state.meeting_editor.entry(id).or_insert_with(Default::default).activities[index].description = description;
    });

    let update_activity_type = dispatch.reduce_mut_callback_with(move |state, e: InputEvent| {
        let activity_type = e.target_unchecked_into::<HtmlSelectElement>().value();

        let activity_type = match activity_type.as_str() {
            "LECT" => ActivityType::LECT,
            "SOLO" => ActivityType::SOLO,
            "PAIR" => ActivityType::PAIR,
            _ => panic!("THIS SHOULD NOT HAPPEN"),
        };

        state.meeting_editor.entry(id).or_insert_with(Default::default).activities[index].activity_type = activity_type;
    });

    let form_classes="border-neutral-300 border rounded p-2 bg-neutral-50 outline-0 transition-shadow focus:ring ring-neutral-300";

    html! {
        <div class="bg-white border-y md:border border-neutral-300 p-2 md:rounded-md grid grid-cols-2 gap-2">
            <div class="flex flex-col">
                <label>{"Name"}</label>
                <input oninput={update_title} value={props.activity.title.to_string()} class={form_classes} />
            </div>

            <div class="flex flex-col">
                <label>{"Type"}</label>
                <select class={form_classes} oninput={update_activity_type}>
                    <option value="LECT" selected={props.activity.activity_type == ActivityType::LECT}>{ "Lecture" }</option>
                    <option value="PAIR" selected={props.activity.activity_type == ActivityType::PAIR}>{ "Pair Programming" }</option>
                    <option value="SOLO" selected={props.activity.activity_type == ActivityType::SOLO}>{ "Solo Competition" }</option>
                </select>
            </div>

            <div class="flex flex-col col-span-2">
                <label>{"Description"}</label>
                <textarea oninput={update_description} value={props.activity.description.clone()} class={form_classes}></textarea>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct MeetingEditorViewProps {
    pub id: Option<i64>,
}

#[function_component]
fn ActivitiesEditor(props: &MeetingEditorViewProps) -> Html {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();

    let id = props.id.unwrap_or(-1);

    // Rerender whenever the number of activities changes
    use_selector(move |state: &State| Some(state.meeting_editor.get(&id)?.activities.len()));

    let add_activity = dispatch.reduce_mut_callback(move |state| {
        state
            .meeting_editor
            .entry(id)
            .or_insert_with(Default::default)
            .activities
            .push(Default::default());
    });

    let remove_activity = dispatch.reduce_mut_callback(move |state| {
        state.meeting_editor.entry(id).or_insert_with(Default::default).activities.pop();
    });

    let activities_html = state.meeting_editor[&id]
        .activities
        .iter()
        .enumerate()
        .map(|(index, activity)| {
            html! {<>
                if index == 0 {
                    <h2 class="text-lg font-bold">{ "Activities" }</h2>
                }

                <ActivityEntry {index} {id} activity={activity.clone()} />
            </>}
        })
        .collect::<Html>();

    html! {
        <>
            { activities_html }

            <div class="flex gap-2 mx-2 md:m-0">
                <button class="transition-shadow rounded-md border hover:ring px-4 py-2 border-blue-700 bg-blue-500 ring-blue-700 text-blue-50" onclick={add_activity}>{"Add activity"}</button>
                <button class="transition-shadow rounded-md border hover:ring px-4 py-2 border-red-700 bg-red-500 ring-red-700 text-red-50" onclick={remove_activity}>{"Remove activity"}</button>
            </div>
        </>
    }
}

async fn submit_create_meeting_request(id: i64, token: String, navigator: Navigator) -> Option<()> {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();

    let res: Value = Request::post(api_url!("/meetings/edit"))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&state.meeting_editor[&id])
        .ok()?
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    if let Some(new_id) = res.get("id") {
        dispatch.reduce_mut(|state| state.meeting_editor.remove(&id));
        navigator.push(&Route::Meeting {
            id: new_id.as_i64().unwrap(),
        });
    } else {
        dispatch.reduce_mut(|state| state.error = Some(res["error"].as_str().unwrap().to_string()));
    }

    Some(())
}

fn submit_create_meeting(id: i64, token: String, navigator: Navigator) {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();
    let meeting = &state.meeting_editor[&id];

    if meeting.title.is_empty() || meeting.description.is_empty() {
        dispatch.reduce_mut(|state| {
            state.error = Some("One or more required fields is empty.".to_string())
        });
        return;
    }

    for activity in &meeting.activities {
        if activity.title.is_empty() || activity.description.is_empty() {
            dispatch.reduce_mut(|state| {
                state.error = Some("One or more required fields is empty.".to_string())
            });
            return;
        }
    }

    let token = token.clone();
    let navigator = navigator.clone();
    spawn_local(async move {
        if let None = submit_create_meeting_request(id, token, navigator).await {
            dispatch.reduce_mut(|state| {
                state.error = Some("Encountered an error while submitting problem".to_string())
            });
        };
    });
}

#[function_component]
pub fn MeetingEditorView(props: &MeetingEditorViewProps) -> Html {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();
    let navigator = use_navigator().unwrap();

    let id = props.id.unwrap_or(-1);

    let token = use_selector(|state: &State| {
        if let Some(session) = &state.session {
            session.token.clone()
        } else {
            String::new()
        }
    });

    let form = if let Some(form) = state.meeting_editor.get(&id) {
        form.clone()
    } else {
        dispatch.reduce_mut(move |state| state.meeting_editor.insert(id, Default::default()));
        Default::default()
    };

    let update_title = dispatch.reduce_mut_callback_with(move |state, e: InputEvent| {
        let title = e.target_unchecked_into::<HtmlInputElement>().value();
        state.meeting_editor.entry(id).or_insert_with(Default::default).title = title;
    });

    let update_description = dispatch.reduce_mut_callback_with(move |state, e: InputEvent| {
        let description = e.target_unchecked_into::<HtmlTextAreaElement>().value();
        state.meeting_editor.entry(id).or_insert_with(Default::default).description = description;
    });

    let update_meeting_time = dispatch.reduce_mut_callback_with(move |state, e: InputEvent| {
        let meeting_time = e.target_unchecked_into::<HtmlInputElement>().value();
        log::info!("meeting time: {}", meeting_time);
        state
            .meeting_editor
            .entry(id)
            .or_insert_with(Default::default)
            .meeting_time = NaiveDateTime::parse_from_str(&meeting_time, "%Y-%m-%dT%H:%M")
            .expect("Failed to parse time");
    });

    let submit = Callback::from(move |_| {
        submit_create_meeting(id, token.to_string(), navigator.clone());
    });

    let time = form.meeting_time.format("%Y-%m-%dT%H:%M").to_string();

    let form_classes="border-neutral-300 border rounded p-2 bg-neutral-50 outline-0 transition-shadow focus:ring ring-neutral-300";

    html! {
        <>
            <Navbar />

            <div class="max-w-screen-md mx-auto my-2 flex flex-col gap-2">
                <div class="flex items-center mx-2 md:m-0">
                    <h1 class="text-2xl font-bold">{ "New Meeting" }</h1>
                    <button onclick={submit} class="ml-auto bg-green-700 hover:bg-green-500 text-green-50 transition-colors rounded-full px-4 py-2 text-sm">{ "Submit" }</button>
                </div>

                <div class="bg-white border-y md:border border-neutral-300 p-2 md:rounded-md grid grid-cols-2 gap-2">
                    <div class="flex flex-col">
                        <label>{"Title"}</label>
                        <input class={form_classes} value={form.title.clone()} oninput={update_title} />
                    </div>

                    <div class="flex flex-col">
                        <label>{"Meeting Time"}</label>
                        <input class={form_classes} type="datetime-local" value={time} oninput={update_meeting_time} />
                    </div>

                    <div class="flex flex-col col-span-2">
                        <label>{"Description"}</label>
                        <textarea class={form_classes} value={form.description.clone()} oninput={update_description} />
                    </div>
                </div>

                <ActivitiesEditor id={props.id} />

            </div>
        </>
    }
}
