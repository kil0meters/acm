use acm::models::{forms::CustomProblemInputForm, runner::RunnerError, test::TestResult};
use gloo_net::http::Request;

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
    api_url,
    components::{LoadingButton, TestResultContents},
    state::State,
};

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

    if let Some(result) = &*result {
        match result {
            Ok(result) => {
                html! { <TestResultContents class={classes!("m-4", "lg:ml-0")} result={result.clone()} /> }
            }
            Err(error) => html! {
                <div class="m-4 lg:ml-0 bg-red-500 text-red-50 p-4 flex flex-col gap-2 rounded-md border-red-600 dark:border-red-500 dark:bg-red-700 border">
                    <h1 class="text-2xl font-bold">{ "error." }</h1>

                    <pre class="bg-red-700 dark:bg-red-800 overflow-auto p-2 rounded">
                        <code>{ error }</code>
                    </pre>
                </div>
            },
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
pub struct InputTesterProps {
    pub id: i64,
}

#[function_component]
pub fn InputTester(props: &InputTesterProps) -> Html {
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

    let value = state
        .problems
        .get(&id)
        .map(|p| p.custom_input.clone())
        .unwrap_or_default();

    html! {
        <div class="border-t border-neutral-300 dark:border-neutral-700 bg-white dark:bg-black flex flex-col lg:flex-row min-h-0">
            <div class="flex flex-col gap-2 lg:w-96 p-4">
                <label>{ "Input" }</label>
                <textarea class="rounded border border-neutral-300 dark:border-neutral-700 bg-neutral-100 dark:bg-neutral-900 outline-0 transition-shadow focus:ring-2 ring-neutral-300 dark:ring-neutral-700 resize-none p-2 lg:flex-auto" {oninput} {value}>
                </textarea>
                <CustomInputButton {id} />
            </div>

            <div class="lg:w-96 lg:h-80 overflow-y-auto">
                <CustomTestResult {id} />
            </div>
        </div>
    }
}

#[function_component]
fn CustomInputButton(props: &InputTesterProps) -> Html {
    let loading = use_state(|| false);
    let dispatch = Dispatch::<State>::new();
    let id = props.id;

    let onclick = {
        let loading = loading.clone();
        dispatch.reduce_mut_future_callback(move |state| {
            let loading = loading.clone();
            Box::pin(async move {
                loading.set(true);

                let entry = state
                    .problems
                    .entry(id)
                    .or_insert_with(|| Default::default());
                let input = entry.custom_input.clone();
                let implementation = entry.implementation.clone();

                let token = match state.session.as_ref() {
                    Some(session) => session.token.clone(),
                    None => {
                        state.error = Some("You must be logged in to do that.".to_string());
                        loading.set(false);
                        return;
                    }
                };

                let res = match Request::post(api_url!("/custom-input"))
                    .header("Authorization", &format!("Bearer {}", token))
                    .json(&CustomProblemInputForm {
                        problem_id: id,
                        input,
                        implementation,
                    })
                    .expect("Failed to serialize json")
                    .send()
                    .await
                {
                    Ok(res) => res,
                    Err(_) => {
                        state.error = Some("Could not connect to server".to_string());
                        loading.set(false);
                        return;
                    }
                };

                let res: Result<TestResult, RunnerError> =
                    res.json().await.expect("Request is in an invalid format");

                state
                    .problems
                    .entry(id)
                    .and_modify(|e| e.custom_test_result = Some(res));

                loading.set(false);
            })
        })
    };

    html! {
        <LoadingButton
            class="px-4 py-2 rounded-full bg-blue-700 hover:bg-blue-500 transition-colors text-sm text-blue-100 mr-auto"
            loading={*loading}
            {onclick}
        >
            { "Run" }
        </LoadingButton>
    }
}
