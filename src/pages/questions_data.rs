use reactive_stores::Store;

#[derive(Store, Debug, Clone)]
pub struct QuestionStore {
    #[store(key: u32 = |row| row.id)]
    rows: Vec<QuestionData>,
}

#[derive(Store, Debug, Clone)]
pub struct QuestionData {
    pub id: u32,
    pub question: String,
    #[store(key: u32 = |tag| tag.id)]
    pub tags: Vec<Tag>,
}

#[derive(Store, Debug, Clone)]
pub struct Tag {
    pub id: u32,
    pub value: String,
}

pub fn get_question_by(id: u32) -> QuestionData {
    get_questions().into_iter().find(|a| a.id == id).unwrap()
}

pub fn get_questions_filtered_by(tag: Option<String>) -> Store<QuestionStore> {
    Store::new(QuestionStore {
        rows: if let Some(tag) = tag {
            get_questions()
                .into_iter()
                .filter(|q| q.tags.iter().find(|t| t.value == tag).is_some())
                .collect()
        } else {
            get_questions()
        },
    })
}

pub fn get_questions() -> Vec<QuestionData> {
    vec![
        QuestionData {
            id: 1,
            question: "At which balling should I pick Chardonnay?".to_string(),
            tags: vec![
                Tag {
                    id: 1,
                    value: "chardonnay".to_string(),
                },
                Tag {
                    id: 2,
                    value: "viticulture".to_string(),
                },
            ],
        },
        QuestionData {
            id: 2,
            question: "What trellising system should I use for Cabernet Sauvignon?".to_string(),
            tags: vec![
                Tag {
                    id: 3,
                    value: "cabernet sauvignon".to_string(),
                },
                Tag {
                    id: 2,
                    value: "viticulture".to_string(),
                },
            ],
        },
        QuestionData {
            id: 3,
            question: "What yeast should I use for Cabernet Sauvignon?".to_string(),
            tags: vec![
                Tag {
                    id: 3,
                    value: "cabernet sauvignon".to_string(),
                },
                Tag {
                    id: 4,
                    value: "fermentation".to_string(),
                },
            ],
        },
        QuestionData {
            id: 4,
            question: "What yeast should I use for Chardonnay?".to_string(),
            tags: vec![
                Tag {
                    id: 1,
                    value: "chardonnay".to_string(),
                },
                Tag {
                    id: 4,
                    value: "fermentation".to_string(),
                },
            ],
        },
        QuestionData {
            id: 5,
            question: "Which experiments did you do during the 2026 harvest?".to_string(),
            tags: vec![Tag {
                id: 5,
                value: "2026 harvest".to_string(),
            }],
        },
    ]
}
