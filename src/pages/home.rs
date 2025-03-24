use leptos::prelude::*;
use leptos_router::components::A;
use reactive_stores::Store;

use crate::components::question::Question;

#[derive(Store, Debug, Clone)]
pub struct Data {
    #[store(key: String = |row| row.id.clone())]
    rows: Vec<QuestionData>,
}

#[derive(Store, Debug, Clone)]
pub struct QuestionData {
    pub id: String,
    pub question: String,
    pub tags: Vec<String>,
}
/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let questions = Store::new(Data {
        rows: vec![
            QuestionData {
                id: "1".to_string(),
                question: "What is the capital of France?".to_string(),
                tags: vec!["geography".to_string(), "history".to_string()],
            },
            QuestionData {
                id: "2".to_string(),
                question: "What is the capital of Spain?".to_string(),
                tags: vec!["geography".to_string()],
            },
            QuestionData {
                id: "3".to_string(),
                question: "What is the capital of Germany?".to_string(),
                tags: vec!["geography".to_string()],
            },
            QuestionData {
                id: "4".to_string(),
                question: "What is the capital of Italy?".to_string(),
                tags: vec!["geography".to_string()],
            },
            QuestionData {
                id: "5".to_string(),
                question: "What is the capital of the United Kingdom?".to_string(),
                tags: vec!["geography".to_string()],
            },
        ],
    });

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container p-4 mx-auto bg-black text-white flex flex-col gap-3">
                <For
                    each=move || questions.rows()
                    key=|row| row.read().id.clone()
                    children=|child| {
                        let id = child.clone().id();
                        let question = child.clone().question();
                        let tags = child.tags();
                        view! {
                            <div class="flex justify-between items-center  border-2 border-grey p-4 rounded-lg">
                                <div class="flex flex-col">
                                    <A href=move || {
                                        format!("/{}", id.get())
                                    }>{move || question.get()}</A>
                                    <div class="flex items-center gap-3">
                                        {move || {
                                            // The tags should represent the values of the metadata
                                            // Which {{cultivar}} should I... with cultivar=chardonnay
                                            // Clicking on chardonnay should filter by that tag using query params
                                            tags.get()
                                                .into_iter()
                                                .map(|tag| view! { <span>{tag}</span> })
                                                .collect_view()
                                        }}
                                    </div>
                                </div>
                                <button>"..."</button>
                            </div>
                        }
                    }
                />
            </div>
        </ErrorBoundary>
    }
}
