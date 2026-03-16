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
                <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
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
                <Title text=move || tr!("app-title")/>

                <main>
                    {move || match state.get() {
                        AppState::Landing => view! { <crate::pages::landing::LandingPage /> }.into_any(),
                        AppState::Login => view! { <crate::pages::auth::LoginPage /> }.into_any(),
                        AppState::Register => view! { <crate::pages::auth::RegisterPage /> }.into_any(),
                        AppState::ParentalCheck => view! { <crate::pages::auth::ParentalCheckPage /> }.into_any(),
                        AppState::MainFeed => view! { <crate::pages::feed::FeedPage /> }.into_any(),
                    }}
                </main>
        },
        locales: "./locales",
        default_language: "de",
    }
}