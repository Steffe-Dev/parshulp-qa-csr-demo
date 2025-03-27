use leptos::prelude::*;

use leptos::Params;
use leptos_router::components::A;
use leptos_router::hooks::use_query;
use leptos_router::{hooks::use_params, params::Params};

use crate::pages::questions::FilterSortQuery;
use crate::pages::questions_answers_data::{self, AnswerStoreFields, AnswerStoreStoreFields};
use crate::pages::questions_data::{self, TagStoreFields};
use crate::util::toast::{self, ToastParams};
use thaw::*;

#[derive(Params, PartialEq)]
struct QuestionParams {
    id: Option<u32>,
}

#[component]
pub fn QuestionAnswers() -> impl IntoView {
    let toaster = ToasterInjection::expect_context();
    let params = use_params::<QuestionParams>();
    let query = use_query::<FilterSortQuery>();

    let filter_by = move || {
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.filter_by.clone())
    };

    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or_default()
    };
    let question = questions_data::get_question_by(id());
    let answers = move || questions_answers_data::get_questions_filtered_by(id(), filter_by());

    view! {
        <Flex vertical=true gap=FlexGap::Large class="p-4">
            <h1 class="text-xl font-light">{question.clone().question}</h1>
            <Show when=move || filter_by().is_some()>
                <A href="">
                    <MessageBar>
                        <MessageBarBody>
                            <MessageBarTitle>
                                "Filtered on '"{move || filter_by().unwrap()}"'"
                            </MessageBarTitle>
                            "Tap to clear"
                        </MessageBarBody>
                    </MessageBar>
                </A>
            </Show>
            <For
                each=move || answers().rows()
                key=|row| row.read().id
                children=move |child| {
                    let answer = child.answer();
                    view! {
                        <Card class="w-full">
                            <CardHeader>
                                <Body1>{move || answer.get()}</Body1>
                                <CardHeaderDescription slot>
                                    <Caption1>"Answered by Renee_vn"</Caption1>
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
                                        <MenuItem value="report" class="text-red-500">
                                            "Report"
                                        </MenuItem>
                                    </Menu>
                                </CardHeaderAction>
                            </CardHeader>
                            <CardFooter>
                                <Flex justify=FlexJustify::SpaceBetween class="w-full">
                                    <TagGroup size=TagSize::Small>
                                        <For
                                            each=move || child.tags()
                                            key=|tag| tag.get().id
                                            children=move |tag| {
                                                let tag = tag.value();
                                                view! {
                                                    // Can I do something with this tag??
                                                    <A href=format!("?filter_by={}", tag.get())>
                                                        <InteractionTag>
                                                            <InteractionTagPrimary>
                                                                {move || tag.get()}
                                                            </InteractionTagPrimary>
                                                        </InteractionTag>
                                                    </A>
                                                }
                                            }
                                        ></For>
                                    </TagGroup>
                                    <ButtonGroup vertical=true>
                                        <Button
                                            icon=icondata::AiHeartOutlined
                                            size=ButtonSize::Small
                                            on_click=move |_| {
                                                toast::dispatch_toast(
                                                    ToastParams {
                                                        title: "Loved!".to_string(),
                                                        body: "We'll let Renee_vn know. 😉".to_string(),
                                                    },
                                                    ToastIntent::Success,
                                                    toaster.clone(),
                                                )
                                            }
                                        >
                                            "23"
                                        </Button>
                                        <Button
                                            icon=icondata::AiBookOutlined
                                            size=ButtonSize::Small
                                            on_click=move |_| {
                                                toast::dispatch_toast(
                                                    ToastParams {
                                                        title: "Bookmarked!".to_string(),
                                                        body: "We added this question to your 'Bookmarks' collection"
                                                            .to_string(),
                                                    },
                                                    ToastIntent::Success,
                                                    toaster.clone(),
                                                )
                                            }
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
            <Show when=move || answers().rows().get().is_empty()>
                <span class="italic">
                    "No answers yet. Please check back later, or expore other questions in the meantime."
                </span>
            </Show>
        </Flex>
    }
}
