use yew::prelude::*;
use yew_router::prelude::*;
use acm::models::Session;

use crate::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let ctx = use_context::<UseStateHandle<Option<Session>>>().unwrap();

    // some kind of use_context::<Profile>() here would be appropriate.

    html! {
        <div class="navbar-wrapper">
            <div class="navbar-links">
                <Link<Route> classes="navbar-title" to={Route::Home}> { "Chico ACM" }</Link<Route>>
                <Link<Route> classes="navbar-link" to={Route::Home}>{ "Problems" }</Link<Route>>
                <Link<Route> classes="navbar-link" to={Route::Leaderboard}>{ "Leaderboard" }</Link<Route>>
            </div>

            if (*ctx).is_none() {
                <div class="signup">
                    <Link<Route> classes="signup-button" to={Route::Signup}>{ "Sign up" }</Link<Route>>
                        <Link<Route> classes="navbar-link" to={Route::Login}>{ "Login" }</Link<Route>>
                </div>
            } else {
                <span>{ "Temp" }</span>
            }
        </div>
    }
}
