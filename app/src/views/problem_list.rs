//! Shows the list of all visible problems.

use acm::models::Problem;
use gloo_net::http::Request;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_router::prelude::*;

use crate::components::Navbar;
use crate::Route;

#[derive(PartialEq, Properties)]
struct ProblemListingProps {
    problem: Problem,
}

#[function_component]
fn ProblemListing(props: &ProblemListingProps) -> Html {
    let div = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    div.set_class_name("problem-listing-description");
    div.set_inner_html(&markdown::to_html(&props.problem.description));

    html! {
        <Link<Route> classes="problem-listing padded card" to={Route::Problem { id: props.problem.id }}>
            <h1>{ &props.problem.title }</h1>

            { Html::VRef(div.into()) }

            <span class="cover" />
        </Link<Route>>

    }
}

#[function_component]
fn ProblemListInner() -> HtmlResult {
    let list = use_future(|| async move {
        Request::get("/api/problems")
            .send()
            .await?
            .json::<Vec<Problem>>()
            .await
    })?;

    if let Ok(problems) = &*list {
        Ok(html! {
            <div class="problem-list-wrapper">
            {
                problems.iter().map(|problem| { html! {
                    <ProblemListing problem = {problem.clone()} /> }
                }).collect::<Html>()
            }
            </div>
        })
    } else {
        Ok(html!({ "Failed to load" }))
    }
}

#[function_component]
pub fn ProblemListView() -> Html {
    html! {
        <div class="container">
            <Navbar />

            <Suspense>
                <ProblemListInner />
            </Suspense>
        </div>
    }
}
