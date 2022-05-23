//! The navigation bar at the top of each page.

use acm::models::{Auth, Session, User};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{state::State, Route};

#[function_component]
pub fn Navbar() -> Html {
    let session = use_selector(|state: &State| state.session.clone());
    let dispatch = Dispatch::<State>::new();

    let logout = dispatch.reduce_mut_callback(|state| {
        state.session = None;
    });

    html! {
        <div class="navbar-wrapper">
            <div class="navbar-links">
                <Link<Route> classes="navbar-title navbar-link" to={Route::Home}> { "Chico ACM" }</Link<Route>>
                <Link<Route> classes="navbar-link" to={Route::Problems}>{ "Problems" }</Link<Route>>
                <Link<Route> classes="navbar-link" to={Route::Leaderboard}>{ "Leaderboard" }</Link<Route>>

                // If logged in and of sufficient rank
                // this is awful.
                if let Some(Session { user: User { auth: Auth::OFFICER | Auth::ADMIN, .. }, ..}) = &*session {
                    <Link<Route> classes="navbar-link" to={Route::ProblemEditor}>{ "Create Problem" }</Link<Route>>
                }
            </div>

            // If the user currently has no session, we simply display the signup/login buttons,
            // otherwise we show links to logout or view their account.
            if let Some(session) = &*session {
                <div class="signup">
                    <Link<Route> classes="button blue navbar-link" to={Route::Account { username: session.user.username.clone() }}>{ "Account" }</Link<Route>>
                    <button class="navbar-link" onclick={logout}>{ "Logout" }</button>
                </div>
            } else {
                <div class="signup">
                    <Link<Route> classes="button blue navbar-link" to={Route::Signup}>{ "Sign up" }</Link<Route>>
                    <Link<Route> classes="navbar-link" to={Route::Login}>{ "Login" }</Link<Route>>
                </div>
            }
        </div>
    }
}
