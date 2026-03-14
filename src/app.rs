use crate::models::AppState;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let state = signal(AppState::Landing);
    provide_context(state);

    view! {
        <Stylesheet id="leptos" href="/pkg/app.css"/>
        <Title text="SafeSocial Kids"/>

        <main>
            {move || match state.0.get() {
                AppState::Landing => view! { <div>"Hier kommt die Landing Page hin"</div> }.into_any(),
                AppState::ParentalCheck => view! { <div>"Hier kommt der Eltern-Check hin"</div> }.into_any(),
                AppState::MainFeed => view! { <div>"Hier kommt dein Feed hin"</div> }.into_any(),
            }}
        </main>
    }
}