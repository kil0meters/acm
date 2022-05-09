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

    let focused_window_tmp = focused_window.clone();
    html! {
        <div class={ classes!("tabbed", props.class.clone()) }>
            <div class="tabbed-buttons">
                {
                    props.titles.iter().enumerate().map(move |(i, title)| {
                        let focused_window_tmp2 = focused_window_tmp.clone();

                        let class_string = if *focused_window_tmp == i {
                            "tabbed-button focused"
                        } else {
                            "tabbed-button"
                        };

                        html! {
                            <button class={ class_string } onclick={Callback::from(move |_| {
                                focused_window_tmp2.set(i);
                            })}>{ title }</button>
                        }
                    }).collect::<Html>()
                }
            </div>

            { props.children.iter().nth(*focused_window).unwrap() }
        </div>
    }
}
