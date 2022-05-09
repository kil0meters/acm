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

#[function_component(LeaderboardItem)]
fn leaderboard_item(props: &LeaderboardItemProps) -> Html {
    html! {
        <div class="leaderboard-item">
            <img class="profile-picture" src="https://via.placeholder.com/512" />
            <a class="name" href="/profile/{props.username.clone()}">{props.name.clone()}</a>
            <span class="star-count">{ format!("{} Stars", props.star_count) }</span>
        </div>
    }
}

#[function_component(LeaderboardView)]
pub fn leaderboard_view() -> Html {
    let data = use_state(|| Vec::<LeaderboardItemProps>::new());

    let data_tmp = data.clone();
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let res = reqwest::get("http://127.0.0.1:8080/api/leaderboard")
                    .await
                    .unwrap()
                    .json::<Vec<LeaderboardItemProps>>()
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

            <div class="leaderboard-wrapper">
            {
                data.iter().map(|props| { html! { <LeaderboardItem name={props.name.clone()} username={props.username.clone()} star_count={props.star_count.clone()} /> } }).collect::<Html>()
            }
            </div>

        </>
    }
}
