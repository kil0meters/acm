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
            <div class="fixed left-0 right-0 top-0 bottom-0 bg-black/30 z-50 overflow-y-auto">
                <div class="mt-[30vh] max-w-lg mx-auto p-2 relative">
                    { for props.children.iter() }

                    <button class="absolute top-6 right-6 rounded-full bg-slate-700 hover:bg-slate-500 text-slate-100 transition-colors px-4 py-2 text-sm" onclick={Callback::from(move |_| {
                        onclose.emit(());
                    })}>{ "Close" }</button>
                </div>
            </div>
        } else {
            <></>
        }
    }
}
