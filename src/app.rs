use crate::models::AppState;
use crate::pages::feed::FeedPage;
use crate::pages::landing::LandingPage;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title, MetaTags};
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="de">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (state, set_state) = signal(AppState::Landing);
    provide_context(state);
    provide_context(set_state);

    view! {
        <Stylesheet id="leptos" href="/pkg/app.css"/>
        <Title text="SafeSocial Kids"/>

        <main>
            {move || match state.get() {
                AppState::Landing => view! { <LandingPage /> }.into_any(),
                AppState::ParentalCheck => view! {
                    <div class="min-h-screen flex items-center justify-center bg-blue-50">
                        <div class="bg-white p-8 rounded-3xl shadow-xl text-center border-4 border-blue-200">
                            <h1 class="text-2xl font-black text-blue-600 mb-4">"Eltern-Check"</h1>
                            <p class="text-gray-600 mb-6">"Nur für Erwachsene! Bitte bestätigen."</p>
                            <button
                                class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-3 px-8 rounded-full transition-transform active:scale-95"
                                on:click=move |_| set_state.set(AppState::MainFeed)
                            >
                                "Erfolg simulieren"
                            </button>
                        </div>
                    </div>
                }.into_any(),
                AppState::MainFeed => view! { <FeedPage /> }.into_any(),
            }}
        </main>
    }
}