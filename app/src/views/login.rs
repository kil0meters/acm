//! A login page

use acm::models::{forms::LoginForm, Session};

use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{ErrorBox, Navbar},
    Route,
};

#[function_component]
pub fn LoginView() -> Html {
    let ctx = use_context::<UseStateHandle<Option<Session>>>().unwrap();
    let navigator = use_navigator().unwrap();
    let error = use_state(|| None);

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

            let ctx = ctx.clone();
            let navigator = navigator.clone();
            let error = error.clone();

            // This version of yew doesn't have builtin handling for async, we use this function
            // that waits for a future using Javascript's event loop.
            spawn_local(async move {
                let client = reqwest::Client::new();

                // You might think this code is ugly, but it actually makes quite a bit of sense:
                let res: Value = client
                    .post("http://127.0.0.1:8080/api/login") // Submit a post request
                    .json(&login_data) // Attach a JSON object to the request
                    .send() // Send the request
                    .await // Wait for the future to establish
                    .unwrap() // Ignore errors (TODO: Don't ignore errors)
                    .json() // Parse the response body into JSON
                    .await // Wait for THAT future
                    .unwrap(); // Ignore any parsing errors

                match serde_json::from_value::<Session>(res.clone()) {
                    Ok(session) => {
                        ctx.set(Some(session));
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

            <div class="auth-wrapper">
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
