//! The navigation bar at the top of each page.

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{state::State, Route};

#[derive(Properties, PartialEq)]
struct NavbarLinkProps {
    to: Route,
    #[prop_or_default]
    children: Children,
    #[prop_or_default]
    class: Classes,
}

#[function_component]
fn NavbarLink(props: &NavbarLinkProps) -> Html {
    let link_styles = "font-bold text-lg self-start md:self-center hover:text-neutral-600 transition-colors ease-in-out md:block";

    html! {
        <Link<Route> classes={classes!(link_styles, props.class.clone())}
                     to={props.to.clone()}>
            {props.children.clone()}
        </Link<Route>>
    }
}

#[function_component]
pub fn Navbar() -> Html {
    let session = use_selector(|state: &State| state.session.clone());
    let menu_shown = use_state(|| false);

    let toggle_menu = {
        let menu_shown = menu_shown.clone();
        Callback::from(move |_| {
            menu_shown.set(!*menu_shown);
        })
    };

    let hidden_style = if *menu_shown { "" } else { "hidden" };

    html! {
        <div class="sticky top-0 z-50 w-full">
            <div class="p-4 flex flex-col gap-4 md:flex-row bg-white/90 backdrop-blur-lg border-neutral-300 border-b">
                <div class="flex">
                    <Link<Route> classes="font-extrabold text-2xl hover:text-neutral-600 transition-colors ease-in-out flex items-center"
                                 to={Route::Home}>
                        { "Chico ACM" }
                    </Link<Route>>

                    <button class="md:hidden ml-auto rounded-full p-2 px-5 bg-blue-700 text-blue-50
                                   hover:bg-blue-500 transition-colors" onclick={toggle_menu}>
                        { "Menu" }
                    </button>
                </div>

                <NavbarLink class={hidden_style} to={Route::Problems}>{ "Problems" }</NavbarLink>
                <NavbarLink class={hidden_style} to={Route::Meetings}>{ "Meetings" }</NavbarLink>
                <NavbarLink class={hidden_style} to={Route::Leaderboard}>{ "Leaderboard" }</NavbarLink>

                // If the user currently has no session, we simply display the signup/login buttons,
                // otherwise we show links to logout or view their account.
                if let Some(session) = &*session {
                    <NavbarLink class={classes!("md:ml-auto", hidden_style)} to={Route::Account { username: session.user.username.clone() }}>{ "Account" }</NavbarLink>
                    <NavbarLink class={hidden_style} to={Route::Logout}>{ "Sign out" }</NavbarLink>
                } else {
                    <NavbarLink class={classes!("md:ml-auto", hidden_style)} to={Route::Signup}>{ "Sign up" }</NavbarLink>
                    <NavbarLink class={hidden_style} to={Route::Login}>{ "Sign in" }</NavbarLink>
                }
            </div>
        </div>
    }
}
