//! # app
//!
//! This crate contains code for the frontend.

// apparently this saves on bundle size or something
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use acm::models::Session;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod views;

use views::{
    HomeView, LeaderboardView, LoginView, ProblemEditorView, ProblemListView, ProblemView,
    SignupView,
};

#[derive(Clone, Routable, PartialEq)]
enum Route {
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

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <HomeView /> },
        Route::Problem { id } => html! { <ProblemView id={id.clone()} /> },
        Route::Problems => html! { <ProblemListView /> },
        Route::Leaderboard => html! { <LeaderboardView /> },
        Route::Signup => html! { <SignupView /> },
        Route::Login => html! { <LoginView /> },
        Route::ProblemEditor => html! { <ProblemEditorView /> },
    }
}

#[function_component(App)]
fn app() -> Html {
    // We setup a state handle around a Session object so that we can update the session from
    // anywhere else in the application. This typically occurs when the users signs out/in.
    let state_context: UseStateHandle<Option<Session>> = use_state(|| None);

    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    // We check if we currently have a Session, saving it to local storage. Otherwise, we attempt
    // to read an existing Session into local storage.
    if let Some(session) = &*state_context {
        local_storage
            .set_item("session", &serde_json::to_string(session).unwrap())
            .unwrap();
    } else if let Some(session_str) = local_storage.get_item("session").unwrap() {
        let session = serde_json::from_str(&session_str).unwrap();
        state_context.set(Some(session));
    }

    html! {
        <ContextProvider<UseStateHandle<Option<Session>>> context={state_context.clone()}>
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </ContextProvider<UseStateHandle<Option<Session>>>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
