//! An editor view showing a single problem.

use acm::models::{
    forms::RunTestsForm,
    runner::{RunnerError, RunnerResponse},
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
    components::{CodeEditor, Navbar, TestList, InputTester},
    helpers::{parse_markdown, themed_editor_with_model},
};

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
                        <TestList problem_id={id} />
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
