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
            let verify_password = form_data.get("verify_password").as_string().unwrap();

            if password != verify_password {
                error.set(Some("Passwords do not match".to_string()));
                return;
            }

            let signup_data = SignupForm {
                name,
                username,
                password,
            };

            let navigator = navigator.clone();
            let error = error.clone();
            spawn_local(async move {
                let dispatch = Dispatch::<State>::new();

                let res: Value = Request::post("/api/signup")
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

            <div class="auth-wrapper padded card">
                if let Some(e) = &*error {
                    <ErrorBox>{ e }</ErrorBox>
                }

                <h1>{ "Join." }</h1>

                <form name="signup-form" class="auth-form" onsubmit={ submit_form } method="POST">
                    <label for="name" class="authorize-form-label">{ "Name" }</label>
                    <input name="name"
                           placeholder={ "'); DROP TABLE STUDENTS" }
                           class="authorize-form-input"
                           required={ true } />
                    <label for="username" class="authorize-form-label">{ "Username" }</label>
                    <input name="username"
                           placeholder={ "'); DROP TABLE STUDENTS" }
                           class="authorize-form-input"
                           pattern=r"[a-zA-Z0-9]+"
                           minlength="1"
                           maxlength="16"
                           required={ true } />
                    <label for="password" class="authorize-form-label">{ "Password" }</label>
                    <input name="password"
                           placeholder={ "hunter2" }
                           class="authorize-form-input"
                           pattern=r"[a-zA-Z0-9!@#$%^&*()\s]+"
                           minlength="8"
                           maxlength="256" type="password"
                           required={ true } />
                    <label for="verify_password" class="authorize-form-label">{ "Verify Password" }</label>
                    <input name="verify_password"
                           placeholder={ "hunter2" }
                           class="authorize-form-input"
                           pattern=r"[a-zA-Z0-9!@#$%^&*()\s]+"
                           minlength="8"
                           maxlength="256" type="password"
                           required={ true } />
                    <button class="button green" type="submit">{ "Sign up" }</button>
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
                let res: Value = Request::post("/api/login")
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

            <div class="auth-wrapper padded card">
                if let Some(e) = &*error {
                    <ErrorBox>{ e }</ErrorBox>
                }

                <h1>{ "Login." }</h1>

                <form name="login-form" class="auth-form" onsubmit={ submit_form } method="POST">
                    <label for="username" class="authorize-form-label">{ "Username" }</label>
                    <input name="username"
                           placeholder={ "'); DROP TABLE STUDENTS" }
                           class="authorize-form-input"
                           pattern=r"[a-zA-Z0-9]+"
                           minlength="1"
                           maxlength="16"
                           required={ true } />
                    <label for="password" class="authorize-form-label">{ "Password" }</label>
                    <input name="password"
                           placeholder={ "hunter2" }
                           class="authorize-form-input"
                           pattern=r"[a-zA-Z0-9!@#$%^&*()\s]+"
                           minlength="8"
                           maxlength="256" type="password"
                           required={ true } />
                    <button class="button green" type="submit">{ "login" }</button>
                </form>
            </div>
        </>
    }
}
