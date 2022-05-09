use monaco::{
    api::{CodeEditorOptions, TextModel},
    yew::CodeEditor,
};
use std::rc::Rc;
use web_sys::Node;
use yew::prelude::*;

use crate::components::{Navbar, Tabbed};

#[derive(PartialEq, Properties)]
struct MarkdownEditorProps {
    model: TextModel,
}

#[function_component(MarkdownEditor)]
fn markdown_editor(props: &MarkdownEditorProps) -> Html {
    let preview = use_state(|| false);

    let options = Rc::new(
        CodeEditorOptions::default()
            .with_model((props.model).clone())
    )
    .to_sys_options();

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
    let description = use_state(|| TextModel::create("", Some("markdown"), None).unwrap());
    let runner_code = use_state(|| TextModel::create("", Some("cpp"), None).unwrap());
    let template_code = use_state(|| TextModel::create("", Some("cpp"), None).unwrap());

    let runner_editor_options = Rc::new(
        CodeEditorOptions::default()
            .with_model((*runner_code).clone())
    )
    .to_sys_options();
    runner_editor_options.set_font_size(Some(18.0));

    let template_editor_options = Rc::new(
        CodeEditorOptions::default()
            .with_model((*template_code).clone())
    )
    .to_sys_options();
    template_editor_options.set_font_size(Some(18.0));

    html! {
        <div class="problem-editor-wrapper">
            <Navbar />

            <MarkdownEditor model={ (*description).clone() } />

            <Tabbed class="problem-editor-content" titles={ vec!["runner", "template", "tests"] }>
                <div><CodeEditor options = { runner_editor_options } /></div>
                <div><CodeEditor options = { template_editor_options } /></div>
                <div></div>
            </Tabbed>
        </div>
    }
}
