use leptos::prelude::*;

use leptos::Params;
use leptos_router::{hooks::use_params, params::Params};

use super::questions::Tag;

#[derive(Params, PartialEq)]
struct QuestionParams {
    id: Option<usize>,
}

#[component]
pub fn QuestionAnswers() -> impl IntoView {
    let params = use_params::<QuestionParams>();

    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or_default()
    };

    view! {
        {move || {
            get_answers_by_id(id() as u32)
                .into_iter()
                .map(|a| view! { <div>{a.answer}</div> })
                .collect_view()
        }}
    }
}

#[derive(Clone)]
pub struct Answer {
    id: u32,
    question_id: u32,
    answer: String,
    tags: Vec<Tag>,
}

fn get_answers_by_id(id: u32) -> Vec<Answer> {
    answers()
        .iter()
        .cloned()
        .filter(|a| a.question_id == id)
        .collect()
}

fn answers() -> Vec<Answer> {
    vec![Answer {
        id: 11,
        question_id: 1,
        answer: "25, because, yolo.".to_string(),
        tags: vec![Tag {
            id: 111,
            value: "chardonnay".to_string(),
        }],
    }]
}
