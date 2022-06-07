//! An editor view showing a single problem.

use acm::models::Submission;
use acm::models::{forms::SubmitProblemForm, Problem};
use gloo_net::http::Request;
use monaco::api::TextModel;

use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use yewdux::prelude::*;

use crate::api_url;
use crate::state::State;
use crate::{
    components::{CodeEditor, InputTester, LoadingButton, Navbar, SubmissionTestList, Tabbed},
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
    div.set_class_name("prose prose-neutral");

    html! {
        <div class="grow bg-white p-4 h-full max-h-full overflow-y-auto">
            <h1 class="text-3xl font-bold">{ props.title.clone() }</h1>

            { Html::VRef(div.into()) }
        </div>
    }
}

fn make_submission(problem_id: i64, submission: &Submission) -> Html {
    let dispatch = Dispatch::<State>::new();

    let implementation = submission.code.clone();
    let btn = html! {
        <button class="px-4 py-2 text-sm bg-blue-600 hover:bg-blue-500 rounded-full font-bold text-blue-50 transition-colors"
            onclick={dispatch.reduce_mut_callback(move |state| {
                let implementation = implementation.clone();
                state.problems.entry(problem_id).and_modify(move |problem| {
                    problem.model.as_ref().expect("Expected a model").set_value(&implementation);
                    problem.implementation = implementation;
                });
            })}>
            { "Load" }
        </button>
    };

    let time_str = submission.time.format("%B %-d, %Y @ %-I:%M %p").to_string();

    if let Some(_) = &submission.error {
        html! {
            <div class="flex gap-2 items-center bg-red-100 p-4 border-neutral-300 border-b">
                <span class="text-red-600 font-bold text-lg">{ "Error" }</span>
                <span class="ml-auto text-red-600 text-sm">{ time_str }</span>
                { btn }
            </div>
        }
    } else if submission.success {
        html! {
            <div class="flex gap-2 items-center bg-green-100 p-4 border-neutral-300 border-b">
                <span class="text-green-600 font-bold text-lg">{ "Passed" }</span>
                <span class="ml-auto text-green-600 text-sm">{ time_str }</span>
                { btn }
            </div>
        }
    } else {
        html! {
            <div class="flex gap-2 items-center bg-neutral-50 p-4 border-neutral-300 border-b">
                <span class="text-red-600 font-bold text-lg">{ "Failed" }</span>
                <span class="ml-auto text-neutral-400 text-sm">{ time_str }</span>
                { btn }
            </div>
        }
    }
}

#[derive(Properties, PartialEq)]
struct SubmissionHistoryProps {
    id: i64,
}

#[function_component]
fn SubmissionHistory(props: &SubmissionHistoryProps) -> HtmlResult {
    let id = props.id;
    let state = Dispatch::<State>::new().get();

    let token = match state.session.as_ref() {
        Some(session) => session.token.clone(),
        None => {
            return Ok(html! { "You must be logged in" });
        }
    };

    let history = use_future(|| async move {
        Request::get(api_url!("/problems/{}/history", id))
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await?
            .json::<Vec<Submission>>()
            .await
    })?;

    let history_html = match &*history {
        Ok(history) => history
            .iter()
            .map(|submission| make_submission(id, submission))
            .collect::<Html>(),
        Err(_) => html! {
            <span>{ "Failed to load." }</span>
        },
    };

    Ok(html! {
        <div class="h-full bg-white overflow-y-auto">
            {history_html}
        </div>
    })
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

    let model = TextModel::create(code, Some("cpp"), None).unwrap();

    {
        let model = model.clone();
        dispatch.reduce_mut(move |state| {
            state.problems.entry(id).or_default().model = Some(model);
        });
    }

    let options = themed_editor_with_model(model.clone());

    let onfocusout = dispatch.reduce_mut_callback(move |state| {
        let entry = state
            .problems
            .entry(id)
            .or_insert_with(|| Default::default());
        entry.implementation = model.get_value();
    });

    html! {
        <div class="md:h-full" {onfocusout}>
            // <CodeEditor classes="" options = {options}/>
            <CodeEditor classes="h-[60vh] md:h-full md:w-full" options = {options}/>
        </div>
    }
}

