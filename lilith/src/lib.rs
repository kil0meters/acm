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
    AccountView, HomeView, LeaderboardView, LoginView, LogoutView, MeetingEditorView, MeetingsView,
    ProblemEditorView, ProblemListView, ProblemView, SignupView,
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

    #[at("/logout")]
    Logout,

    #[at("/problems/:id")]
    Problem { id: i64 },

    #[at("/problems")]
    Problems,

    #[at("/leaderboard")]
    Leaderboard,

    #[at("/problems/new")]
    ProblemEditor,

    #[at("/meetings")]
    Meetings,

    #[at("/meetings/:id")]
    Meeting { id: i64 },

    #[at("/meetings/:id/edit")]
    MeetingEditor { id: i64 },

    #[at("/meetings/new")]
    MeetingEditorNew,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Account { username } => html! { <AccountView {username} /> },
        Route::Home => html! { <HomeView /> },
        Route::Problem { id } => html! { <ProblemView {id} /> },
        Route::Problems => html! { <ProblemListView /> },
        Route::Leaderboard => html! { <LeaderboardView /> },
        Route::Login => html! { <LoginView /> },
        Route::Meetings => html! { <MeetingsView /> },
        Route::Meeting { id } => html! { <MeetingsView {id} /> },
        Route::MeetingEditor { id } => html! { <MeetingEditorView {id} /> },
        Route::MeetingEditorNew => html! { <MeetingEditorView /> },
        Route::ProblemEditor => html! { <ProblemEditorView /> },
        Route::Signup => html! { <SignupView /> },
        Route::Logout => html! { <LogoutView /> },
    }
}

#[function_component]
pub fn App() -> Html {
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

/* #[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(&*props.url);

    html! {
        <Router {history}>
            <Switch<Route> render={switch} />
        </Router>
    }
} */
