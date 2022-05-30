//! Some kind of landing page hopefully

use yew::prelude::*;
use yew::suspense::Suspense;

use crate::{
    components::{Navbar, Footer},
    views::ProblemViewInner
};

#[function_component]
pub fn HomeView() -> Html {
    html! {
        <div class="homepage">
            <Navbar />

            <div class="home-wrapper">
                <img src="https://web.archive.org/web/20211220234950im_/http://chico-acm.com/images/acm-club.png" />

                <h2>{ "Try a problem." }</h2>

                <p>{ "It's free!*" }</p>

                <Suspense>
                    <ProblemViewInner id={1} />
                </Suspense>
            </div>

            <Footer />
        </div>
    }
}
