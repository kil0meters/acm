use acm::models::{
    forms::GenerateTestsForm,
    runner::RunnerError,
    test::{Test, TestResult},
};
use gloo_net::http::Request;
use monaco::api::TextModel;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::suspense::use_future;
use yewdux::prelude::*;

use crate::{
    components::{CodeEditor, LoadingButton, Modal},
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
        <div class="grid grid-cols-2 gap-2">
            <div class="flex flex-col gap-2">
                <span>{"Input"}</span>
                <textarea class="h-32 resize-none border-neutral-300 border rounded p-2 bg-neutral-50 outline-0 transition-shadow focus:ring ring-neutral-300" oninput={input_changed} value={props.test.input.clone()} />
            </div>

            <div class="flex flex-col gap-2">
                <span>{"Expected Output"}</span>
                <textarea class="h-32 resize-none border-neutral-300 border rounded p-2 bg-neutral-50 outline-0 transition-shadow focus:ring ring-neutral-300" oninput={expected_output_changed} value={props.test.expected_output.clone()} />
            </div>
        </div>

    }
}

#[function_component]
fn TestEditorList() -> Html {
    // We rerender only when a test is added or removed.
    let tests = use_selector(|state: &State| state.problem_editor.tests.clone());
    let loading = use_state(|| false);
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

    let loading_tmp = loading.clone();
    let populate_tests = dispatch.reduce_mut_future_callback(move |state| {
        let loading = loading_tmp.clone();
        Box::pin(async move {
            loading.set(true);

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

            loading.set(false);
        })
    });

    html! {
        <div class="flex flex-col gap-2 p-2 overflow-y-auto">
            {
                tests.iter().map(|test| {
                    html! {
                        <TestEditor test={test.clone()}/>
                    }
                }).collect::<Html>()
            }

            <div class="flex items-center gap-1 justify-center">
                <button class="px-4 py-2 rounded-l-full bg-blue-600 hover:bg-blue-500 text-blue-50 transition-colors text-sm w-36" onclick={add_test}>{ "Add test" }</button>
                <button class="px-4 py-2 bg-red-600 hover:bg-red-500 text-red-50 transition-colors text-sm w-36" onclick={remove_test}>{ "Remove test" }</button>
                <LoadingButton loading={*loading} class="px-4 py-2 rounded-r-full bg-neutral-600 hover:bg-neutral-500 text-neutral-50 transition-colors text-sm min-w-[9rem] justify-center whitespace-nowrap" onclick={populate_tests}>{ "Populate output" }</LoadingButton>
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
        <div class="grid grid-rows-2 xl:grid-rows-1 xl:grid-cols-3">
            <div class="col-span-2 border-b border-neutral-300 lg:border-b-0 lg:border-r" {onfocusout}>
                <CodeEditor classes="min-h-[40vh] lg:h-full" {options} />
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
            <button class="aspect-square bg-neutral-200 border border-neutral-400 b rounded transition-shadow hover:shadow-md hover:ring-2 ring-neutral-400"
                    onclick={show_modal}>
                { format!("Test #{}", props.test.index) }
            </button>

            <Modal shown={*modal_shown} onclose={hide_modal}>
                <div class="bg-white rounded-md border border-neutral-300 p-4 flex flex-col gap-2">
                    <h2 class="text-2xl">{ "Test #" } { props.test.index }</h2>

                    <label>{ "Input" }</label>

                    <pre class="p-2 bg-blue-50 rounded-md">
                        <code>{ &props.test.input }</code>
                    </pre>

                    <label>{ "Expected" }</label>

                    <pre class="p-2 bg-blue-50 rounded-md">
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
        <div class="flex flex-col gap-2">
            <div class="flex items-center gap-2">
                if props.failed {
                    <span class="text-red-600 text-2xl">{ "Failed" }</span>
                    <span class="text-red-600">{ props.result.time / 1000 } {"µs"}</span>
                } else {
                    <span class="text-red-600 text-2xl">{ "Passed" }</span>
                    <span class="text-green-600">{ props.result.time / 1000 } {"µs"}</span>
                }
            </div>

            <label>{ "Input" }</label>

            <pre class="p-2 bg-blue-50 rounded-md">
                <code>{ &props.result.input }</code>
            </pre>

            <label>{ "Expected" }</label>

            <pre class="p-2 bg-blue-50 rounded-md">
                <code>{ &props.result.expected_output }</code>
            </pre>

            <label>{ "Output" }</label>

            <pre class="p-2 bg-blue-50 rounded-md">
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

    let base_button_styles =
        "aspect-square border b rounded transition-shadow hover:shadow-md hover:ring-2";

    html! {
        <>
            <button class={
                classes!(base_button_styles, if props.failed { "bg-red-200 border-red-400 ring-red-400 text-red-900" } else { "bg-green-200 border-green-400 ring-green-400 text-green-900" } )
            }
            onclick={show_modal}>{ format!("Test #{}", props.result.index) }</button>

            <Modal shown={*modal_shown} onclose={hide_modal}>
                <div class="bg-white rounded-md border border-neutral-300 p-4">
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
                    <div class="flex-col flex p-4 bg-green-500 text-green-50">
                        <span class="font-bold text-2xl">{ "Congratulations!" }</span>
                        <span>{ "Your code passed all of the supplied tests." }</span>
                        <span>{ "Ran in " } { res.runtime } { " ms." }</span>
                    </div>
                }
            } else {
                html! {
                    <div class="grid grid-cols-3 lg:grid-cols-4 p-2 gap-2">
                        {
                            res.failed_tests.iter()
                            .map(|t| {
                                html! {
                                    <TestResultEntry failed={true} result={t.clone()} />
                                }
                            })
                            .collect::<Html>()
                        }

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
                }
            }
        }
        Some(Err(e)) => html! {
            html! {
                <div class="bg-red-500 text-red-50 p-2 flex flex-col gap-2">
                    <h1 class="text-2xl font-bold">{ "error." }</h1>

                    <pre class="bg-red-700 overflow-x-auto p-2 rounded">
                        <code>{ e }</code>
                    </pre>
                </div>
            }
        },
        None => match *tests {
            Ok(ref tests) => html! {
                <div class="grid grid-cols-3 lg:grid-cols-4 p-2 gap-2">
                {
                    tests
                    .into_iter()
                    .map(|t| {
                        html! {
                            <TestEntry test={t.clone()} />
                        }
                    })
                    .collect::<Html>()
                }
                </div>
            },
            Err(ref failure) => failure.to_string().into(),
        },
    };

    let collapse_styles = if *shown {
        "p-4 bg-neutral-200 hover:bg-neutral-100 cursor-pointer select-none transition-colors border-b border-neutral-300 rounded-t-md md:rounded-none"
    } else {
        "p-4 bg-neutral-200 hover:bg-neutral-100 cursor-pointer select-none transition-colors rounded-md md:rounded-none"
    };

    Ok(html! {
        <div class="flex flex-col border border-neutral-300 rounded-md mx-2 mb-2 md:m-0 md:border-0">
            <a class={collapse_styles} onclick={onclick}>
                if *shown {
                    {"Hide tests"}
                } else {
                    {"Show tests"}
                }
            </a>

            if *shown {
                <div class="max-h-96 overflow-y-auto">
                    { tests_html }
                </div>
            } else {}
        </div>
    })
}
