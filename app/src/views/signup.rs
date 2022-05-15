//! A sign up page.

use acm::models::forms::SignupForm;

use wasm_bindgen_futures::spawn_local;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;

use crate::components::Navbar;

#[function_component(SignupView)]
pub fn signup_view() -> Html {
    let submit_form = {
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            let form: HtmlFormElement = e.target_unchecked_into();
            let form_data = FormData::new_with_form(&form).unwrap();

            let name = form_data.get("name").as_string().unwrap();
            let username = form_data.get("username").as_string().unwrap();
            let password = form_data.get("password").as_string().unwrap();

            // TODO: verify password
            //
            // let verify_password = form_data.get("verify_password").as_string().unwrap();

            let signup_data = SignupForm {
                name,
                username,
                password,
            };

            // TODO: We should probably automatically sign the user in after they've signed up, but
            // what can you do.
            spawn_local(async move {
                let client = reqwest::Client::new();
                client
                    .post("http://127.0.0.1:8080/api/signup")
                    .json(&signup_data)
                    .send()
                    .await
                    .unwrap();
            });
        })
    };

    html! {
        <>
            <Navbar />

            <div class="auth-wrapper">
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
