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
        <div class="overflow-x-hidden">
            <Navbar />

            <div class="max-w-screen-lg mx-auto">
                <div class="mx-6 mt-8">
                <img class="shadow-md rounded-lg rotate-1 hover:shadow-lg hover:rotate-0 lg:scale-105 hover:scale-105 lg:hover:scale-110 transition-all duration-500"
                     src="https://web.archive.org/web/20211220234950im_/http://chico-acm.com/images/acm-club.png"
                     height="2039" width="4019"/>
                </div>

                <h2 class="py-8 text-center text-6xl drop-shadow-lg font-extrabold text-transparent bg-clip-text bg-gradient-to-tl from-blue-600 to-orange-500">
                    { "fancy tagline" }
                </h2>

                <p class="text-lg text-center mb-4">{ "We aren't meeting at the moment, but feel free to try your hand at a programming challenge." }</p>

                <div class="border-y md:border md:rounded-lg border-neutral-300 dark:border-neutral-700 overflow-hidden md:shadow-md md:h-[40rem]">
                    <Suspense>
                        <ProblemViewInner id={1} />
                    </Suspense>
                </div>
            </div>

            <Footer />
        </div>
    }
}
