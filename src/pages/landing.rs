use leptos::prelude::*;
use crate::models::AppState;
use crate::components::post_card::PostCard;

#[component]
pub fn LandingPage() -> impl IntoView {
    let set_state = use_context::<WriteSignal<AppState>>()
        .expect("AppState sollte im Context sein");

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-yellow-100 via-orange-50 to-pink-100 p-4 md:p-12">
            <PostCard>
                <div class="text-center">
                    <h1 class="text-4xl font-black text-orange-500 mb-8 italic tracking-tight">
                        "SafeSocial"
                    </h1>

                    <p class="text-gray-600 mb-8 font-medium">
                        "Der sicherste Ort für dich und deine Freunde."
                    </p>

                    <button
                        class="w-full py-5 bg-orange-500 hover:bg-orange-600 text-white rounded-full font-black text-xl shadow-xl transform active:scale-95 transition-all"
                        on:click=move |_| set_state.set(AppState::MainFeed)
                    >
                        "LOS GEHT'S! 🚀"
                    </button>
                </div>
            </PostCard>
        </div>
    }
}