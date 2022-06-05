//! Shows the list of all visible problems.

use acm::models::Problem;
use gloo_net::http::Request;
use yew::prelude::*;
use yew::suspense::use_future;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    components::Navbar,
    helpers::{is_officer, parse_markdown},
    state::State,
    Route,
};

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

    div.set_class_name("prose prose-neutral");
    div.set_inner_html(&parse_markdown(&props.problem.description));

    html! {
        <Link<Route> classes="sm:rounded-md border-neutral-300 border-y sm:border sm:mx-2 md:m-0 bg-white p-4 hover:shadow-md transition-shadow"
                     to={Route::Problem { id: props.problem.id }}>
            <h1 class="text-2xl font-bold">{ &props.problem.title }</h1>

            { Html::VRef(div.into()) }
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

    let dispatch = Dispatch::<State>::new();
    let state = dispatch.get();

    if let Ok(problems) = &*list {
        Ok(html! {
            <div class="max-w-screen-md mx-auto mt-4 flex flex-col gap-4">
                if is_officer(&state.session) {
                    <Link<Route> classes="ml-auto text-green-50 text-sm font-bold rounded-full bg-green-700 hover:bg-green-500 transition-colors px-4 py-2 mr-4 md:mr-0"
                                 to={Route::ProblemEditor}>{"New Problem"}</Link<Route>>
                }

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
        <>
            <Navbar />

            <Suspense>
                <ProblemListInner />
            </Suspense>
        </>
    }
}
