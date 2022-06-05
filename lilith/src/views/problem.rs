//! An editor view showing a single problem.

use acm::models::{forms::RunTestsForm, Problem};
use gloo_net::http::Request;
use monaco::api::TextModel;

use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use yewdux::prelude::*;

use crate::state::State;
use crate::{
    components::{CodeEditor, InputTester, LoadingButton, Navbar, TestList},
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
        <div class="grow bg-white border-y md:border-b-0 border-neutral-300 p-4 max-h-full overflow-y-auto">
            <h1 class="text-3xl font-bold">{ props.title.clone() }</h1>

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

                let form = RunTestsForm {
                    problem_id: id,
                    test_code: problem_code,
                };

                match Request::post("/api/run-tests")
                    .header("Authorization", &format!("Bearer {}", token))
                    .json(&form)
                    .expect("Failed to serialize json")
                    .send()
                    .await
                {
                    Ok(res) => {
                        let res = res.json().await.expect("Request is in an invalid format");

                        state.test_results.insert(id, res);
                        state.tests_shown = true;
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
        Request::get(&format!("/api/problems/{}", id))
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
                        <TestList problem_id={id} />
                    </Suspense>
                    <Description title={ problem.title.clone() } content={ problem.description.clone() } />
                </div>

                <ProblemEditor {id} template={ problem.template.clone() } />
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
