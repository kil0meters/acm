//! A view used by officers to create/edit problems.

use monaco::api::{CodeEditorOptions, TextModel};

use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use acm::models::{forms::CreateProblemForm, Session};

use crate::{
    components::{CodeEditor, ErrorBox, Modal, Navbar, Tabbed, TestsEditor},
    Route,
};

#[derive(PartialEq, Properties)]
struct MarkdownEditorProps {
    model: TextModel,
}

#[function_component]
fn MarkdownEditor(props: &MarkdownEditorProps) -> Html {
    let preview = use_state(|| false);

    let options =
        Rc::new(CodeEditorOptions::default().with_model((props.model).clone())).to_sys_options();

    options.set_font_size(Some(18.0));
    options.set_automatic_layout(Some(true));

    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    div.set_inner_html(&markdown::to_html(&props.model.get_value()));

    let preview_tmp = preview.clone();
    html! {
        <div class="markdown-editor">
            <div class="top-row">
                <span>{ "problem description" }</span>

                <button class="button grey" onclick={ Callback::from(move |_| preview_tmp.set(!*preview_tmp)) }>
                    { if *preview { "hide preview" } else { "show preview" } }
                </button>
            </div>

            <div class="editor-content">
                if *preview {
                    { Html::VRef(div.into()) }
                } else {
                    <CodeEditor options = {options} />
                }
            </div>
        </div>
    }
}

#[function_component]
pub fn ProblemEditorView() -> Html {
    let title = use_mut_ref(|| String::new());
    let description = use_state(|| TextModel::create("", Some("markdown"), None).unwrap());
    let runner_code = use_state(|| TextModel::create("", Some("cpp"), None).unwrap());
    let template_code = use_state(|| TextModel::create("", Some("cpp"), None).unwrap());

    let navigator = use_navigator().unwrap();
    let ctx = use_context::<UseStateHandle<Option<Session>>>().unwrap();

    let token = if let Some(session) = (*ctx).as_ref() {
        session.token.clone()
    } else {
        String::new()
    };

    let error: UseStateHandle<Option<String>> = use_state(|| None);

    let runner_editor_options =
        Rc::new(CodeEditorOptions::default().with_model((*runner_code).clone())).to_sys_options();

    runner_editor_options.set_font_size(Some(18.0));
    runner_editor_options.set_automatic_layout(Some(true));

    let template_editor_options =
        Rc::new(CodeEditorOptions::default().with_model((*template_code).clone())).to_sys_options();
    template_editor_options.set_font_size(Some(18.0));
    template_editor_options.set_automatic_layout(Some(true));

    // This basically just takes all of the entered data by the user and submits that to the
    // server, thereby creating a problem or an error.
    let create_problem = {
        let title = title.clone();
        let description = description.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let title_text = (*title).clone().into_inner();
            let description_text = description.get_value();
            let runner_text = runner_code.get_value();
            let template_text = template_code.get_value();

            if title_text.is_empty()
                || description_text.is_empty()
                || runner_text.is_empty()
                || template_text.is_empty()
            {
                error.set(Some("One or more required fields is empty.".to_string()));

                return;
            }

            let create_problem_data = CreateProblemForm {
                title: title_text,
                description: description_text,
                runner: runner_text,
                template: template_text,

                tests: vec![],
            };

            let token = token.clone();
            let error = error.clone();
            let navigator = navigator.clone();
            spawn_local(async move {
                let client = reqwest::Client::new();
                let res: Value = client
                    .post("http://127.0.0.1:8080/api/create-problem")
                    .header(AUTHORIZATION, &format!("Bearer {}", token))
                    .json(&create_problem_data)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                if let Some(id) = res.get("id") {
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
        <div class="problem-editor-wrapper">
            <Navbar />

            if let Some(e) = &*error {
                <Modal onclose = { clear_error }>
                    <ErrorBox>{ e }</ErrorBox>
                </Modal>
            }

            <div class="problem-editor-sidebar">
                <input class="title-input" oninput={Callback::from(move |e: InputEvent| {
                    let elm: HtmlInputElement = e.target_unchecked_into();
                    *title.borrow_mut() = elm.value();
                })} />
                <MarkdownEditor model={ (*description).clone() } />
            </div>

            <div class="problem-editor-content">
                <Tabbed class="problem-editor-main" titles={ vec!["runner", "template", "tests"] }>
                    <div><CodeEditor options = { runner_editor_options } /></div>
                    <div><CodeEditor options = { template_editor_options } /></div>
                    <div><TestsEditor /></div>
                </Tabbed>

                <div class="code-runner-wrapper">
                    <button class="button green" onclick={create_problem}>{ "Submit" }</button>
                </div>
            </div>
        </div>
    }
}
