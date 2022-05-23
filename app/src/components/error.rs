//! A red error box component that takes in arbitrary children.

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ErrorBoxProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ErrorBox(props: &ErrorBoxProps) -> Html {
    html! {
        <div class="padded card error">
            <h2>{ "error." }</h2>

            { for props.children.iter() }
        </div>
    }
}
