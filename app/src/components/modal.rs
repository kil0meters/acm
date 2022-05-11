use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub children: Children,
    pub onclose: Callback<()>,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let shown = use_state(|| true);

    let shown_tmp = shown.clone();
    let onclose_tmp = props.onclose.clone();
    html! {
        if *shown {
            <div class="modal">
                <button class="button grey modal-button" onclick={Callback::from(move |_| {
                    shown_tmp.set(false);
                    onclose_tmp.emit(());
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
