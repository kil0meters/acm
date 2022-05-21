//! Some kind of landing page hopefully

use yew::prelude::*;

use crate::components::Navbar;

#[function_component]
pub fn HomeView() -> Html {
    html! {
        <Navbar />
    }
}
