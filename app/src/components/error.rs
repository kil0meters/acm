use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ErrorBoxProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ErrorBox)]
pub fn error_box(props: &ErrorBoxProps) -> Html {
    html! {
        <div class="error">
            <h2>{ "error." }</h2>

            { for props.children.iter() }
        </div>
    }
}
