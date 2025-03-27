use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};
use thaw::*;

#[derive(Params, PartialEq)]
struct ProfileParams {
    username: Option<String>,
}

#[component]
pub fn Profile() -> impl IntoView {
    let params = use_params::<ProfileParams>();
    let username = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.username.clone())
            .unwrap_or_default()
    };
    view! {
            <div class="flex flex-col justfiy-center items-center pt-8 gap-5">
                <div class="inline-block relative">
                    <img
                        class="h-32 w-32 rounded-full"
                        src="https://cataas.com/cat/orange?type=square"
                        alt="Pfp"
                    />
                    <div class="text-center absolute -bottom-4 inset-x-0">
    <Tooltip content="This is your reputation score. Contribute to earn more!">
                        <span class="text-3xl font-bold shadow-lg bg-rose-800 rounded-full w-fit px-2 border-4 border-black">
                            50
                        </span>
                        </Tooltip>
                    </div>
                </div>
                <div class="text-center">
                    <h1 class="text-xl font-light">{move || username()}</h1>
                    <h2 class="text-slate-500">"Renée van Niekerk"</h2>
                </div>
            </div>
        }
}
