use leptos::prelude::*;
use leptos_router::components::A;
use reactive_stores::Store;
use thaw::*;

#[derive(Store, Debug, Clone)]
pub struct Data {
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

#[component]
pub fn Questions() -> impl IntoView {
    let questions = Store::new(Data {
        rows: vec![
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
                        id: 1,
                        value: "cabernet sauvignon".to_string(),
                    },
                    Tag {
                        id: 1,
                        value: "viticulture".to_string(),
                    },
                ],
            },
            QuestionData {
                id: 3,
                question: "What yeast should I use for Cabernet Sauvignon?".to_string(),
                tags: vec![
                    Tag {
                        id: 1,
                        value: "cabernet sauvignon".to_string(),
                    },
                    Tag {
                        id: 1,
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
                        id: 1,
                        value: "fermentation".to_string(),
                    },
                ],
            },
            QuestionData {
                id: 5,
                question: "Which experiments did you do during the 2026 harvest?".to_string(),
                tags: vec![Tag {
                    id: 1,
                    value: "2026 harvest".to_string(),
                }],
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
                            <Card class="w-full">
                                <CardHeader>
                                    <Body1>
                                        <A href=move || {
                                            format!("/{}", id.get())
                                        }>{move || question.get()}</A>
                                    </Body1>
                                    <CardHeaderDescription slot>
                                        <Caption1>"Added by Renee_vn"</Caption1>
                                    </CardHeaderDescription>
                                    <CardHeaderAction slot>
                                        <Menu
                                            on_select=move |value| leptos::logging::warn!("{}", value)
                                            position=MenuPosition::LeftStart
                                        >
                                            <MenuTrigger slot>
                                                <Button
                                                    appearance=ButtonAppearance::Transparent
                                                    icon=icondata::AiMoreOutlined
                                                />
                                            </MenuTrigger>
                                            <MenuItem value="answer">"Answer Question"</MenuItem>
                                            <MenuItem value="report" class="text-red-500">
                                                "Report"
                                            </MenuItem>
                                        </Menu>
                                    </CardHeaderAction>
                                </CardHeader>
                                <CardFooter>
                                    <Flex justify=FlexJustify::SpaceBetween class="w-full">
                                        <TagGroup>
                                            <For
                                                each=move || tags.get()
                                                key=|tag| tag.id
                                                children=move |tag: Tag| {
                                                    view! {
                                                        <InteractionTag size=TagSize::Small>
                                                            <InteractionTagPrimary>
                                                                {move || tag.clone().value}
                                                            </InteractionTagPrimary>
                                                        </InteractionTag>
                                                    }
                                                }
                                            ></For>
                                        </TagGroup>
                                        <ButtonGroup vertical=true>
                                            <Button
                                                icon=icondata::AiHeartOutlined
                                                size=ButtonSize::Small
                                            >
                                                "23"
                                            </Button>
                                            <Button
                                                icon=icondata::AiBookOutlined
                                                size=ButtonSize::Small
                                            >
                                                "4"
                                            </Button>
                                        </ButtonGroup>
                                    </Flex>
                                </CardFooter>
                            </Card>
                        }
                    }
                />
            </div>
        </ErrorBoundary>
    }
}
