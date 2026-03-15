use crate::models::AppState;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_fluent::{leptos_fluent, tr};

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

    leptos_fluent! {
        children: view! {
                <Stylesheet id="leptos" href="/pkg/app.css"/>
                <Title text=move || tr!("app-title")/>

                <main>
                    {move || match state.get() {
                        AppState::Landing => view! { <crate::pages::landing::LandingPage /> }.into_any(),
                        AppState::ParentalCheck => view! {
                            <div class="min-h-screen flex items-center justify-center bg-blue-50">
                                <div class="bg-white p-8 rounded-3xl shadow-xl text-center border-4 border-blue-200">
                                    <h1 class="text-2xl font-black text-blue-600 mb-4">{move || tr!("parental-check-title")}</h1>
                                    <p class="text-gray-600 mb-6">{move || tr!("parental-check-description")}</p>
                                    <button
                                        class="bg-blue-500 hover:bg-blue-600 text-white font-bold py-3 px-8 rounded-full transition-transform active:scale-95"
                                        on:click=move |_| set_state.set(AppState::MainFeed)
                                    >
                                        {move || tr!("parental-check-button")}
                                    </button>
                                </div>
                            </div>
                        }.into_any(),
                        AppState::MainFeed => view! { <crate::pages::feed::FeedPage /> }.into_any(),
                    }}
                </main>
        },
        locales: "./locales",
        default_language: "de",
    }
}