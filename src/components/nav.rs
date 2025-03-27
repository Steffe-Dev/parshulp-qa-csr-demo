use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <div class="flex justify-between items-center p-4 h-16 bg-rose-800/75">
            <A href="/parshulp-qa-csr-demo">
                <span class="text-2xl font-light">"Parshulp"</span>
            </A>
            <A href="/parshulp-qa-csr-demo">
            <span class="h-10 w-10 rounded-full bg-white flex justify-center items-center text-black">"RvN"</span>
            </A>
        </div>
    }
}
