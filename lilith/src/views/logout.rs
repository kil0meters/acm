use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{state::State, Route};

#[function_component]
pub fn LogoutView() -> Html {
    let navigator = use_navigator().unwrap();
    let dispatch = Dispatch::<State>::new();

    dispatch.reduce(|_| State::default());
    navigator.push(&Route::Home);

    html! {}
}
