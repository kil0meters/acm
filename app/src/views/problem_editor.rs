//! A view used by officers to create/edit problems.

use monaco::api::{CodeEditorOptions, TextModel};

use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::{CodeEditor, ErrorBox, Modal, Navbar, Tabbed, TestsEditor},
    Route, state::State,
};

#[function_component]
fn MarkdownEditor() -> Html {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();
    let description = TextModel::create(&state.problem_editor.description, Some("markdown"), None).unwrap();

    let preview = use_state(|| false);

    let options =
        Rc::new(CodeEditorOptions::default().with_model(description.clone())).to_sys_options();

    options.set_font_size(Some(18.0));
    options.set_automatic_layout(Some(true));

    let preview_tmp = preview.clone();
    html! {
        <div class="markdown-editor" onfocusout={dispatch.reduce_mut_callback(move |state| {
            state.problem_editor.description = description.get_value()
        })}>
            <div class="top-row">
                <span>{ "problem description" }</span>

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

                    div.set_inner_html(&markdown::to_html(&state.problem_editor.description));
                    div.set_class_name("markdown-preview");

                    Html::VRef(div.into())
                }}
            } else {
                <CodeEditor options = {options} />
            }
        </div>
    }
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
    let template_code = TextModel::create(&state.problem_editor.template, Some("cpp"), None).unwrap();

    let error: UseStateHandle<Option<String>> = use_state(|| None);

    let runner_editor_options =
        Rc::new(CodeEditorOptions::default().with_model(runner_code.clone())).to_sys_options();
    runner_editor_options.set_font_size(Some(18.0));
    runner_editor_options.set_automatic_layout(Some(true));

    let template_editor_options =
        Rc::new(CodeEditorOptions::default().with_model(template_code.clone())).to_sys_options();
    template_editor_options.set_font_size(Some(18.0));
    template_editor_options.set_automatic_layout(Some(true));

    // This basically just takes all of the entered data by the user and submits that to the
    // server, thereby creating a problem or an error.
    let create_problem = {
        let error = error.clone();
        let dispatch = dispatch.clone();

        Callback::from(move |_| {
            if state.problem_editor.title.is_empty()
                || state.problem_editor.description.is_empty()
                || state.problem_editor.runner.is_empty()
                || state.problem_editor.template.is_empty()
            {
                error.set(Some("One or more required fields is empty.".to_string()));

                return;
            }

            let token = token.clone();
            let state = state.clone();
            let dispatch = dispatch.clone();
            let error = error.clone();
            let navigator = navigator.clone();
            spawn_local(async move {
                let client = reqwest::Client::new();
                let res: Value = client
                    .post("http://127.0.0.1:8080/api/create-problem")
                    .header(AUTHORIZATION, &format!("Bearer {}", token))
                    .json(&state.problem_editor)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                if let Some(id) = res.get("id") {
                    dispatch.reduce_mut(|state| state.problem_editor = Default::default());
                    navigator.push(&Route::Problem {
                        id: id.as_i64().unwrap(),
                    })
                } else {
                    error.set(Some(res["error"].as_str().unwrap().to_string()))
                }
            });
        })
    };

    // Simple callback function to stop showing the error dialog. Ideally this would be handled on
    // the side of the error dialog, but that didn't seem to be possible when I wrote this *shrug*
    let clear_error = {
        let error = error.clone();

        Callback::from(move |_| {
            error.set(None);
        })
    };

    html! {
        <div class="container">
            <Navbar />

            <div class="problem-editor-wrapper">
                if let Some(e) = &*error {
                    <Modal onclose = { clear_error }>
                        <ErrorBox>{ e }</ErrorBox>
                    </Modal>
                }

                <div class="problem-editor-sidebar">
                <input value={title} class="title-input" oninput={dispatch.reduce_mut_callback_with(|state, e: InputEvent| {
                    let title = e.target_unchecked_into::<HtmlInputElement>().value();
                    state.problem_editor.title = title;
                })} />
                    <MarkdownEditor />
                </div>

                <div class="problem-editor-content">
                    <Tabbed class="problem-editor-main" titles={ vec!["runner", "template", "tests"] }>
                        <div onfocusout={dispatch.reduce_mut_callback(move |state| {
                            state.problem_editor.runner = runner_code.get_value()
                        })}><CodeEditor options = { runner_editor_options } /></div>
                        <div onfocusout={dispatch.reduce_mut_callback(move |state| {
                            state.problem_editor.template = template_code.get_value()
                        })}><CodeEditor options = { template_editor_options } /></div>
                        <TestsEditor />
                    </Tabbed>

                    <div class="code-runner-wrapper">
                        <button class="button green" onclick={create_problem}>{ "Submit" }</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
