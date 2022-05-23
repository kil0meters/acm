use acm::models::{Auth, Submission, User};
use gloo_net::http::Request;
use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use yew_router::prelude::*;

use crate::{components::Navbar, Route};

#[function_component]
fn RecentSubmissions(props: &AccountViewProps) -> HtmlResult {
    let username = props.username.clone();

    let submissions = use_future(|| async move {
        Request::get(&format!("/api/user-info/{}/submissions", username))
            .send()
            .await?
            .json::<Vec<Submission>>()
            .await
    })?;

    let submissions_html = match &*submissions {
        Ok(submissions) => submissions
            .iter()
            .map(|s| {
                html! {
                    <div class="padded card">
                        if s.success {
                            <span class="submission-passed">
                                { "Passed" }
                            </span>
                            <span class="submission-runtime">{ s.runtime } { "ms" }</span>
                        } else {
                            <span class="submission-failed">
                                { "Failed" }
                            </span>
                        }

                        <Link<Route> classes="view-problem" to={Route::Problem { id: s.problem_id }}>{"View Problem"}</Link<Route>>

                        <pre>
                            { &s.code }
                        </pre>
                    </div>
                }
            })
            .collect::<Html>(),
        Err(_) => html! { "Failed to fetch submissions" },
    };

    Ok(html! {
        <>
            <h2>{"Recent Submissions"}</h2>

            <div class="previous-submissions">
                { submissions_html }
            </div>
        </>
    })
}

#[derive(PartialEq, Properties)]
pub struct AccountViewProps {
    pub username: String,
}

#[function_component]
pub fn AccountViewInner(props: &AccountViewProps) -> HtmlResult {
    let navigator = use_navigator().unwrap();

    let username = props.username.clone();
    let user = use_future(|| async move {
        Request::get(&format!("/api/user-info/{}", username))
            .send()
            .await?
            .json::<User>()
            .await
    })?;

    if let Ok(user) = &*user {
        Ok(html! {
            <div class="account-view-wrapper">
                <div class="whatever">
                    <h1>{ &user.name }</h1>
                    <h3>{ &user.username }</h3>


                    {
                        match user.auth {
                                Auth::ADMIN => html! { <span class="ranking">{ "Admin" }</span> },
                                Auth::OFFICER => html! { <span class="ranking">{ "Officer" }</span> },
                                Auth::MEMBER => html! { <span class="ranking">{ "Member" }</span> }
                        }
                    }

                </div>

                <div>
                    /* <div class="stats padded card">

                    </div> */

                    <Suspense>
                        <RecentSubmissions username = { user.username.clone() } />
                    </Suspense>
                </div>

            </div>
        })
    } else {
        navigator.push(&Route::Home);
        Ok(html! {})
    }
}

#[function_component]
pub fn AccountView(props: &AccountViewProps) -> Html {
    html! {
        <div class="container">
            <Navbar />
            <Suspense>
                <AccountViewInner username={props.username.clone()} />
            </Suspense>
        </div>
    }
}