#[function_component]
fn SubmitButton(props: &ProblemViewProps) -> Html {
    let id = props.id;
    let dispatch = Dispatch::<State>::new();
    let loading = use_state(|| false);

    let submit = {
        let loading = loading.clone();
        dispatch.reduce_mut_future_callback(move |state| {
            let loading = loading.clone();
            Box::pin(async move {
                loading.set(true);

                let token = match state.session.as_ref() {
                    Some(session) => session.token.clone(),
                    None => {
                        state.error = Some("You must be logged in to do that.".to_string());
                        loading.set(false);
                        return;
                    }
                };

                let problem_code = state
                    .problems
                    .get(&id)
                    .expect("Is this even possible?")
                    .implementation
                    .to_string();

                let form = SubmitProblemForm {
                    problem_id: id,
                    implementation: problem_code,
                };

                match Request::post(api_url!("/submit-problem"))
                    .header("Authorization", &format!("Bearer {}", token))
                    .json(&form)
                    .expect("Failed to serialize json")
                    .send()
                    .await
                {
                    Ok(res) => {
                        let res = res.json().await.expect("Request is in an invalid format");

                        state
                            .problems
                            .entry(id)
                            .and_modify(|p| p.submission = Some(res));
                    }
                    Err(_) => {
                        state.error = Some("Could not connect to server".to_string());
                    }
                };

                loading.set(false);
            })
        })
    };

    html! {
        <LoadingButton
            loading={*loading}
            class="p-4 border-l border-neutral-300 bg-green-500 hover:bg-green-400 transition-colors text-white"
            onclick={submit}>
            { "Submit" }
        </LoadingButton>
    }
}

#[function_component]
fn CodeRunner(props: &ProblemViewProps) -> Html {
    let dispatch = Dispatch::<State>::new();
    let id = props.id;

    let toggle_console = dispatch.reduce_mut_callback(move |state| {
        let entry = state
            .problems
            .entry(id)
            .or_insert_with(|| Default::default());
        entry.docker_shown = !entry.docker_shown;
    });

    let docker_shown = use_selector(move |state: &State| {
        state
            .problems
            .get(&id)
            .map(|x| x.docker_shown)
            .unwrap_or(false)
    });

    html! {
        <div class="sticky md:static bottom-0">
            if *docker_shown {
                <InputTester {id} />
            }

            <div class="flex bg-white border-t border-neutral-300">
                <button class="mr-auto p-4 border-r border-neutral-300 hover:bg-neutral-200 transition-colors" onclick={toggle_console}>
                if *docker_shown {
                    { "Hide console" }
                } else {
                    { "Show console" }
                }
                </button>

                <SubmitButton {id} />
            </div>
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
        Request::get(api_url!("/problems/{}", id))
            .send()
            .await?
            .json::<Problem>()
            .await
    })?;

    match &*problem {
        Ok(problem) => Ok(html! {
            <div class="md:grid md:grid-cols-[400px_minmax(0,1fr)] lg:grid-cols-[500px_minmax(0,1fr)] md:grid-rows-full-min md:h-full">
                <div class="md:border-r border-neutral-300 pt-2 md:p-0 row-span-2 flex flex-col">
                    <Suspense>
                        <SubmissionTestList problem_id={id} />
                    </Suspense>
                    <Tabbed class="h-full border-y md:border-b-0 border-neutral-300 overflow-y-auto" titles={ vec!["Description", "History"] }>
                        <Description title={ problem.title.clone() } content={ problem.description.clone() } />
                        <Suspense>
                            <SubmissionHistory {id} />
                        </Suspense>
                    </Tabbed>
                </div>

                <ProblemEditor {id} template={problem.template.clone()} />
                <CodeRunner {id} />
            </div>
        }),
        Err(e) => Ok(html! { e }),
    }
}

#[function_component]
pub fn ProblemView(props: &ProblemViewProps) -> Html {
    html! {
        <div class="h-screen w-screen grid grid-rows-min-full grid-cols-full">
            <Navbar />
            <Suspense>
                <ProblemViewInner id={props.id} />
            </Suspense>
        </div>
    }
}
