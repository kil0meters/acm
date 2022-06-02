use yew::prelude::*;
use yewdux::prelude::*;
use acm::models::{runner::RunnerError, test::TestResult, forms::CustomProblemInputForm};
use web_sys::HtmlTextAreaElement;
use gloo_net::http::Request;

use crate::{
    state::State,
    components::TestResultContents,
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

    let onclick = dispatch.reduce_mut_future_callback(move |state| {
        Box::pin(async move {
            let entry = state
                .problems
                .entry(id)
                .or_insert_with(|| Default::default());
            let input = entry.custom_input.clone();
            let implementation = entry.implementation.clone();

            let token = state.session.as_ref().unwrap().token.clone();

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

            let res: Result<TestResult, RunnerError> = res.json().await.unwrap();

            state
                .problems
                .entry(id)
                .and_modify(|e| e.custom_test_result = res.ok());
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

