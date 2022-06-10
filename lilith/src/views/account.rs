use acm::models::{Auth, Submission, User};
use gloo_net::http::Request;
use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use yew_router::prelude::*;

use crate::{api_url, components::Navbar, Route};

#[function_component]
fn RecentSubmissions(props: &AccountViewProps) -> HtmlResult {
    let username = props.username.clone();

    let submissions = use_future(|| async move {
        Request::get(api_url!("/user-info/{}/submissions", username))
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
                    <div class="border-y border-neutral-300 dark:border-neutral-700 bg-white dark:bg-black sm:rounded-md sm:m-2 md:m-0 sm:border p-4 flex flex-col gap-4">
                        <div class="flex gap-2">
                            if s.success {
                                    <span class="font-bold text-green-600 text-2xl self-center">
                                        { "Passed" }
                                    </span>
                                    <span class="text-green-600 self-center text-sm">{ s.runtime } { "ms" }</span>
                            } else {
                                <span class="font-bold text-red-600 text-2xl self-center">
                                    { "Failed" }
                                </span>
                            }

                            <Link<Route> classes="ml-auto self-center bg-blue-700 hover:bg-blue-500 transition-colors text-blue-50 px-3 py-2 text-sm rounded-full font-bold"
                                         to={Route::Problem { id: s.problem_id }}>
                                {"View Problem"}
                            </Link<Route>>
                        </div>

                        <pre class="rounded-md bg-blue-50 dark:bg-slate-800 p-2 overflow-auto max-h-72 border border-blue-200 dark:border-slate-700">
                            { &s.code }
                        </pre>
                    </div>
                }
            })
            .collect::<Html>(),
        Err(_) => html! { "Failed to fetch submissions" },
    };

    Ok(html! {
        <div class="flex flex-col gap-4">
            <h2 class="text-2xl font-bold pt-4 px-4 lg:p-0">{"Recent Submissions"}</h2>

            { submissions_html }
        </div>
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
        Request::get(api_url!("/user-info/{}", username))
            .send()
            .await?
            .json::<User>()
            .await
    })?;

    if let Ok(user) = &*user {
        Ok(html! {
            <div class="grid grid-rows-min-full grid-cols-[minmax(0,1fr)] lg:grid-rows-1 lg:grid-flow-col lg:gap-4 lg:p-4 lg:grid-cols-[300px_minmax(0,1fr)] max-w-screen-md lg:max-w-screen-lg mx-auto">
                <div class="flex flex-col gap-2 p-4 lg:p-0">
                    <h1 class="text-2xl font-bold">{ &user.name }</h1>
                    <h3 class="text-neutral-500 dark:text-neutral-400">{ &user.username }</h3>

                    <span class="rounded-full px-4 p-2 bg-neutral-600 text-neutral-50 self-start text-sm">
                    {
                        match user.auth {
                                Auth::ADMIN => "Admin",
                                Auth::OFFICER => "Officer",
                                Auth::MEMBER => "Member"
                        }
                    }
                    </span>

                </div>

                <Suspense>
                    <RecentSubmissions username = { user.username.clone() } />
                </Suspense>

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
        <>
            <Navbar />
            <Suspense>
                <AccountViewInner username={props.username.clone()} />
            </Suspense>
        </>
    }
}
