//! Some kind of landing page hopefully

use yew::prelude::*;
use yew::suspense::Suspense;

use crate::{
    components::{Footer, Navbar},
    views::ProblemViewInner,
};

#[function_component]
pub fn HomeView() -> Html {
    html! {
        <div class="overflow-x-hidden flex flex-col gap-4 min-h-screen justify-center items-center">
            <Navbar />

            <h2 class="py-4 text-center text-6xl drop-shadow-md font-extrabold text-transparent bg-clip-text bg-gradient-to-b from-neutral-600 to-neutral-900 dark:from-neutral-50 dark:to-neutral-400">
                { "Chico ACM" }
            </h2>

            <p class="text-lg text-center">{ "We aren't meeting at the moment, but feel free to try your hand at a programming challenge." }</p>

            <div class="border-y md:border md:rounded-lg border-neutral-300 dark:border-neutral-700 overflow-hidden md:shadow-md w-full max-w-screen-xl md:mx-4 grow md:h-0">
                <Suspense>
                    <ProblemViewInner id={1} />
                </Suspense>
            </div>

            <Footer />
        </div>
    }
}
