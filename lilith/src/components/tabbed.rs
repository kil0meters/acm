//! A tabbed container that takes an arbitrary number of children.

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabbedProps {
    pub titles: Vec<&'static str>,
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn Tabbed(props: &TabbedProps) -> Html {
    let focused_window = use_state(|| 0);

    let focused_window_n = *focused_window;
    let focused_window = focused_window.clone();

    let styles = "bg-white grid grid-rows-min-full grid-cols-full dark:bg-black";

    html! {
        <div class={ classes!(styles, props.class.clone()) }>
            <div class="flex border-neutral-300 dark:border-neutral-700 border-b">
                {
                    props.titles.iter().enumerate().map(move |(i, title)| {

                        let class_string = if *focused_window == i {
                            "px-4 py-2 bg-neutral-300 transition-colors dark:bg-neutral-800 dark:hover:bg-neutral-700 hover:bg-neutral-100 border-neutral-300 dark:border-neutral-700 border-r"
                        } else {
                            "px-4 py-2 bg-neutral-200 transition-colors dark:bg-neutral-800 dark:hover:bg-neutral-700 hover:bg-neutral-50 border-neutral-300 dark:border-neutral-700 border-r"
                        };

                        let focused_window = focused_window.clone();
                        html! {
                            <button class={ class_string } onclick={Callback::from(move |_| {
                                focused_window.set(i);
                            })}>{ title }</button>
                        }
                    }).collect::<Html>()
                }
            </div>

            { props.children.iter().nth(focused_window_n).unwrap() }
        </div>
    }
}
