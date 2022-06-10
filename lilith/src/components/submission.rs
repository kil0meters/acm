use acm::models::Submission;
use yew::prelude::*;

use crate::components::ShareButton;

#[derive(Properties, PartialEq)]
pub struct SubmissionFeedbackProps {
    pub submission: Submission,

    #[prop_or(true)]
    pub share: bool,
}

#[function_component]
pub fn SubmissionFeedback(props: &SubmissionFeedbackProps) -> Html {
    let submission = &props.submission;

    if let Some(error) = &submission.error {
        html! {
            <div class="bg-red-500 dark:bg-red-700 text-red-50 p-4 flex flex-col gap-2 h-full">
                <div class="flex items-start">
                    <h1 class="text-2xl font-bold">{ "error." }</h1>
                    if props.share {
                        <ShareButton
                            path={format!("/submissions/{}", submission.id)}
                            class="bg-red-700 dark:bg-red-800 hover:bg-red-600 dark:hover:bg-red-900 text-red-50 rounded-full px-4 py-2 ml-auto text-sm transition-colors"
                        />
                    }
                </div>

                <pre class="bg-red-700 dark:bg-red-800 overflow-x-auto p-2 rounded">
                    <code>{ error }</code>
                </pre>
            </div>
        }
    } else {
        if submission.success {
            html! {
                <div class="flex-col flex p-4 bg-green-500 dark:bg-green-800 text-green-50 h-full">
                    <div class="flex items-start">
                        <span class="font-bold text-2xl">{ "Congratulations!" }</span>
                        if props.share {
                            <ShareButton
                                path={format!("/submissions/{}", submission.id)}
                                class="bg-green-700 hover:bg-green-600 text-green-50 rounded-full px-4 py-2 ml-auto text-sm transition-colors"
                            />
                        }
                    </div>
                    <span>{ "Your code passed all of the supplied tests." }</span>
                    <span>{ "Ran in " } { submission.runtime } { " ms." }</span>
                </div>
            }
        } else {
            html! {
                <div class="flex-col flex p-4 bg-white dark:bg-black h-full">
                    <div class="flex items-start">
                        <span class="text-red-500 font-bold text-2xl">{ "Failed" }</span>
                        if props.share {
                            <ShareButton
                                path={format!("/submissions/{}", submission.id)}
                                class="bg-neutral-600 hover:bg-neutral-500 text-neutral-50 rounded-full px-4 py-2 ml-auto text-sm transition-colors"
                            />
                        }
                    </div>

                    <span>{ "Your code did not pass all of the tests." }</span>
                    <span>{ "Ran in " } { submission.runtime } { " ms." }</span>
                </div>
            }
        }
    }
}
