use yew::prelude::*;

#[function_component]
fn Spinner() -> Html {
    html! {
        "spinner"
    }
}

#[derive(PartialEq, Properties)]
pub struct LoadingButtonProps {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: Children,

    #[prop_or(false)]
    pub loading: bool,

    #[prop_or_default]
    pub onclick: Callback<()>,
}

#[function_component]
pub fn LoadingButton(props: &LoadingButtonProps) -> Html {
    let loading = props.loading;

    let onclick = props.onclick.clone();
    let onclick = Callback::from(move |_| {
        if !loading {
            onclick.emit(());
        }
    });

    html! {
        <button onclick={onclick}
                class={classes!("loading-button",
                if loading { "active" } else { "" },
                props.class.clone())}>

            <span class="loading-button-text">
                {props.children.clone()}
            </span>

            <span class="spinner" />
        </button>

    }
}
