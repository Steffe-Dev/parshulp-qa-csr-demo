use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
use thaw::*;

use crate::pages::questions_data;

#[derive(Params, PartialEq)]
struct ProfileParams {
    id: Option<u32>,
}

#[component]
pub fn AddAnswer() -> impl IntoView {
    let params = use_params::<ProfileParams>();
    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or_default()
    };
    let answer_input = RwSignal::new(String::from(""));

    view! {
        <div class="flex flex-col gap-3 p-4">
            <h1 class="text-xl font-light">{move || questions_data::get_question_by(id()).clone().question}</h1>
            <Textarea value=answer_input placeholder="Enter your answer here..." />
            <Button>"Submit"</Button>
        </div>
    }
}
