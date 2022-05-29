//! The navigation bar at the top of each page.

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{helpers::is_officer, state::State, Route};

#[function_component]
pub fn Navbar() -> Html {
    let session = use_selector(|state: &State| state.session.clone());
    let dispatch = Dispatch::<State>::new();
    let menu_shown = use_state(|| false);

    let logout = dispatch.reduce_mut_callback(|state| {
        state.session = None;
    });

    let toggle_menu = {
        let menu_shown = menu_shown.clone();
        Callback::from(move |_| {
            menu_shown.set(!*menu_shown);
        })
    };

    html! {
        <div class={classes!("navbar", if *menu_shown { "navbar-open" } else { "" })}>
            <div class="title-container">
                <Link<Route> classes="navbar-title navbar-link" to={Route::Home}> { "Chico ACM" }</Link<Route>>
                <button class="menu-button button grey" onclick={toggle_menu}>{ "Menu" }</button>
            </div>

            <Link<Route> classes="navbar-link" to={Route::Problems}>{ "Problems" }</Link<Route>>
            <Link<Route> classes="navbar-link" to={Route::Meetings}>{ "Meetings" }</Link<Route>>
            <Link<Route> classes="navbar-link" to={Route::Leaderboard}>{ "Leaderboard" }</Link<Route>>

            if is_officer(&*session) {
                <Link<Route> classes="navbar-link" to={Route::ProblemEditor}>{ "Create Problem" }</Link<Route>>
            }

            // If the user currently has no session, we simply display the signup/login buttons,
            // otherwise we show links to logout or view their account.
            if let Some(session) = &*session {
                <Link<Route> classes="button blue navbar-link signup-button" to={Route::Account { username: session.user.username.clone() }}>{ "Account" }</Link<Route>>
                <button class="navbar-link" onclick={logout}>{ "Logout" }</button>
            } else {
                <Link<Route> classes="button blue navbar-link signup-button" to={Route::Signup}>{ "Sign up" }</Link<Route>>
                <Link<Route> classes="navbar-link" to={Route::Login}>{ "Login" }</Link<Route>>
            }
        </div>
    }
}
