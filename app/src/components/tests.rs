use acm::models::test::Test;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::state::State;

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
                <textarea oninput={input_changed} value={props.test.input.clone()} />
            </div>

            <div class="test-editor-col">
                <span>{"Expected Output"}</span>
                <textarea oninput={expected_output_changed} value={props.test.expected_output.clone()} />
            </div>
        </div>

    }
}

// TODO: This callback jumping is awful. Look into yewdux to simplify this.
#[function_component]
pub fn TestsEditor() -> Html {
    // We rerender only when a test is added or removed.
    use_selector(|state: &State| state.problem_editor.tests.len());

    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();

    let add_test = dispatch.reduce_mut_callback(|state| {
        state.problem_editor.tests.push(Test {
            index: state.problem_editor.tests.len() as i64,
            ..Default::default()
        })
    });

    let remove_test = dispatch.reduce_mut_callback(|state| {
        state.problem_editor.tests.pop();
    });

    html! {
        <div class="tests-editor">
            {
                state.problem_editor.tests.iter().map(|test| {
                    html! {
                        <TestEditor test={test.clone()}/>
                    }
                }).collect::<Html>()
            }

            <div class="tests-buttons">
                <button class="blue button" onclick={add_test}>{ "Add test" }</button>
                <button class="red button" onclick={remove_test}>{ "Remove test" }</button>
            </div>
        </div>
    }
}
