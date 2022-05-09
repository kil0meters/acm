use acm::models::{forms::LoginForm, Session};
use log::info;
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use web_sys::{FormData, HtmlFormElement};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    components::{ErrorBox, Navbar},
    Route,
};

#[function_component(LoginView)]
pub fn login_view() -> Html {
    let ctx = use_context::<UseStateHandle<Option<Session>>>().unwrap();
    let history = use_history().unwrap();
    let error = use_state(|| None);

    let error2 = error.clone();
    let submit_form = {
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();

            let form: HtmlFormElement = e.target_unchecked_into();
            let form_data = FormData::new_with_form(&form).unwrap();

            let username = form_data.get("username").as_string().unwrap();
            let password = form_data.get("password").as_string().unwrap();

            let login_data = LoginForm { username, password };

            let ctx_tmp = ctx.clone();
            let history_tmp = history.clone();
            let error_tmp = error2.clone();
            spawn_local(async move {
                let client = reqwest::Client::new();
                let res: Value = client
                    .post("http://127.0.0.1:8080/api/login")
                    .json(&login_data)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                match serde_json::from_value::<Session>(res.clone()) {
                    Ok(session) => {
                        ctx_tmp.set(Some(session));
                        history_tmp.push(Route::Home);
                    }
                    Err(_) => {
                        error_tmp.set(Some(
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
