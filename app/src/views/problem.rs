use monaco::{api::CodeEditorOptions, yew::CodeEditor};
use std::rc::Rc;
use yew::prelude::*;

use crate::components::Navbar;

#[derive(Clone, Debug, PartialEq, Properties)]
struct TestProps {
    test_id: String,
    name: String,
    is_error: bool,
}

#[function_component(Test)]
fn test(props: &TestProps) -> Html {
    let problem = use_context::<Problem>().unwrap();

    html! {
        <a class={classes!("test", if props.is_error { "failure" } else { "success" })}
            href={format!("/problems/{}/tests/{}", problem.problem_id, props.test_id)}>{ props.name.clone() }</a>
    }
}

#[function_component(Tests)]
fn tests() -> Html {
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
                                <Test name={format!("Test #{}", test_number)} test_id="asdf" is_error={if test_number % 3 == 0 { false } else {true} } />
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

#[function_component(Description)]
fn description() -> Html {
    html! {
        <div class="description-wrapper">
            <h1>{ "First K elements" }</h1>

            <p>
                {"Return the first K elements of a vector."}
            </p>
        </div>
    }
}

#[function_component(CodeRunner)]
fn code_runner() -> Html {
    html! {
        <div class="code-runner-wrapper">
            <a class="submit-button">{ "Submit" }</a>
        </div>
    }
}

#[function_component(Editor)]
fn editor() -> Html {
    let options = Rc::new(
        CodeEditorOptions::default()
            .with_language("cpp".into())
            .with_builtin_theme(monaco::sys::editor::BuiltinTheme::VsDark),
    )
    .to_sys_options();

    options.set_font_size(Some(18.0));

    html! {
        <div class="editor-wrapper">
            <CodeEditor options = {options}/>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Problem {
    problem_id: String,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ProblemViewProps {
    pub id: String,
}

#[function_component(ProblemView)]
pub fn problem_view(props: &ProblemViewProps) -> Html {
    let prop = use_state(|| Problem {
        problem_id: props.id.clone(),
    });

    html! {
        <ContextProvider<Problem> context={(*prop).clone()}>
            <div class="problem-wrapper">
                <Navbar />
                <div class="sidebar-wrapper">
                    <Tests />
                    <Description />
                </div>
                <div class="content-wrapper">
                    <CodeRunner />
                    <Editor />
                </div>
            </div>
        </ContextProvider<Problem>>
    }
}
