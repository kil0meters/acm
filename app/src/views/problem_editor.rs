use monaco::{
    api::{CodeEditorOptions, TextModel},
    yew::CodeEditor,
};

use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use acm::models::{forms::CreateProblemForm, Session};

use crate::{
    components::{ErrorBox, Modal, Navbar, Tabbed},
    Route,
};

#[derive(PartialEq, Properties)]
struct MarkdownEditorProps {
    model: TextModel,
}

#[function_component(MarkdownEditor)]
fn markdown_editor(props: &MarkdownEditorProps) -> Html {
    let preview = use_state(|| false);

    let options =
        Rc::new(CodeEditorOptions::default().with_model((props.model).clone())).to_sys_options();

    options.set_font_size(Some(18.0));

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

#[function_component(TestsEditor)]
fn tests_editor() -> Html {
    html! {
        <div>
        </div>
    }
}

#[function_component(ProblemEditorView)]
pub fn problem_editor_view() -> Html {
    let title = use_state(|| String::new());
    let description = use_state(|| TextModel::create("", Some("markdown"), None).unwrap());
    let runner_code = use_state(|| TextModel::create("", Some("cpp"), None).unwrap());
    let template_code = use_state(|| TextModel::create("", Some("cpp"), None).unwrap());

    let history = use_history().unwrap();
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

    let template_editor_options =
        Rc::new(CodeEditorOptions::default().with_model((*template_code).clone())).to_sys_options();
    template_editor_options.set_font_size(Some(18.0));

    let title_tmp = title.clone();
    let description_tmp = description.clone();
    let error_tmp = error.clone();
    let create_problem = Callback::from(move |_| {
        let title_text = (*title_tmp).clone();
        let description_text = description_tmp.get_value();
        let runner_text = runner_code.get_value();
        let template_text = template_code.get_value();

        if title_text.is_empty()
            || description_text.is_empty()
            || runner_text.is_empty()
            || template_text.is_empty()
        {
            error_tmp.set(Some("One or more required fields is empty.".to_string()));

            return;
        }

        let create_problem_data = CreateProblemForm {
            title: title_text,
            description: description_text,
            runner: runner_text,
            template: template_text,
        };

        let token_tmp = token.clone();
        let error_tmp = error_tmp.clone();
        let history_tmp = history.clone();
        spawn_local(async move {
            let client = reqwest::Client::new();
            let res: Value = client
                .post("http://127.0.0.1:8080/api/authorized/create-problem")
                .header(AUTHORIZATION, &format!("Bearer {}", token_tmp))
                .json(&create_problem_data)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            if let Some(id) = res.get("id") {
                history_tmp.push(Route::Problem {
                    id: id.as_i64().unwrap(),
                })
            } else {
                error_tmp.set(Some(res["str"].as_str().unwrap().to_string()))
            }
        });
    });

    let error_tmp = error.clone();
    html! {
        <div class="problem-editor-wrapper">
            <Navbar />

            if let Some(e) = &*error {
                <Modal onclose = {Callback::from(move |_| {
                    error_tmp.set(None);
                })}>
                    <ErrorBox>{ e }</ErrorBox>
                </Modal>
            }

            <div class="problem-editor-sidebar">
                <input class="title-input" oninput={Callback::from(move |e: InputEvent| {
                    let elm: HtmlInputElement = e.target_unchecked_into();
                    title.set(elm.value());
                })} />
                <MarkdownEditor model={ (*description).clone() } />
            </div>

            <div class="problem-editor-content">
                <Tabbed class="problem-editor-main" titles={ vec!["runner", "template", "tests"] }>
                    <div><CodeEditor options = { runner_editor_options } /></div>
                    <div><CodeEditor options = { template_editor_options } /></div>
                    <div></div>
                </Tabbed>

                <div class="code-runner-wrapper">
                    <button class="button green" onclick={create_problem}>{ "Submit" }</button>
                </div>
            </div>
        </div>
    }
}
