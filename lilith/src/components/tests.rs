use acm::models::{
    forms::GenerateTestsForm,
    runner::RunnerError,
    test::{Test, TestResult},
};
use gloo_net::http::Request;
use monaco::api::TextModel;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::suspense::{use_future, use_future_with_deps, Suspense};
use yewdux::prelude::*;

use crate::{
    api_url,
    components::{CodeEditor, LoadingButton, Modal, SubmissionFeedback},
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
                <textarea class="h-32 resize-none border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300" oninput={input_changed} value={props.test.input.clone()} />
            </div>

            <div class="flex flex-col gap-2">
                <span>{"Expected Output"}</span>
                <textarea class="h-32 resize-none border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300" oninput={expected_output_changed} value={props.test.expected_output.clone()} />
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

            let res: Result<Vec<Test>, RunnerError> = Request::post(api_url!("/generate-tests"))
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

            <div class="grid grid-cols-3 gap-1 mx-auto max-w-sm w-full">
                <button class="py-2 rounded-l-full bg-blue-600 hover:bg-blue-500 text-blue-50 transition-colors text-sm whitespace-nowrap" onclick={add_test}>{ "Add" }</button>
                <button class="py-2 bg-red-600 hover:bg-red-500 text-red-50 transition-colors text-sm whitespace-nowrap" onclick={remove_test}>{ "Remove" }</button>
                <LoadingButton loading={*loading} class="px-4 py-2 rounded-r-full bg-neutral-600 hover:bg-neutral-500 text-neutral-50 transition-colors text-sm justify-center whitespace-nowrap" onclick={populate_tests}>{ "Populate" }</LoadingButton>
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
        <div class="grid grid-rows-2 grid-cols-1 xl:grid-rows-1 xl:grid-cols-3">
            <div class="xl:col-span-2 border-b border-neutral-300 dark:border-neutral-700 xl:border-b-0 xl:border-r" {onfocusout}>
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
            <button class="aspect-square bg-neutral-200 dark:bg-slate-700 border border-neutral-400 dark:border-slate-800 b rounded transition-shadow hover:shadow-md hover:ring-2 ring-neutral-400 dark:ring-slate-800"
                    onclick={show_modal}>
                { format!("Test #{}", props.test.index) }
            </button>

            <Modal shown={*modal_shown} onclose={hide_modal}>
                <div class="bg-white rounded-md border border-neutral-300 dark:bg-black dark:border-neutral-700 p-4 flex flex-col gap-2">
                    <h2 class="text-2xl">{ "Test #" } { props.test.index }</h2>

                    <label>{ "Input" }</label>

                    <pre class="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
                        <code>{ &props.test.input }</code>
                    </pre>

                    <label>{ "Expected" }</label>

                    <pre class="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
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
    pub class: Option<Classes>,
}

