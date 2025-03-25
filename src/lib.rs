use components::nav::Nav;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use pages::{not_found::NotFound, question_answers::QuestionAnswers};
use thaw::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::questions::Questions;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let theme = RwSignal::new(Theme::dark());

    view! {
        <Title text="Parshulp" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Meta name="description" content="Basic prototype of a wine-centered Q&A platform" />
        <ConfigProvider theme>
            <Router>
                <Layout class="bg-black text-white h-screen w-screen">
                    <LayoutHeader>
                        <Nav />
                    </LayoutHeader>
                    <Layout>
                        <Routes fallback=|| view! { <NotFound /> }>
                            <Route path=path!("/") view=Questions />
                            <Route path=path!(":id") view=QuestionAnswers />
                            <Route path=path!("/*") view=Questions />
                        </Routes>
                    </Layout>
                </Layout>
            </Router>
        </ConfigProvider>
    }
}
