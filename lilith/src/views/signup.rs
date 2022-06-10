//! A sign up page.

use acm::models::{
    forms::{LoginForm, SignupForm},
    Session,
};
use gloo_net::http::Request;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{
    api_url,
    components::{ErrorBox, Navbar},
    state::State,
    Route,
};

#[function_component]
pub fn SignupView() -> Html {
    let navigator = use_navigator().unwrap();
    let error = use_state(|| None);

    let submit_form = {
        let error = error.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            let form: HtmlFormElement = e.target_unchecked_into();
            let form_data = FormData::new_with_form(&form).unwrap();

            let name = form_data.get("name").as_string().unwrap();
            let username = form_data.get("username").as_string().unwrap();
            let password = form_data.get("password").as_string().unwrap();

            let signup_data = SignupForm {
                name,
                username,
                password,
            };

            let navigator = navigator.clone();
            let error = error.clone();
            spawn_local(async move {
                let dispatch = Dispatch::<State>::new();

                let res: Value = Request::post(api_url!("/signup"))
                    .json(&signup_data)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                match serde_json::from_value::<Session>(res.clone()) {
                    Ok(session) => {
                        dispatch.reduce_mut(move |state| state.session = Some(session));
                        navigator.push(&Route::Home);
                    }
                    Err(_) => {
                        error.set(Some(
                            res.get("error").unwrap().as_str().unwrap().to_string(),
                        ));
                    }
                }
            });
        })
    };

    html! {
        <>
            <Navbar />

            <div class="max-w-md mx-auto px-2 sm:p-0 mt-4 flex flex-col gap-4 text-lg">
                if let Some(e) = &*error {
                    <ErrorBox>{ e }</ErrorBox>
                }

                <h1 class="text-4xl font-bold">{ "Join." }</h1>

                <form class="flex flex-col gap-4"
                    name="signup-form" onsubmit={ submit_form } method="POST">

                    <FormRow name="name" label="Name" maxlength="16" placeholder="'); DROP TABLE STUDENTS;"/>
                    <FormRow name="username" label="Username" pattern=r"[a-zA-Z0-9]+"
                             placeholder="username" maxlength="16" />
                    <FormRow name="password" label="Password" pattern=r"[a-zA-Z0-9!@#$%^&*()\s]+"
                             placeholder="password" maxlength="256" input_type="password" />

                    <button class="rounded hover:ring transition-all focus:ring-4 border border-green-700
                                   hover:bg-green-500 ring-green-700 text-green-100 p-2 bg-green-600" type="submit">{ "Sign up" }</button>
                </form>
            </div>
        </>
    }
}

#[function_component]
pub fn LoginView() -> Html {
    let navigator = use_navigator().unwrap();
    let error = use_state(|| None);
    let dispatch = Dispatch::<State>::new();

    // This is code that's executed whenever the submit button is pressed. We read the values of
    // the form, then submit a request to the server, updating the session if successful.
    let submit_form = {
        let error = error.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            let form: HtmlFormElement = e.target_unchecked_into();
            let form_data = FormData::new_with_form(&form).unwrap();

            let username = form_data.get("username").as_string().unwrap();
            let password = form_data.get("password").as_string().unwrap();

            let login_data = LoginForm { username, password };

            let dispatch = dispatch.clone();
            let navigator = navigator.clone();
            let error = error.clone();

            // This version of yew doesn't have builtin handling for async, we use this function
            // that waits for a future using Javascript's event loop.
            spawn_local(async move {
                // You might think this code is ugly, but it actually makes quite a bit of sense:
                let res: Value = Request::post(api_url!("/login"))
                    .json(&login_data) // Attach a JSON object to the request
                    .unwrap() // Ignore parsing errors
                    .send() // Send the request
                    .await // Wait for the future to establish
                    .unwrap() // Ignore request errors (TODO: Don't ignore errors)
                    .json() // Parse the response body into JSON
                    .await // Wait for THAT future
                    .unwrap(); // Ignore any parsing errors

                match serde_json::from_value::<Session>(res.clone()) {
                    Ok(session) => {
                        dispatch.reduce_mut(move |state| state.session = Some(session));
                        navigator.push(&Route::Home);
                    }
                    Err(_) => {
                        error.set(Some(
                            res.get("error").unwrap().as_str().unwrap().to_string(),
                        ));
                    }
                }
            });
        })
    };

    html! {
        <>
            <Navbar />

            <div class="max-w-md mx-auto px-2 sm:p-0 mt-4 flex flex-col gap-4 text-lg">
                if let Some(e) = &*error {
                    <ErrorBox>{ e }</ErrorBox>
                }

                <h1 class="text-4xl font-bold">{ "Login." }</h1>

                <form class="flex flex-col gap-4"
                    name="login-form" onsubmit={ submit_form } method="POST">

                    <FormRow name="username" label="Username" pattern=r"[a-zA-Z0-9]+"
                             placeholder="username" maxlength="16" />
                    <FormRow name="password" label="Password" pattern=r"[a-zA-Z0-9!@#$%^&*()\s]+"
                             placeholder="password" maxlength="256" input_type="password" />

                    <button class="rounded hover:ring transition-all focus:ring-4 border border-green-700
                                   hover:bg-green-500 ring-green-700 text-green-100 p-2 bg-green-600" type="submit">{ "Sign in" }</button>
                </form>
            </div>
        </>
    }
}

#[derive(PartialEq, Properties)]
struct FormRowProps {
    label: &'static str,
    placeholder: &'static str,
    name: &'static str,
    pattern: Option<&'static str>,
    maxlength: &'static str,
    input_type: Option<&'static str>,
}

#[function_component]
fn FormRow(props: &FormRowProps) -> Html {
    let FormRowProps {
        label,
        placeholder,
        name,
        pattern,
        maxlength,
        input_type,
    } = *props;

    html! {
        <div class="flex flex-col gap-2">
            <label for={name} class="">{ label }</label>
            <input {name}
                   {placeholder}
                   class="border-neutral-300 dark:border-neutral-700 border rounded p-2 bg-neutral-50 dark:bg-neutral-900 outline-0 transition-shadow focus:ring dark:ring-neutral-700 ring-neutral-300"
                   {pattern}
                   minlength="1"
                   {maxlength}
                   type={input_type}
                   required={true} />
        </div>
    }
}