#[function_component]
pub fn TestResultContents(props: &TestResultProps) -> Html {
    html! {
        <div class={classes!(&props.class, "flex", "flex-col", "gap-2")}>
            <div class="flex items-center gap-2">
                if props.result.success {
                    <span class="text-green-600 text-2xl">{ "Passed" }</span>
                    <span class="text-green-600">{ props.result.runtime / 1000 } {"µs"}</span>
                } else {
                    <span class="text-red-600 text-2xl">{ "Failed" }</span>
                    <span class="text-red-600">{ props.result.runtime / 1000 } {"µs"}</span>
                }
            </div>

            <label>{ "Input" }</label>

            <pre class="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
                <code>{ &props.result.input }</code>
            </pre>

            <label>{ "Expected" }</label>

            <pre class="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
                <code>{ &props.result.expected_output }</code>
            </pre>

            <label>{ "Output" }</label>

            <pre class="p-2 bg-blue-50 rounded-md border-blue-200 dark:border-slate-700 dark:bg-slate-800 border overflow-auto">
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
                classes!(base_button_styles,
                    if props.result.success {
                        "bg-green-200 dark:bg-green-800 dark:border-green-600 dark:ring-green-600 dark:text-green-100 border-green-400 ring-green-400 text-green-900"
                    } else {
                        "bg-red-200 dark:bg-red-800 dark:border-red-600 dark:ring-red-600 dark:text-red-100 border-red-400 ring-red-400 text-red-900"
                    }
                )
            }
            onclick={show_modal}>{ format!("Test #{}", props.result.index) }</button>

            <Modal shown={*modal_shown} onclose={hide_modal}>
                <div class="bg-white dark:bg-black rounded-md border border-neutral-300 dark:border-neutral-700 p-4">
                    <TestResultContents result={props.result.clone()} />
                </div>
            </Modal>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct TestResultListProps {
    submission_id: i64,
}

#[function_component]
fn TestResultList(props: &TestResultListProps) -> HtmlResult {
    let submission_id = props.submission_id;

    let tests = use_future_with_deps(
        |submission_id| async move {
            Request::get(api_url!("/submissions/{}/tests", submission_id))
                .send()
                .await?
                .json::<Vec<TestResult>>()
                .await
        },
        submission_id,
    )?;

    match &*tests {
        Ok(tests) => Ok(html! {
            <div class="grid grid-cols-3 lg:grid-cols-4 p-2 gap-2">
                {
                    for tests.iter().map(|t| {
                        html! {
                            <TestResultEntry result={t.clone()} />
                        }
                    })
                }
            </div>
        }),
        Err(e) => Ok(html! { e.to_string() }),
    }
}

#[function_component]
fn TestList(props: &TestsProps) -> HtmlResult {
    let problem_id = props.problem_id;

    let tests = use_future(|| async move {
        Request::get(api_url!("/problems/{}/tests", problem_id))
            .send()
            .await?
            .json::<Vec<Test>>()
            .await
    })?;

    match &*tests {
        Ok(tests) => Ok(html! {
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
        }),
        Err(_) => Ok(html! {}),
    }
}

#[derive(PartialEq, Properties)]
pub struct TestsProps {
    pub problem_id: i64,
}

#[function_component]
pub fn SubmissionTestList(props: &TestsProps) -> HtmlResult {
    let problem_id = props.problem_id;

    let submission = use_selector(move |state: &State| {
        state
            .problems
            .get(&problem_id)
            .map(|x| x.submission.clone())
            .flatten()
    });

    let shown = use_state(|| false);
    let onclick = {
        let shown = shown.clone();
        Callback::from(move |_| {
            shown.set(!*shown);
        })
    };

    let collapse_styles = if *shown {
        "p-4 bg-neutral-200 dark:bg-neutral-800 dark:hover:bg-neutral-700 hover:bg-neutral-100 cursor-pointer select-none transition-colors border-b last:border-b-0 border-neutral-300 dark:border-neutral-700 rounded-t-md md:rounded-none"
    } else {
        "p-4 bg-neutral-200 dark:bg-neutral-800 dark:hover:bg-neutral-700 hover:bg-neutral-100 cursor-pointer select-none transition-colors rounded-md md:rounded-none"
    };

    Ok(html! {
        <div class="flex flex-col">
            <Suspense>
                <SubmissionResult {problem_id} />
            </Suspense>

            <div class="flex flex-col border border-neutral-300 dark:border-neutral-700 rounded-md mx-2 mb-2 md:m-0 md:border-0">
                <a class={collapse_styles} onclick={onclick}>
                    if *shown {
                        {"Hide tests"}
                    } else {
                        {"Show tests"}
                    }
                </a>

                if *shown {
                    <Suspense>
                        <div class="max-h-96 overflow-y-auto peer">
                            if let Some(submission) = &*submission {
                                <TestResultList submission_id={submission.id} />
                            } else {
                                <TestList {problem_id} />
                            }
                        </div>
                    </Suspense>
                }
            </div>
        </div>
    })
}

#[function_component]
fn SubmissionResult(props: &TestsProps) -> Html {
    let id = props.problem_id;

    let submission = use_selector(move |state: &State| {
        state
            .problems
            .get(&id)
            .map(|p| p.submission.clone())
            .flatten()
    });

    if let Some(submission) = &*submission {
        html! {
            <div class="border-b border-neutral-300 dark:border-neutral-700 mb-2 md:m-0">
                <SubmissionFeedback submission={submission.clone()} />
            </div>
        }
    } else {
        html! {}
    }
}
