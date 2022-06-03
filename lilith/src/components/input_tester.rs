use acm::models::{forms::CustomProblemInputForm, runner::RunnerError, test::TestResult};
use gloo_net::http::Request;
use std::rc::Rc;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::{
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

    html! {
        if let Some(result) = &*result {
            <TestResultContents failed={result.expected_output != result.output} result={result.clone()} />
        }
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
        <div class="padded card problem-console">
            <div class="custom-input">
                <label>{ "Input" }</label>
                <textarea class="acm-input resize-none" {oninput} {value}>
                </textarea>
                <CustomInputButton {id} />
            </div>

            <CustomTestResult {id} />
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

                let res = match Request::post("/api/custom-input")
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

                match res {
                    Ok(res) => {
                        state
                            .problems
                            .entry(id)
                            .and_modify(|e| e.custom_test_result = Some(res));
                    }
                    Err(e) => {
                        state.test_results.entry(id).and_modify(|t| *t = Err(e));

                        state.tests_shown = true;
                    }
                }

                loading.set(false);
            })
        })
    };

    html! {
        <LoadingButton
            class="blue button run-button"
            loading={*loading}
            {onclick}
        >
            { "Run" }
        </LoadingButton>
    }
}
