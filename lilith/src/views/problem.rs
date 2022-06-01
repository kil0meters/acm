//! An editor view showing a single problem.

use acm::models::{
    forms::{RunTestsForm, CustomProblemInputForm},
    runner::{RunnerError, RunnerResponse},
    test::{Test, TestResult},
    Problem,
};
use gloo_net::http::Request;
use monaco::api::TextModel;
use thiserror::Error;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use yewdux::prelude::*;

use crate::state::State;
use crate::{
    components::{CodeEditor, ErrorBox, Modal, Navbar},
    helpers::{parse_markdown, themed_editor_with_model},
};

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
struct TestResultProps {
    result: TestResult,
    failed: bool,
}

#[function_component]
fn TestResultContents(props: &TestResultProps) -> Html {
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
struct TestsProps {
    problem_id: i64,
}

#[function_component]
fn Tests(props: &TestsProps) -> HtmlResult {
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
                        <span class="submission-passed">{ "Congradulations!" }</span>
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

#[derive(Properties, PartialEq)]
struct DescriptionProps {
    title: String,
    content: String,
}

#[function_component]
fn Description(props: &DescriptionProps) -> Html {
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    div.set_inner_html(&parse_markdown(&props.content));

    html! {
        <div class="description padded card">
            <h1>{ props.title.clone() }</h1>

            { Html::VRef(div.into()) }
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct ProblemEditorProps {
    id: i64,

    #[prop_or_default]
    template: String,
}

#[function_component]
fn ProblemEditor(props: &ProblemEditorProps) -> Html {
    let id = props.id;
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();

    let code = {
        if let Some(problem) = state.problems.get(&id) {
            &problem.implementation
        } else {
            &props.template
        }
    };

    let code = TextModel::create(code, Some("cpp"), None).unwrap();
    let options = themed_editor_with_model(code.clone());

    let onfocusout = dispatch.reduce_mut_callback(move |state| {
        let entry = state
            .problems
            .entry(id)
            .or_insert_with(|| Default::default());
        entry.implementation = code.get_value();
    });

    html! {
        <div class="card" {onfocusout}>
            <CodeEditor options = {options}/>
        </div>
    }
}

#[derive(Error, Debug)]
enum ProblemSubmissionError {
    #[error("You must be logged in to do that")]
    NotLoggedIn,

    #[error("You have not added any code to the problem")]
    NoProblemCode,
}

async fn run_tests(
    token: &str,
    form: &RunTestsForm,
) -> Option<Result<RunnerResponse, RunnerError>> {
    Some(
        Request::post("/api/run-tests")
            .header("Authorization", &format!("Bearer {}", token))
            .json(&form)
            .ok()?
            .send()
            .await
            .ok()?
            .json()
            .await
            .ok()?,
    )
}

fn submit_code(id: i64) -> Result<(), ProblemSubmissionError> {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();

    let token = state
        .session
        .as_ref()
        .ok_or(ProblemSubmissionError::NotLoggedIn)?
        .token
        .to_string();

    let problem_code = state
        .problems
        .get(&id)
        .ok_or(ProblemSubmissionError::NoProblemCode)?
        .implementation
        .to_string();

    let form = RunTestsForm {
        problem_id: id,
        test_code: problem_code,
    };

    spawn_local(async move {
        match run_tests(&token, &form).await {
            Some(res) => dispatch.reduce_mut(|state| {
                state.test_results.insert(id, res);
                state.tests_shown = true;
            }),
            None => dispatch.reduce_mut(|state| {
                state.error = Some("Encountered an error while submitting code".to_string());
            }),
        }
    });

    Ok(())
}

#[derive(Properties, PartialEq)]
struct CustomTestResultProps {
    id: i64,
}

#[function_component]
fn CustomTestResult(props: &CustomTestResultProps) -> Html {
    let id = props.id;

    let result = use_selector(move |state: &State| {
        state
            .problems
            .get(&id)
            .map(|x| x.custom_test_result.clone())
            .flatten()
    });

    html! {
        if let Some(result) = &*result {
            <TestResultContents failed={result.expected_output != result.output} result={result.clone()} />
        }
    }
}

#[derive(Properties, PartialEq)]
struct InputTesterProps {
    id: i64,
}

#[function_component]
fn InputTester(props: &InputTesterProps) -> Html {
    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();
    let id = props.id;

    let oninput = dispatch.reduce_mut_callback_with(move |state, e: InputEvent| {
        let text = e.target_unchecked_into::<HtmlTextAreaElement>().value();
        let entry = state
            .problems
            .entry(id)
            .or_insert_with(|| Default::default());
        entry.custom_input = text;
    });

    let onclick = dispatch.reduce_mut_future_callback(move |state| {
        Box::pin(async move {
            let entry = state.problems.entry(id).or_insert_with(|| Default::default());
            let input = entry.custom_input.clone();
            let implementation = entry.implementation.clone();

            let token = state
                .session
                .as_ref()
                .unwrap()
                .token
                .clone();

            let res = Request::post("/api/custom-input")
                .header("Authorization", &format!("Bearer {}", token))
                .json(&CustomProblemInputForm {
                    problem_id: id,
                    input,
                    implementation,
                })
                .unwrap()
                .send()
                .await
                .unwrap();

            let res: Result<TestResult, RunnerError> = res
                .json()
                .await
                .unwrap();

            state.problems.entry(id).and_modify(|e| e.custom_test_result = res.ok());
        })
    });

    let value = state
        .problems
        .get(&id)
        .map(|p| p.custom_input.clone())
        .unwrap_or_default();

    html! {
        <div class="padded card problem-console">
            <div class="custom-input">
                <label>{ "Input" }</label>
                <textarea class="acm-input resize-none" {oninput} {value}>
                </textarea>
                <button class="blue button run-button" {onclick}>{ "Run" }</button>
            </div>

            <CustomTestResult {id} />
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ProblemViewProps {
    pub id: i64,
}

#[function_component]
pub fn ProblemViewInner(props: &ProblemViewProps) -> HtmlResult {
    let id = props.id;
    let problem = use_future(|| async move {
        Request::get(&format!("/api/problems/{}", id))
            .send()
            .await?
            .json::<Problem>()
            .await
    })?;

    let dispatch = Dispatch::<State>::new();
    let docker_shown = use_selector(move |state: &State| {
        if let Some(entry) = state.problems.get(&id) {
            entry.docker_shown
        } else {
            false
        }
    });

    let submit = dispatch.reduce_mut_callback(move |state| match submit_code(id) {
        Err(e) => state.error = Some(e.to_string()),
        _ => {}
    });

    let toggle_console = dispatch.reduce_mut_callback(move |state| {
        let entry = state
            .problems
            .entry(id)
            .or_insert_with(|| Default::default());
        entry.docker_shown = !entry.docker_shown;
    });

    match &*problem {
        Ok(problem) => Ok(html! {
            <div class="problem-wrapper">
                <div class="sidebar-wrapper">
                    <Suspense>
                        <Tests problem_id={id} />
                    </Suspense>
                    <Description title={ problem.title.clone() } content={ problem.description.clone() } />
                </div>
                <div class={classes!("content-wrapper", if *docker_shown { "shown" } else {""})}>
                    <ProblemEditor {id} template={ problem.template.clone() } />

                    if *docker_shown {
                        <InputTester {id} />
                    }

                    <div class="code-runner-wrapper">
                        <div class="activity-selector">
                            <button class="button grey" onclick={toggle_console}>
                                if *docker_shown {
                                    { "Hide console" }
                                } else {
                                    { "Show console" }
                                }
                            </button>
                        </div>

                        <button class="button green" onclick={submit}>{ "Submit" }</button>
                    </div>
                </div>
            </div>
        }),
        Err(e) => Ok(html! { e }),
    }
}

#[function_component]
pub fn ProblemView(props: &ProblemViewProps) -> Html {
    html! {
        <div class="container">
            <Navbar />
            <Suspense>
                <ProblemViewInner id={props.id} />
            </Suspense>
        </div>
    }
}
