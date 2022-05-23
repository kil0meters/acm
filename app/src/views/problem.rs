//! An editor view showing a single problem.

use acm::models::{
    forms::RunTestsForm,
    test::{Test, TestResult},
    Problem,
};
use gloo_net::http::Request;
use monaco::api::{CodeEditorOptions, TextModel};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use yewdux::prelude::*;

use std::rc::Rc;

use crate::components::{CodeEditor, ErrorBox, Modal, Navbar};
use crate::state::State;

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
                    <h1>{ "Test #" } { props.test.index }</h1>

                    <pre>
                        <code>{ &props.test.input }</code>
                    </pre>

                    <pre>
                        <code>{ &props.test.expected_output }</code>
                    </pre>
                </div>
            </Modal>
        </>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct TestResultEntryProps {
    result: TestResult,
    failed: bool,
}

#[function_component]
fn TestResultEntry(props: &TestResultEntryProps) -> Html {
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
                    <h1>{ "Test #" } { props.result.index }</h1>

                    <pre>
                        <code>{ &props.result.input }</code>
                    </pre>

                    <pre>
                        <code>{ &props.result.expected_output }</code>
                    </pre>

                    <pre>
                        <code>{ &props.result.output }</code>
                    </pre>
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

    let shown = use_state(|| false);
    let test_results =
        use_selector(move |state: &State| state.test_results.get(&problem_id).map(|x| x.clone()));

    let onclick = {
        let shown = shown.clone();
        Callback::from(move |_| shown.set(!*shown))
    };

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
    // 1. If the most recent submission contains a compilation or runtime error, we display that at
    //    the top of the message
    //
    // 2. If the user has not yet run code, we simply show all of the tested in a greyed out state.
    let tests_html = match &*test_results {
        Some(Ok(res)) => {
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

    div.set_inner_html(&markdown::to_html(&props.content));

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
        if let Some(code) = state.problem_code.get(&id) {
            code
        } else {
            &props.template
        }
    };

    let code = TextModel::create(code, Some("cpp"), None).unwrap();
    let options = Rc::new(CodeEditorOptions::default().with_model(code.clone())).to_sys_options();

    options.set_font_size(Some(18.0));
    options.set_automatic_layout(Some(true));

    let onfocusout = dispatch.reduce_mut_callback(move |state| {
        state.problem_code.insert(id, code.get_value());
    });

    html! {
        <div class="card" {onfocusout}>
            <CodeEditor options = {options}/>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ProblemViewProps {
    pub id: i64,
}

#[function_component]
fn ProblemViewInner(props: &ProblemViewProps) -> HtmlResult {
    let dispatch = Dispatch::<State>::new();

    let id = props.id;
    let problem = use_future(|| async move {
        Request::get(&format!("/api/problems/{}", id))
            .send()
            .await?
            .json::<Problem>()
            .await
    })?;

    let submit = {
        Callback::from(move |_| {
            let state = dispatch.get();

            let form = RunTestsForm {
                problem_id: id,
                test_code: state.problem_code[&id].clone(),
            };

            let dispatch = dispatch.clone();
            spawn_local(async move {
                let res = Request::post("/api/run-tests")
                    .json(&form)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                dispatch.reduce_mut(|state| state.test_results.insert(id, res));
            });
        })
    };

    match &*problem {
        Ok(problem) => Ok(html! {
            <div class="problem-wrapper">
                <div class="sidebar-wrapper">
                    <Suspense>
                        <Tests problem_id={id} />
                    </Suspense>
                    <Description title={ problem.title.clone() } content={ problem.description.clone() } />
                </div>
                <div class="content-wrapper">
                    <ProblemEditor {id} template={ problem.template.clone() } />

                    <div class="code-runner-wrapper">
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
