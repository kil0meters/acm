//! # app
//!
//! This crate contains code for the frontend.

// apparently this saves on bundle size or something
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

mod components;
mod helpers;
mod state;
mod views;

use components::{ErrorBox, Modal};
use state::State;
use views::{
    AccountView, HomeView, LeaderboardView, LoginView, ProblemEditorView, ProblemListView,
    ProblemView, SignupView,
};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/u/:username")]
    Account { username: String },

    #[at("/")]
    Home,

    #[at("/signup")]
    Signup,

    #[at("/login")]
    Login,

    #[at("/problems/:id")]
    Problem { id: i64 },

    #[at("/problems")]
    Problems,

    #[at("/leaderboard")]
    Leaderboard,

    #[at("/create-problem")]
    ProblemEditor,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Account { username } => html! { <AccountView {username} /> },
        Route::Home => html! { <HomeView /> },
        Route::Problem { id } => html! { <ProblemView {id} /> },
        Route::Problems => html! { <ProblemListView /> },
        Route::Leaderboard => html! { <LeaderboardView /> },
        Route::Signup => html! { <SignupView /> },
        Route::Login => html! { <LoginView /> },
        Route::ProblemEditor => html! { <ProblemEditorView /> },
    }
}

#[function_component]
fn App() -> Html {
    let dispatch = Dispatch::<State>::new();

    let dismiss_error = dispatch.reduce_mut_callback(|state| {
        state.error = None;
    });

    let error = use_selector(|state: &State| state.error.clone());

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />

            // Generic error modal used throughout application
            if let Some(error) = &*error {
                <Modal shown={true} onclose={dismiss_error}>
                    <ErrorBox>
                        { error }
                    </ErrorBox>
                </Modal>
            }
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
