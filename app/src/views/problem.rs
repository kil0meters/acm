//! An editor view showing a single problem.

use acm::models::{test::Test, Problem, Session};
use gloo_net::http::Request;
use monaco::api::{CodeEditorOptions, TextModel};
use serde::Serializer;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::{use_future, Suspense};

use crate::components::{CodeEditor, Navbar};

#[derive(Clone, Debug, PartialEq, Properties)]
struct TestEntryProps {
    test: Test,
}

#[function_component]
fn TestEntry(props: &TestEntryProps) -> Html {
    html! {
        <a class="test success" href="/tmp">{ format!("Test #{}", props.test.index) }</a>
    }
}

#[derive(PartialEq, Properties)]
struct TestsProps {
    problem_id: i64,
}

#[function_component]
fn Tests(props: &TestsProps) -> HtmlResult {
    let shown = use_state(|| false);
    let onclick = {
        let shown = shown.clone();
        Callback::from(move |_| shown.set(!*shown))
    };

    let problem_id = props.problem_id;
    let tests = use_future(|| async move {
        Request::get(&format!("/api/problems/{}/tests", problem_id))
            .send()
            .await?
            .json::<Vec<Test>>()
            .await
    })?;

    let tests_html = match *tests {
        Ok(ref tests) => tests
            .into_iter()
            .map(|t| {
                html! {
                    <TestEntry test={t.clone()} />
                }
            })
            .collect::<Html>(),
        Err(ref failure) => failure.to_string().into(),
    };

    Ok(html! {
        <div class="tests-wrapper">
            if *shown {
                <a class="hide-tests" onclick={onclick}>{ "Hide tests" }</a>
                <div class="tests">
                    { tests_html }
                </div>
            } else {
                <a class="hide-tests" onclick={onclick}>{ "Show tests" }</a>
            }
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
        <div class="description-wrapper">
            <h1>{ props.title.clone() }</h1>

            { Html::VRef(div.into()) }
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ProblemViewProps {
    pub id: i64,
}

#[function_component]
fn ProblemViewInner(props: &ProblemViewProps) -> HtmlResult {
    let id = props.id;
    let problem = use_future(|| async move {
        Request::get(&format!("/api/problems/{}", id))
            .send()
            .await?
            .json::<Problem>()
            .await
    })?;

    match &*problem {
        Ok(problem) => {
            let code = TextModel::create(&problem.template, Some("cpp"), None).unwrap();
            let options = Rc::new(CodeEditorOptions::default().with_model(code.clone())).to_sys_options();

            options.set_font_size(Some(18.0));
            options.set_automatic_layout(Some(true));

            Ok(html! {
            <div class="problem-wrapper">
                <div class="sidebar-wrapper">
                    <Suspense>
                        <Tests problem_id={id} />
                    </Suspense>
                    <Description title={ problem.title.clone() } content={ problem.description.clone() } />
                </div>
                <div class="content-wrapper">
                    <div class="code-runner-wrapper">
                        <a class="button green">{ "Submit" }</a>
                    </div>

                    <div class="editor-wrapper">
                        <CodeEditor options = {options}/>
                    </div>
                </div>
            </div>
            })
        }
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
