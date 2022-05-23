//! A fullscreen popup that can be closed.
//!
//! TODO: This should be refactored to use `<dialog>`

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub children: Children,
    pub shown: bool,
    pub onclose: Callback<()>,
}

#[function_component]
pub fn Modal(props: &ModalProps) -> Html {
    let onclose = props.onclose.clone();
    html! {
        if props.shown {
            <div class="modal">
                <div class="modal-wrapper">
                    { for props.children.iter() }

                    <button class="button grey modal-button" onclick={Callback::from(move |_| {
                        onclose.emit(());
                    })}>{ "Close" }</button>
                </div>
            </div>
        } else {
            <></>
        }
    }
}
