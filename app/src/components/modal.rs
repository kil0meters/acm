//! A fullscreen popup that can be closed.
//!
//! TODO: This should be refactored to use `<dialog>`

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub children: Children,
    pub onclose: Callback<()>,
}

#[function_component]
pub fn Modal(props: &ModalProps) -> Html {
    let shown = use_state(|| true);

    let onclose = props.onclose.clone();
    html! {
        if *shown {
            <div class="modal">
                <button class="button grey modal-button" onclick={Callback::from(move |_| {
                    shown.set(false);
                    onclose.emit(());
                })}>{ "Close" }</button>
                <div class="modal-wrapper">
                    { for props.children.iter() }
                </div>
            </div>
        } else {
            <></>
        }
    }
}
