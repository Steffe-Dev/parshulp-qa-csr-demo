use reactive_stores::Store;

use crate::pages::questions_data::Tag;

#[derive(Store, Debug, Clone)]
pub struct AnswerStore {
    #[store(key: u32 = |row| row.id)]
    pub rows: Vec<Answer>,
}

#[derive(Store, Debug, Clone)]
pub struct Answer {
    pub id: u32,
    pub question_id: u32,
    pub answer: String,
    #[store(key: u32 = |tag| tag.id)]
    pub tags: Vec<Tag>,
}

pub fn get_questions_filtered_by(id: u32, tag: Option<String>) -> Store<AnswerStore> {
    Store::new(AnswerStore {
        rows: if let Some(tag) = tag {
            get_answers_by_question_id(id)
                .into_iter()
                .filter(|q| q.tags.iter().find(|t| t.value == tag).is_some())
                .collect()
        } else {
            get_answers_by_question_id(id)
        },
    })
}

pub fn get_answers_by_question_id(id: u32) -> Vec<Answer> {
    answers()
        .iter()
        .cloned()
        .filter(|a| a.question_id == id)
        .collect()
}

pub fn answers() -> Vec<Answer> {
    vec![Answer {
        id: 11,
        question_id: 1,
        answer: "22, because that will ensure that an optimal ripeness is reached without compromising on acid.".to_string(),
        tags: vec![Tag {
            id: 1,
            value: "chardonnay".to_string(),
        }],
    },Answer {
        id: 22,
        question_id: 1,
        answer: "32, because, yolo #roesyntjies".to_string(),
        tags: vec![Tag {
            id: 1,
            value: "chardonnay".to_string(),
        }, Tag {
            id: 5,
            value: "cowboy".to_string(),
        }],
    }]
}
