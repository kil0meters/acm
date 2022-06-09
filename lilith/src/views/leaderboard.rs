//! A leaderboard view that shows the current standing of users in the club.
//!
//! Ideally this would be setup in a non-competitive manner.

use acm::models::LeaderboardItem;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use yew_router::prelude::*;

use crate::api_url;
use crate::{components::Navbar, Route};

#[derive(Debug, PartialEq, Properties, Serialize, Deserialize)]
struct LeaderboardEntryProps {
    item: LeaderboardItem,
    position: usize,
}

#[function_component]
fn LeaderboardEntry(props: &LeaderboardEntryProps) -> Html {
    html! {
        <Link<Route> to={Route::Account { username: props.item.username.clone() }}
                     classes="border-b border-neutral-300 p-4 last:border-b-0 flex flex-row gap-4 first:rounded-t-md last:rounded-b-md hover:bg-neutral-100 transition-colors">
            <div class="bg-blue-700 text-neutral-50 flex items-center justify-center rounded-full w-9 h-9 text-xl font-bold self-center">{ props.position }</div>
            <div class="flex flex-col">
                <span class="text-xl font-bold text-neutral-800">{ &props.item.name }</span>
                <span class="text-neutral-500">{ &props.item.username }</span>
            </div>
            <span class="ml-auto bg-yellow-300 text-yellow-800 rounded-full px-4 h-9 self-center flex items-center">{ props.item.count } { if props.item.count > 1 { " Stars" } else { " Star" } } </span>
        </Link<Route>>
    }
}

#[function_component]
fn LeaderboardViewInner() -> HtmlResult {
    let leaderboard_items = use_future(|| async move {
        Request::get(api_url!("/leaderboard/first-place"))
            .send()
            .await?
            .json::<Vec<LeaderboardItem>>()
            .await
    })?;

    let list_html = match &*leaderboard_items {
        Ok(items) => items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                html! {
                    <LeaderboardEntry position={i+1} item={item.clone()} />
                }
            })
            .collect::<Html>(),
        Err(e) => html! { e },
    };

    Ok(html! {
        <div class="flex flex-col border-y sm:rounded-md sm:border sm:m-2 md:m-0 border-neutral-300 bg-white">
            { list_html }
        </div>
    })
}

#[function_component]
pub fn LeaderboardView() -> Html {
    html! {
        <>
            <Navbar />
            <div class="max-w-screen-md mx-auto">
                <h1 class="text-3xl font-extrabold p-2">{"Leaderboard"}</h1>

                <Suspense>
                    <LeaderboardViewInner />
                </Suspense>
            </div>
        </>
    }
}
