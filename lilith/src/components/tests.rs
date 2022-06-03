use acm::models::{
    forms::{CreateProblemForm, GenerateTestsForm},
    runner::RunnerError,
    test::{Test, TestResult},
};
use gloo_net::http::Request;
use monaco::api::TextModel;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use yewdux::prelude::*;

use crate::{
    components::{CodeEditor, ErrorBox, Modal},
    helpers::themed_editor_with_model,
    state::State,
};

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

#[derive(Clone, Debug, PartialEq, Properties)]
struct TestEntryProps {
    test: Test,
}

#[function_component]
fn TestEntry(props: &TestEntryProps) -> Html {
    let modal_shown = use_state(|| false);

    let show_modal = {
        let modal_shown = modal_shown.clone();

        Callback::from(move |_| {
            modal_shown.set(true);
        })
    };

    let hide_modal = {
        let modal_shown = modal_shown.clone();

        Callback::from(move |_| {
            modal_shown.set(false);
        })
    };

    html! {
        <>
            <button class="button grey" onclick={show_modal}>{ format!("Test #{}", props.test.index) }</button>

            <Modal shown={*modal_shown} onclose={hide_modal}>
                <div class="padded card">
                    <h2>{ "Test #" } { props.test.index }</h2>

                    <label>{ "Input" }</label>

                    <pre>
                        <code>{ &props.test.input }</code>
                    </pre>

                    <label>{ "Expected" }</label>

                    <pre>
                        <code>{ &props.test.expected_output }</code>
                    </pre>
                </div>
            </Modal>
        </>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TestResultProps {
    pub result: TestResult,
    pub failed: bool,
}

#[function_component]
pub fn TestResultContents(props: &TestResultProps) -> Html {
    html! {
        <div>
            <div class="submission-title">
                if props.failed {
                    <span class="failed res-title">{ "Failed" }</span> <span class="failed">{ props.result.time / 1000 } {"µs"}</span>
                } else {
                    <span class="passed res-title">{ "Passed" }</span> <span class="passed">{ props.result.time / 1000 } {"µs"}</span>
                }
            </div>

            <label>{ "Input" }</label>

            <pre>
                <code>{ &props.result.input }</code>
            </pre>

            <label>{ "Expected" }</label>

            <pre>
                <code>{ &props.result.expected_output }</code>
            </pre>

            <label>{ "Output" }</label>

            <pre>
                <code>{ &props.result.output }</code>
            </pre>
        </div>
    }
}

#[function_component]
fn TestResultEntry(props: &TestResultProps) -> Html {
    let modal_shown = use_state(|| false);

    let show_modal = {
        let modal_shown = modal_shown.clone();

        Callback::from(move |_| {
            modal_shown.set(true);
        })
    };

    let hide_modal = {
        let modal_shown = modal_shown.clone();

        Callback::from(move |_| {
            modal_shown.set(false);
        })
    };

    html! {
        <>
            <button class={
                classes!("button", if props.failed { "red" } else { "green" } )
            }
            onclick={show_modal}>{ format!("Test #{}", props.result.index) }</button>

            <Modal shown={*modal_shown} onclose={hide_modal}>
                <div class="padded card">
                    <TestResultContents result={props.result.clone()} failed={props.failed} />
                </div>
            </Modal>
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct TestsProps {
    pub problem_id: i64,
}

#[function_component]
pub fn TestList(props: &TestsProps) -> HtmlResult {
    let problem_id = props.problem_id;
    let dispatch = Dispatch::<State>::new();

    let test_results =
        use_selector(move |state: &State| state.test_results.get(&problem_id).map(|x| x.clone()));
    let shown = use_selector(move |state: &State| state.tests_shown);

    let onclick = dispatch.reduce_mut_callback(|state| state.tests_shown = !state.tests_shown);

    let tests = use_future(|| async move {
        Request::get(&format!("/api/problems/{}/tests", problem_id))
            .send()
            .await?
            .json::<Vec<Test>>()
            .await
    })?;

    // Render the contents of the test widget conditionally based on the current state.
    //
    // 1. If the most recent submission ran fine, we display the test results
    //
    //     a. If the code worked without error
    //
    // 2. If the most recent submission contains a compilation or runtime error, we display that at
    //    the top of the message
    //
    // 3. If the user has not yet run code, we simply show all of the tested in a greyed out state.
    let tests_html = match &*test_results {
        Some(Ok(res)) => {
            if res.failed_tests.is_empty() {
                html! {
                    <>
                        <span class="passed res-title">{ "Congratulations!" }</span>
                        <span>{ "Your code passed all of the supplied tests." }</span>
                        <span>{ "Ran in " } { res.runtime } { " ms." }</span>
                    </>
                }
            } else {
                let failed_tests = html! {
                    <div class="failed-tests">
                        <h3>{"Failed"}</h3>

                        <div class="test-list">
                        {
                            res.failed_tests.iter()
                            .map(|t| {
                                html! {
                                    <TestResultEntry failed={true} result={t.clone()} />
                                }
                            })
                            .collect::<Html>()
                        }
                        </div>
                    </div>
                };

                let passed_tests = html! {
                    <div class="passed-tests">
                        <h3>{"Passed"}</h3>

                        <div class="test-list">
                        {
                            res.passed_tests.iter()
                            .map(|t| {
                                html! {
                                    <TestResultEntry failed={false} result={t.clone()} />
                                }
                            })
                            .collect::<Html>()
                        }
                        </div>
                    </div>
                };

                html! {<> {failed_tests} {passed_tests} </>}
            }
        }
        Some(Err(e)) => html! {
            html! {
                <ErrorBox>{ e }</ErrorBox>
            }
        },
        None => match *tests {
            Ok(ref tests) => tests
                .into_iter()
                .map(|t| {
                    html! {
                        <TestEntry test={t.clone()} />
                    }
                })
                .collect::<Html>(),
            Err(ref failure) => failure.to_string().into(),
        },
    };

    Ok(html! {
        <div class="tests-wrapper card">
            <a class="hide-tests" onclick={onclick}>
                {
                    if *shown { "Hide tests" }
                    else { "Show tests" }
                }
            </a>

            if *shown {
                <div class="padded tests">
                    { tests_html }
                </div>
            } else {}
        </div>
    })
}
