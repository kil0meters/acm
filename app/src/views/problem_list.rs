use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use acm::models::Problem;

use crate::components::Navbar;
use crate::Route;

#[derive(PartialEq, Properties)]
struct ProblemListingProps {
    problem: Problem,
}

#[function_component(ProblemListing)]
fn problem_listing(props: &ProblemListingProps) -> Html {

    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    div.set_class_name("problem-listing-description");
    div.set_inner_html(&markdown::to_html(&props.problem.description));

    html! {
        <Link<Route> classes="problem-listing" to={Route::Problem { id: props.problem.id }}>
            <h1>{ &props.problem.title }</h1>

            { Html::VRef(div.into()) }
        </Link<Route>>

    }
}

#[function_component(ProblemListView)]
pub fn problem_list_view() -> Html {
    let data = use_state(|| Vec::<Problem>::new());

    let data_tmp = data.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let res = reqwest::get("http://127.0.0.1:8080/api/problems")
                    .await
                    .unwrap()
                    .json::<Vec<Problem>>()
                    .await
                    .unwrap();

                data_tmp.set(res);
            });

            || ()
        },
        (),
    );

    html! {
        <>
            <Navbar />

            <div class="problem-list-wrapper">
            {
                data.iter().map(|problem| { html! {
                    <ProblemListing problem = {problem.clone()} /> }
                }).collect::<Html>()
            }
            </div>

        </>
    }
}
