//! A view used by officers to create/edit problems.

use acm::models::{Activity, Meeting};
use monaco::api::TextModel;

use gloo_net::http::Request;
use serde_json::Value;

use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with_deps, Suspense};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{CodeEditor, Navbar, Tabbed, TestsEditor},
    helpers::parse_markdown,
    helpers::themed_editor_with_model,
    state::State,
    Route,
};

#[function_component]
fn MarkdownEditor() -> Html {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();
    let description =
        TextModel::create(&state.problem_editor.description, Some("markdown"), None).unwrap();

    let preview = use_state(|| false);

    let options = themed_editor_with_model(description.clone());

    let preview_tmp = preview.clone();
    html! {
        <div class="markdown-editor card" onfocusout={dispatch.reduce_mut_callback(move |state| {
            state.problem_editor.description = description.get_value()
        })}>
            <div class="top-row">
                <span>{ "Problem Description" }</span>

                <button class="button grey" onclick={ Callback::from(move |_| preview_tmp.set(!*preview_tmp)) }>
                    { if *preview { "hide preview" } else { "show preview" } }
                </button>
            </div>

            if *preview {
                {{
                    let div = web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .create_element("div")
                        .unwrap();

                    div.set_inner_html(&parse_markdown(&state.problem_editor.description));
                    div.set_class_name("markdown-preview");

                    Html::VRef(div.into())
                }}
            } else {
                <CodeEditor options = {options} />
            }
        </div>
    }
}

async fn submit_problem_request(token: String, navigator: Navigator) -> Option<()> {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();

    let res: Value = Request::post("/api/problems/new")
        .header("Authorization", &format!("Bearer {}", token))
        .json(&state.problem_editor)
        .ok()?
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    if let Some(id) = res.get("id") {
        dispatch.reduce_mut(|state| state.problem_editor = Default::default());
        navigator.push(&Route::Problem {
            id: id.as_i64().unwrap(),
        })
    } else {
        dispatch.reduce_mut(|state| state.error = Some(res["error"].as_str().unwrap().to_string()));
    }

    Some(())
}

fn submit_problem(token: String, navigator: Navigator) {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();

    if state.problem_editor.title.is_empty()
        || state.problem_editor.description.is_empty()
        || state.problem_editor.runner.is_empty()
        || state.problem_editor.template.is_empty()
    {
        dispatch.reduce_mut(|state| {
            state.error = Some("One or more required fields is empty.".to_string())
        });
        return;
    }

    let token = token.clone();
    let navigator = navigator.clone();
    spawn_local(async move {
        if let None = submit_problem_request(token, navigator).await {
            dispatch.reduce_mut(|state| {
                state.error = Some("Encountered an error while submitting problem".to_string())
            });
        };
    });
}

#[function_component]
fn MeetingActivitySelector() -> HtmlResult {
    let meetings = use_future(|| async move {
        Request::get("/api/meetings")
            .send()
            .await?
            .json::<Vec<Meeting>>()
            .await
    })?;

    let meeting_id = use_state(|| -1);

    let dispatch = Dispatch::<State>::new();

    let update_meeting = {
        let meeting_id = meeting_id.clone();
        Callback::from(move |e: InputEvent| {
            let res = e.target_unchecked_into::<HtmlSelectElement>().value();
            meeting_id.set(res.parse().unwrap());
        })
    };

    let meetings_list = match &*meetings {
        Ok(list) => list
            .iter()
            .map(|m| {
                html! {
                    <option value={ m.id.to_string() }>{ &m.title }</option>
                }
            })
            .collect::<Html>(),
        Err(_) => html! {},
    };

    Ok(html! {
        <div class="activity-selector">
            <label>{ "Meeting: " }</label>
            <select class="acm-input" oninput={update_meeting}>
                <option value="-1" selected={true}>{ "None" }</option>

                { meetings_list }
            </select>

            if *meeting_id != -1 {
                <Suspense>
                    <ActivitySelector meeting_id={*meeting_id} />
                </Suspense>
            }
        </div>
    })
}

#[derive(Properties, PartialEq)]
struct ActivitySelectorProps {
    meeting_id: i64,
}

#[function_component]
fn ActivitySelector(props: &ActivitySelectorProps) -> HtmlResult {
    let activities = use_future_with_deps(
        |meeting_id| async move {
            Request::get(&format!("/api/meetings/{}/activities", meeting_id))
                .send()
                .await?
                .json::<Vec<Activity>>()
                .await
        },
        props.meeting_id,
    )?;

    let dispatch = Dispatch::<State>::new();

    let update_activity = dispatch.reduce_mut_callback_with(move |state, e: InputEvent| {
        let res = e.target_unchecked_into::<HtmlSelectElement>().value();
        state.problem_editor.activity_id = res.parse().ok();
    });

    let activities_list = match &*activities {
        Ok(list) => list
            .iter()
            .map(|a| {
                html! {
                    <option value={ a.id.to_string() }>{ &a.title }</option>
                }
            })
            .collect::<Html>(),
        Err(_) => html! {},
    };

    Ok(html! {
        <>
            <label>{ "Activity:" }</label>
            <select class="acm-input" oninput={update_activity}>
                <option value="None" selected={true}>{ "None" }</option>
                { activities_list }
            </select>
        </>
    })
}

#[function_component]
pub fn ProblemEditorView() -> Html {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();
    let navigator = use_navigator().unwrap();

    let title = state.problem_editor.title.clone();

    let token = use_selector(|state: &State| {
        if let Some(session) = &state.session {
            session.token.clone()
        } else {
            String::new()
        }
    });

    let runner_code = TextModel::create(&state.problem_editor.runner, Some("cpp"), None).unwrap();
    let template_code =
        TextModel::create(&state.problem_editor.template, Some("cpp"), None).unwrap();

    let runner_editor_options = themed_editor_with_model(runner_code.clone());
    let template_editor_options = themed_editor_with_model(template_code.clone());

    // This basically just takes all of the entered data by the user and submits that to the
    // server, thereby creating a problem or an error.
    let create_problem =
        Callback::from(move |_| submit_problem(token.to_string(), navigator.clone()));

    html! {
        <div class="container">
            <Navbar />

            <div class="problem-editor-wrapper">
                <div class="problem-editor-sidebar">
                <input value={title} placeholder="Title" class="title-input acm-input card" oninput={dispatch.reduce_mut_callback_with(|state, e: InputEvent| {
                    let title = e.target_unchecked_into::<HtmlInputElement>().value();
                    state.problem_editor.title = title;
                })} />
                    <MarkdownEditor />
                </div>

                <div class="problem-editor-content">
                    <Tabbed class="card" titles={ vec!["Runner", "Template", "Tests"] }>
                        <div onfocusout={dispatch.reduce_mut_callback(move |state| {
                            state.problem_editor.runner = runner_code.get_value()
                        })}><CodeEditor options = { runner_editor_options } /></div>
                        <div onfocusout={dispatch.reduce_mut_callback(move |state| {
                            state.problem_editor.template = template_code.get_value()
                        })}><CodeEditor options = { template_editor_options } /></div>
                        <TestsEditor />
                    </Tabbed>

                    <div class="code-runner-wrapper">
                        <Suspense fallback={html!{<div class="activity-selector" />}}>
                            <MeetingActivitySelector />
                        </Suspense>
                        <button class="button green" onclick={create_problem}>{ "Submit" }</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
