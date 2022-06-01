use acm::models::{
    forms::{CreateProblemForm, GenerateTestsForm},
    runner::RunnerError,
    test::Test,
};
use gloo_net::http::Request;
use monaco::api::TextModel;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{components::CodeEditor, helpers::themed_editor_with_model, state::State};

#[derive(PartialEq, Properties)]
struct TestEditorProps {
    test: Test,
}

#[function_component]
fn TestEditor(props: &TestEditorProps) -> Html {
    let dispatch = Dispatch::<State>::new();
    let idx = props.test.index;

    let input_changed = dispatch.reduce_mut_callback_with(move |state, e: InputEvent| {
        let text = e.target_unchecked_into::<HtmlTextAreaElement>().value();
        state.problem_editor.tests[idx as usize].input = text;
    });

    let expected_output_changed = dispatch.reduce_mut_callback_with(move |state, e: InputEvent| {
        let text = e.target_unchecked_into::<HtmlTextAreaElement>().value();
        state.problem_editor.tests[idx as usize].expected_output = text;
    });

    html! {
        <div class="test-editor">
            <div class="test-editor-col">
                <span>{"Input"}</span>
                <textarea class="acm-input" oninput={input_changed} value={props.test.input.clone()} />
            </div>

            <div class="test-editor-col">
                <span>{"Expected Output"}</span>
                <textarea class="acm-input" oninput={expected_output_changed} value={props.test.expected_output.clone()} />
            </div>
        </div>

    }
}

#[function_component]
fn TestEditorList() -> Html {
    // We rerender only when a test is added or removed.
    let tests = use_selector(|state: &State| state.problem_editor.tests.clone());
    let dispatch = Dispatch::<State>::new();

    let add_test = dispatch.reduce_mut_callback(|state| {
        state.problem_editor.tests.push(Test {
            index: state.problem_editor.tests.len() as i64,
            ..Default::default()
        })
    });

    let remove_test = dispatch.reduce_mut_callback(|state| {
        state.problem_editor.tests.pop();
    });

    let populate_tests = dispatch.reduce_mut_future_callback(|state| {
        Box::pin(async move {
            let res: Result<Vec<Test>, RunnerError> = Request::post("/api/generate-tests")
                .header(
                    "Authorization",
                    &format!("Bearer {}", state.session.as_ref().unwrap().token),
                )
                .json(&GenerateTestsForm {
                    runner: state.problem_editor.runner.clone(),
                    reference: state.problem_editor.reference.clone(),
                    username: state.session.as_ref().unwrap().user.username.clone(),
                    inputs: state
                        .problem_editor
                        .tests
                        .iter()
                        .map(|test| test.input.clone())
                        .collect::<Vec<String>>(),
                })
                .unwrap()
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            match res {
                Ok(tests) => state.problem_editor.tests = tests,
                Err(e) => state.error = Some(e.to_string()),
            };
        })
    });

    html! {
        <div class="tests-editor-list">
            {
                tests.iter().map(|test| {
                    html! {
                        <TestEditor test={test.clone()}/>
                    }
                }).collect::<Html>()
            }

            <div class="tests-buttons">
                <button class="blue button" onclick={add_test}>{ "Add test" }</button>
                <button class="red button" onclick={remove_test}>{ "Remove test" }</button>
                <button class="grey button" onclick={populate_tests}>{ "Populate output" }</button>
            </div>
        </div>
    }
}

#[function_component]
pub fn TestsEditor() -> Html {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();

    let reference = TextModel::create(&state.problem_editor.reference, Some("cpp"), None).unwrap();
    let options = themed_editor_with_model(reference.clone());

    let onfocusout = dispatch.reduce_mut_callback(move |state| {
        state.problem_editor.reference = reference.get_value();
    });

    html! {
        <div class="tests-editor">
            <div {onfocusout}>
                <CodeEditor {options} />
            </div>
            <TestEditorList />
        </div>
    }
}
