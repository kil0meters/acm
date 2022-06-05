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

    let loading_button_classes = "flex gap-2 items-center";

    html! {
        <button onclick={onclick}
                class={classes!(loading_button_classes, props.class.clone())}>

            if loading {
            // if true {
                <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
            }

            <span class="h-5 flex items-center">
                {props.children.clone()}
            </span>


        </button>

    }
}
