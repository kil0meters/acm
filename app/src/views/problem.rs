//! An editor view showing a single problem.

use acm::models::{Problem, Session};
use monaco::api::{CodeEditorOptions, TextModel};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::{CodeEditor, Navbar};

#[derive(Clone, Debug, PartialEq, Properties)]
struct TestProps {
    test_id: String,
    name: String,
    problem_id: i64,
    is_error: bool,
}

#[function_component]
fn Test(props: &TestProps) -> Html {
    html! {
        <a class={classes!("test", if props.is_error { "failure" } else { "success" })}
            href={format!("/problems/{}/tests/{}", props.problem_id, props.test_id)}>{ props.name.clone() }</a>
    }
}

#[derive(PartialEq, Properties)]
struct TestsProps {
    problem_id: i64,
}

#[function_component]
fn Tests(props: &TestsProps) -> Html {
    let shown = use_state(|| false);
    let onclick = {
        let shown = shown.clone();
        Callback::from(move |_| shown.set(!*shown))
    };

    html! {
        <div class="tests-wrapper">
            if *shown {
                <a class="hide-tests" onclick={onclick}>{ "Hide tests" }</a>
                <div class="tests">
                    {
                        (0..=100).into_iter().map(|test_number| {
                            html!{
                                <Test problem_id={props.problem_id} name={format!("Test #{}", test_number)} test_id="asdf" is_error={if test_number % 3 == 0 { false } else {true} } />
                            }
                        }).collect::<Html>()
                    }
                </div>
            } else {
                <a class="hide-tests" onclick={onclick}>{ "Show tests" }</a>
            }
        </div>
    }
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
pub fn ProblemView(props: &ProblemViewProps) -> Html {
    let id = props.id;

    // let ctx = use_context::<UseStateHandle<Option<Session>>>().unwrap();

    let data = use_state(|| None);
    let code = use_state(|| TextModel::create("", Some("cpp"), None).unwrap());
    let options =
        Rc::new(CodeEditorOptions::default().with_model((*code).clone())).to_sys_options();

    options.set_font_size(Some(18.0));
    options.set_automatic_layout(Some(true));

    {
        let data = data.clone();
        let code = code.clone();

        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let res = reqwest::get(format!("http://127.0.0.1:8080/api/problems/{}", id))
                        .await
                        .unwrap()
                        .json::<Problem>()
                        .await
                        .unwrap();

                    (*code).set_value(&res.template);

                    data.set(Some(res));
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="problem-wrapper">
            <Navbar />

            if let Some(problem) = &*data {
                <div class="sidebar-wrapper">
                    <Tests problem_id={id} />
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
            } else {
                { "Loading..." }
            }
        </div>
    }
}
