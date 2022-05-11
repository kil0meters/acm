use acm::models::{Auth, Session, User};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let ctx = use_context::<UseStateHandle<Option<Session>>>().unwrap();
    let local_storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

    html! {
        <div class="navbar-wrapper">
            <div class="navbar-links">
                <Link<Route> classes="navbar-title navbar-link" to={Route::Home}> { "Chico ACM" }</Link<Route>>
                <Link<Route> classes="navbar-link" to={Route::Problems}>{ "Problems" }</Link<Route>>
                <Link<Route> classes="navbar-link" to={Route::Leaderboard}>{ "Leaderboard" }</Link<Route>>

                // If logged in and of sufficient rank
                // this is awful.
                if let Some(Session { user: User { auth: Auth::OFFICER | Auth::ADMIN, .. }, ..}) = *ctx {
                    <Link<Route> classes="navbar-link" to={Route::ProblemEditor}>{ "Create Problem" }</Link<Route>>
                }
            </div>

            if (*ctx).is_none() {
                <div class="signup">
                    <Link<Route> classes="button blue navbar-link" to={Route::Signup}>{ "Sign up" }</Link<Route>>
                    <Link<Route> classes="navbar-link" to={Route::Login}>{ "Login" }</Link<Route>>
                </div>
            } else {
                <div class="signup">
                    <Link<Route> classes="button blue navbar-link" to={Route::Signup}>{ "Account" }</Link<Route>>
                    <button class="navbar-link" onclick={Callback::from(move |_| {
                        local_storage.remove_item("session").unwrap();
                        ctx.set(None);
                    })}>{ "Logout" }</button>
                </div>
            }
        </div>
    }
}
