//! A leaderboard view that shows the current standing of users in the club.
//!
//! Ideally this would be setup in a non-competitive manner.

use acm::models::LeaderboardItem;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use yew_router::prelude::*;

use crate::{components::Navbar, Route};

#[derive(Debug, PartialEq, Properties, Serialize, Deserialize)]
struct LeaderboardEntryProps {
    item: LeaderboardItem,
    position: usize,
}

#[function_component]
fn LeaderboardEntry(props: &LeaderboardEntryProps) -> Html {
    html! {
        <Link<Route> to={Route::Account { username: props.item.username.clone() }} classes="padded leaderboard-item">
            <span class="leaderboard-rank">{ props.position }</span>
            <span class="leaderboard-name">{ &props.item.name }</span>
            <span class="leaderboard-username">{ &props.item.username }</span>
            <span class="leaderboard-stars">{ props.item.count } { if props.item.count > 1 { " Stars" } else { " Star" } } </span>
        </Link<Route>>
    }
}

#[function_component]
fn LeaderboardViewInner() -> HtmlResult {
    let leaderboard_items = use_future(|| async move {
        Request::get("/api/leaderboard/first-place")
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
        <div class="leaderboard-list card">
            { list_html }
        </div>
    })
}

#[function_component]
pub fn LeaderboardView() -> Html {
    html! {
        <div class="container">
            <Navbar />
            <div class="leaderboard-wrapper">
                <h1>{"Leaderboard"}</h1>

                <Suspense>
                    <LeaderboardViewInner />
                </Suspense>
            </div>
        </div>
    }
}
