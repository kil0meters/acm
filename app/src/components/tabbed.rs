//! A tabbed container that takes an arbitrary number of children.

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabbedProps {
    pub titles: Vec<&'static str>,
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Tabbed)]
pub fn tabbed(props: &TabbedProps) -> Html {
    let focused_window = use_state(|| 0);

    let focused_window_n = *focused_window;
    let focused_window = focused_window.clone();
    html! {
        <div class={ classes!("tabbed", props.class.clone()) }>
            <div class="tabbed-buttons">
                {
                    props.titles.iter().enumerate().map(move |(i, title)| {

                        let class_string = if *focused_window == i {
                            "tabbed-button focused"
                        } else {
                            "tabbed-button"
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
