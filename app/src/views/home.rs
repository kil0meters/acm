//! Some kind of landing page hopefully

use yew::prelude::*;

use crate::components::Navbar;

#[function_component(HomeView)]
pub fn home_view() -> Html {
    html! {
        <Navbar />
    }
}
