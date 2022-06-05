//! A view used by officers to create/edit problems.

use acm::models::{Activity, Meeting};
use monaco::api::TextModel;

use gloo_net::http::Request;
use monaco::sys::editor::IEditorOptionsWordWrap;
use serde_json::Value;

use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with_deps, Suspense};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    api_url,
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
    options.set_word_wrap(Some(IEditorOptionsWordWrap::On));

    let update_description = dispatch.reduce_mut_callback(move |state| {
        state.problem_editor.description = description.get_value()
    });

    let preview_tmp = preview.clone();
    html! {
        <div class="grid grid-rows-min-full grid-cols-full border-y border-neutral-300 bg-white" onfocusout={update_description}>
            <div class="flex items-center border-b border-neutral-300 p-2">
                <span class="font-bold">{ "Problem Description" }</span>

                <button class="ml-auto rounded-full bg-blue-700 hover:bg-blue-500 text-sm font-bold text-blue-50 transition-colors px-4 py-2"
                        onclick={ Callback::from(move |_| preview_tmp.set(!*preview_tmp)) }>
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
                    div.set_class_name("prose prose-neutral p-2 min-h-[40vh]");

                    Html::VRef(div.into())
                }}
            } else {
                <CodeEditor classes="h-[40vh] lg:h-full" options = {options} />
            }
        </div>
    }
}

async fn submit_problem_request(token: String, navigator: Navigator) -> Option<()> {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();

    let res: Value = Request::post(api_url!("/problems/new"))
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
        Request::get(api_url!("/meetings"))
            .send()
            .await?
            .json::<Vec<Meeting>>()
            .await
    })?;

    let meeting_id = use_state(|| -1);

    let _dispatch = Dispatch::<State>::new();

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
        <div class="w-full grid grid-cols-min-full lg:flex lg:g gap-2 border-b border-neutral-300 bg-white p-2 items-center">
            <label class="font-bold">{ "Meeting" }</label>
            <select class="border-neutral-300 border rounded p-2 bg-neutral-50 outline-0 transition-shadow focus:ring ring-neutral-300" oninput={update_meeting}>
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
            Request::get(api_url!("/meetings/{}/activities", meeting_id))
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
            <label class="font-bold">{ "Activity" }</label>
            <select class="border-neutral-300 border rounded p-2 bg-neutral-50 outline-0 transition-shadow focus:ring ring-neutral-300" oninput={update_activity}>
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

    let edit_title = dispatch.reduce_mut_callback_with(|state, e: InputEvent| {
        let title = e.target_unchecked_into::<HtmlInputElement>().value();
        state.problem_editor.title = title;
    });

    // This basically just takes all of the entered data by the user and submits that to the
    // server, thereby creating a problem or an error.
    let create_problem =
        Callback::from(move |_| submit_problem(token.to_string(), navigator.clone()));

    let update_runner = dispatch
        .reduce_mut_callback(move |state| state.problem_editor.runner = runner_code.get_value());

    let update_template = dispatch.reduce_mut_callback(move |state| {
        state.problem_editor.template = template_code.get_value()
    });

    html! {
        <div class="grid grid-rows-min-full grid-cols-full w-screen h-screen">
            <Navbar />

            <div class="flex flex-col gap-2 lg:gap-0 lg:grid lg:grid-cols-[450px_minmax(0,1fr)] lg:grid-rows-full-min">
                <div class="grid grid-rows-min-full grid-cols-full gap-2 lg:gap-0 lg:border-r border-neutral-300 row-span-2">
                    <div class="border-y lg:border-0 bg-white border-neutral-300 flex flex-col gap-2 p-2">
                        <label class="font-bold">{ "Title" }</label>
                        <input class="border-neutral-300 border rounded p-2 bg-neutral-50 outline-0 transition-shadow focus:ring ring-neutral-300"
                               oninput={edit_title} value={title} placeholder="Title" />
                    </div>
                    <MarkdownEditor />
                </div>

                <Tabbed class="border-y border-neutral-300 lg:border-0" titles={ vec!["Runner", "Template", "Tests"] }>
                    <div onfocusout={update_runner} class="h-[40vh] lg:h-full">
                        <CodeEditor classes="h-full" options = { runner_editor_options } />
                    </div>

                    <div onfocusout={update_template} class="h-[40vh] lg:h-full">
                        <CodeEditor classes="h-full" options = { template_editor_options } />
                    </div>

                    <TestsEditor />
                </Tabbed>

                <div class="border-t border-neutral-300 flex flex-col items-center gap-2 lg:bg-white lg:flex-row">
                    <Suspense fallback={html!{<div></div>}}>
                        <MeetingActivitySelector />
                    </Suspense>

                    <button class="lg:ml-auto rounded-full px-4 py-2 bg-green-600 hover:bg-green-500 text-green-50 transition-colors mx-2 mb-8 lg:m-0 lg:rounded-none lg:h-full" onclick={create_problem}>{ "Submit" }</button>
                </div>
            </div>
        </div>
    }
}
