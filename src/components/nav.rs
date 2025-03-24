use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <div class="flex justify-center items-center p-4 h-16 bg-slate-900">
            <A href="/">"Parshulp"</A>
        </div>
    }
}
