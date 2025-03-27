use leptos::prelude::*;
use leptos_router::hooks::use_query;
use leptos_router::{components::A, params::Params};
use thaw::*;

use crate::pages::questions_data::{
    self, QuestionDataStoreFields, QuestionStoreStoreFields, TagStoreFields,
};
use crate::util::toast::{self, ToastParams};

#[derive(Params, PartialEq)]
pub struct FilterSortQuery {
    pub filter_by: Option<String>,
}

#[component]
pub fn Questions() -> impl IntoView {
    let toaster = ToasterInjection::expect_context();

    let query = use_query::<FilterSortQuery>();
    let filter_by = move || {
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.filter_by.clone())
    };

    let questions = move || questions_data::get_questions_filtered_by(filter_by());
    view! {
        <Flex vertical=true gap=FlexGap::Large class="p-4">
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
                each=move || questions().rows()
                key=|row| row.read().id.clone()
                children=move |child| {
                    let id = child.id();
                    let question = child.question();
                    view! {
                        <Card class="w-full">
                            <CardHeader>
                                <Body1>
                                    <A href=move || {
                                        format!("{}", id.get())
                                    }>{move || question.get()}</A>
                                </Body1>
                                <CardHeaderDescription slot>
                                    <Caption1>"Added by Renee_vn (50)"</Caption1>
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
                                        <MenuItem value="answer">
                                        <A href=move || format!("add_answer/{}", id.get())>
                                        "Answer Question"
                                        </A>
                                        </MenuItem>
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
        </Flex>
    }
}
