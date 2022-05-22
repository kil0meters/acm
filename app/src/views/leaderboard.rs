//! A leaderboard view that shows the current standing of users in the club.
//!
//! Ideally this would be setup in a non-competitive manner.

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::Navbar;

#[derive(Debug, PartialEq, Properties, Serialize, Deserialize)]
pub struct LeaderboardItemProps {
    name: String,
    username: String,
    star_count: i32,
}

#[function_component]
fn LeaderboardItem(props: &LeaderboardItemProps) -> Html {
    html! {
        <div class="leaderboard-item">
            <img class="profile-picture" src="https://via.placeholder.com/512" />
            <a class="name" href="/profile/{props.username.clone()}">{props.name.clone()}</a>
            <span class="star-count">{ format!("{} Stars", props.star_count) }</span>
        </div>
    }
}

#[function_component]
pub fn LeaderboardView() -> Html {
    let leaderboard_items = use_state(|| Vec::<LeaderboardItemProps>::new());

    {
        let leaderboard_items = leaderboard_items.clone();

        // use_effect_with_deps with no arguments here makes the code within the closure executed
        // exactly once, rather than each time the component is built. This is essential to avoid
        // inappropriate fetches.
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let res = Request::get("http://127.0.0.1:8080/api/leaderboard")
                        .send()
                        .await
                        .unwrap()
                        .json::<Vec<LeaderboardItemProps>>()
                        .await
                        .unwrap();

                    leaderboard_items.set(res);
                });

                || ()
            },
            (),
        );
    }

    html! {
        <>
            <Navbar />

            <div class="leaderboard-wrapper">
            {
                leaderboard_items.iter().map(|props| {
                    html! {
                        <LeaderboardItem name={props.name.clone()}
                                         username={props.username.clone()}
                                         star_count={props.star_count.clone()} />
                    }
                }).collect::<Html>()
            }
            </div>

        </>
    }
}
