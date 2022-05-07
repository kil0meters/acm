// apparently this saves on bundle size or something
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use log::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod views;

use views::{HomeView, LeaderboardView, ProblemView};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/problem/:id")]
    Problem { id: String },

    #[at("/leaderboard")]
    Leaderboard,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <HomeView /> },
        Route::Problem { id } => html! { <ProblemView id={id.clone() } /> },
        Route::Leaderboard => html! { <LeaderboardView /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
