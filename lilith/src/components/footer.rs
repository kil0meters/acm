use yew::prelude::*;

#[function_component]
pub fn Footer() -> Html {
    html! {
        <div class="h-52 w-full mt-8 bg-neutral-100 border-neutral-300 border-t flex items-center justify-center">
            <span class="select-none">{ "made with 🐌" }</span>
        </div>
    }
}
