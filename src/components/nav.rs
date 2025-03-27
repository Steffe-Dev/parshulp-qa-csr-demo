use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <div class="flex justify-between items-center p-4 h-16 bg-rose-800/75">
            <A href="/parshulp-qa-csr-demo">
                <span class="text-2xl font-light">"Parshulp"</span>
            </A>
            <A href="/parshulp-qa-csr-demo/profile/Renee_vn">
                <img
                    class="h-12 w-12 rounded-full border-4 border-black"
                    src="https://cataas.com/cat?type=square"
                    alt="Pfp"
                />
            </A>
        </div>
    }
}
