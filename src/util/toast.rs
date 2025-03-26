use leptos::prelude::*;
use thaw::*;

pub struct ToastParams {
    pub title: String,
    pub body: String,
}

pub fn dispatch_toast(params: ToastParams, intent: ToastIntent, toaster: ToasterInjection) {
    toaster.dispatch_toast(
        move || {
            view! {
                <Toast>
                    <ToastTitle>{params.title}</ToastTitle>
                    <ToastBody>{params.body}</ToastBody>
                </Toast>
            }
        },
        ToastOptions::default()
            .with_intent(intent)
            .with_position(ToastPosition::TopEnd),
    )
}
