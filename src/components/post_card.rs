use leptos::prelude::*;

#[component]
pub fn PostCard(
    children: Children
) -> impl IntoView {
    view! {
        <div class="
            w-[95%] max-w-sm p-6
            md:max-w-md md:p-10
            mx-auto bg-white/80 backdrop-blur-md
            rounded-[2.5rem] md:rounded-[3.5rem]
            shadow-2xl border-4 border-white
            transition-all duration-300 hover:scale-[1.01]
        ">
            {children()}
        </div>
    }
}