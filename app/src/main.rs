// apparently this saves on bundle size or something
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use acm::models::Session;
use log::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod views;

use views::{
    HomeView,
    LeaderboardView,
    LoginView,
    ProblemEditorView,
    ProblemView,
    ProblemListView,
    SignupView
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
    let ctx: UseStateHandle<Option<Session>> = use_state(|| None);

    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    if let Some(session) = &*ctx {
        local_storage
            .set_item("session", &serde_json::to_string(session).unwrap())
            .unwrap();
    } else if let Some(session_str) = local_storage.get_item("session").unwrap() {
        let session = serde_json::from_str(&session_str).unwrap();
        ctx.set(Some(session));
    }

    html! {
        <ContextProvider<UseStateHandle<Option<Session>>> context={ctx.clone()}>
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
