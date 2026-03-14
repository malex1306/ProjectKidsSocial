use leptos::prelude::*;
use crate::models::AppState;

#[component]
pub fn LandingPage() -> impl IntoView {
    let set_state = use_context::<WriteSignal<AppState>>()
        .expect("AppState sollte im Context sein");

    view! {
        <div class="min-h-screen flex items-center justify-center bg-blue-500">
            <div class="text-center bg-white p-10 rounded-3xl shadow-2xl">
                <h1 class="text-2xl font-bold mb-4">"Willkommen bei SafeSocial!"</h1>
                <button
                    class="bg-orange-500 text-white px-8 py-3 rounded-full font-bold"
                    on:click=move |_| set_state.set(AppState::MainFeed)
                >
                    "Jetzt starten"
                </button>
            </div>
        </div>
    }
}