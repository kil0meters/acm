use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    // some kind of use_context::<Profile>() here would be appropriate.

    html! {
        <div class="navbar-wrapper">
            <div class="navbar-links">
                <Link<Route> classes="navbar-title" to={Route::Home}> { "Chico ACM" }</Link<Route>>
                <Link<Route> classes="navbar-link" to={Route::Home}>{ "Problems" }</Link<Route>>
                <Link<Route> classes="navbar-link" to={Route::Leaderboard}>{ "Leaderboard" }</Link<Route>>
            </div>
        </div>
    }
}
