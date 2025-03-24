use leptos::{ev, html::*, prelude::*};

/// A parameterized incrementing button
#[component]
pub fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count) = signal(0);
    button()
        .on(ev::click, move |_| set_count.set(count.get() + increment))
        .child(("Click me: ", count))
}
