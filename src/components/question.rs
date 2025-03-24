use leptos::prelude::*;

use crate::pages::home::QuestionData;
use leptos::Params;
use leptos_router::{
    hooks::use_params,
    params::{self, Params},
};

#[derive(Params, PartialEq)]
struct QuestionParams {
    id: Option<usize>,
}

#[component]
pub fn Question(/* questionData: QuestionData */) -> impl IntoView {
    let params = use_params::<QuestionParams>();

    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or_default()
    };

    view! { {id} }
}
