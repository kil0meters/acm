use acm::models::Submission;
use gloo_net::http::Request;
use yew::prelude::*;
use yew::suspense::{use_future, Suspense};
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::state::State;
use crate::Route;
use crate::{
    api_url,
    components::{Navbar, SubmissionFeedback},
};

#[derive(Properties, PartialEq)]
pub struct SubmisisonViewProps {
    pub id: i64,
}

#[function_component]
fn SubmissionViewInner(props: &SubmisisonViewProps) -> HtmlResult {
    let id = props.id;
    let navigator = use_navigator().unwrap();
    let dispatch = Dispatch::<State>::new();

    let submission = use_future(|| async move {
        Request::get(api_url!("/submissions/{}", id))
            .send()
            .await?
            .json::<Submission>()
            .await
    })?;

    match &*submission {
        Ok(submission) => {
            let problem_id = submission.problem_id;

            let submission_tmp = submission.clone();
            let view_in_editor = dispatch.reduce_mut_callback_with(move |state, e: MouseEvent| {
                e.prevent_default();

                let problem = state.problems.entry(problem_id).or_default();
                problem.implementation = submission_tmp.code.clone();
                problem.submission = Some(submission_tmp.clone());

                navigator.push(&Route::Problem { id: problem_id });
            });

            Ok(html! {
                <div class="bg-white md:rounded-xl border border-neutral-300 flex flex-col mt-4 md:grid md:grid-cols-5 overflow-hidden max-w-screen-md md:mx-auto md:mt-[20vh]">
                    <div class="p-4 col-span-3 border-neutral-300 border-b md:border-b-0 md:border-r flex flex-col gap-4">
                        <div class="flex flex-col gap-1">
                            <h1 class="text-3xl font-extrabold">{ "Problem " } { submission.problem_id }</h1>
                            <span class="text-sm text-neutral-500">
                                // TODO: Actually fetch username
                                <Link<Route>
                                    to={Route::Account { username: "kilometers".to_string() }}
                                    classes="hover:underline"
                                >{ "Miles Benton" }</Link<Route>>
                                {" • "}
                                { submission.time.format("%B %-d, %Y @ %-I:%M %p").to_string() }
                            </span>
                        </div>

                        <pre class="bg-blue-50 p-4 rounded-md">
                            <code>{ &submission.code }</code>
                        </pre>

                        <div class="mt-auto p-4 rounded-md bg-yellow-300 border-yellow-500 border text-yellow-900">
                            <h1 class="font-bold text-xl mb-2">{ "Warning" }</h1>
                            <p>{ "You will lose your current progress." }</p>
                        </div>

                        <a href={format!("/problems/{}", submission.problem_id)}
                           onclick={view_in_editor}
                           class="rounded-full p-2 bg-blue-600 hover:bg-blue-500 text-blue-50 transition-colors text-center">
                            { "View in editor" }
                        </a>
                    </div>
                    <div class="col-span-2 border-b border-neutral-300 md:border-0">
                        <SubmissionFeedback share={false} submission={submission.clone()} />
                    </div>
                </div>
            })
        }
        Err(_) => Ok(html! {"Failed to fetch submission"}),
    }
}

#[function_component]
pub fn SubmissionView(props: &SubmisisonViewProps) -> Html {
    html! {
        <>
            <Navbar />

            <Suspense>
                <SubmissionViewInner id={props.id} />
            </Suspense>
        </>
    }
}
