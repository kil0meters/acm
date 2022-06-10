use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ShareButtonProps {
    pub path: String,
    pub class: Classes,
}

#[function_component]
pub fn ShareButton(props: &ShareButtonProps) -> Html {
    let shown = use_state(|| false);

    let window = web_sys::window().unwrap();
    let hostname = window.location().hostname().unwrap();

    let share_url = format!("https://{hostname}{}", props.path);

    let onclick = {
        let shown = shown.clone();
        Callback::from(move |_| {
            shown.set(!*shown);
        })
    };

    html! {
        <div class="relative ml-auto">
            <button {onclick} class={props.class.clone()}>
                {"Share"}
            </button>
            if *shown {
                <div class="absolute top-12 right-0 md:left-0 md:right-auto bg-white dark:bg-neutral-900 dark:border-neutral-700 border-neutral-300 border p-4 rounded-md shadow-md text-neutral-800 dark:text-neutral-50 z-50">
                    <pre>
                        <code>{ &share_url }</code>
                    </pre>
                </div>
            }
        </div>
    }
}
