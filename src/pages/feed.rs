use leptos::prelude::*;
use leptos_meta::{Stylesheet, Title};

#[component]
pub fn FeedPage() -> impl IntoView {

    let (name, set_name) = signal("Kind".to_string());

    let is_valid = move || {
        let n = name.get().to_lowercase();
        let bad_words = ["doof", "blöd", "mist", "gurke"];
        n.len() > 2 && !bad_words.iter().any(|word| n.contains(word))
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/app.css"/>
        <Title text="SafeSocial Kids - Feed"/>

        <div class="min-h-screen bg-gradient-to-br from-yellow-100 via-orange-50 to-pink-100 p-6 md:p-12 font-sans text-gray-800">
            <div class="max-w-md mx-auto bg-white/80 backdrop-blur-md rounded-[3rem] shadow-2xl p-10 border-4 border-white">

                <h1 class="text-4xl font-black text-orange-500 mb-8 text-center italic tracking-tight">
                    "Hallo, " {move || name.get()} "!"
                </h1>

                <div class="space-y-8">
                    <div class="relative">
                        <label class="block text-xs font-black text-orange-300 mb-2 uppercase tracking-widest ml-4">
                            "Wer bist du?"
                        </label>
                        <input
                            type="text"
                            placeholder="Dein Name..."
                            class="w-full px-6 py-4 bg-white border-4 rounded-full outline-none transition-all duration-300 text-lg font-bold shadow-inner"
                            class:border-red-400=move || !is_valid()
                            class:text-red-500=move || !is_valid()
                            class:border-green-400=move || is_valid()
                            class:text-green-600=move || is_valid()

                            on:input=move |ev| {
                                set_name.set(event_target_value(&ev));
                            }
                            prop:value=name
                        />
                    </div>

                    <div class="transform transition-all duration-500 hover:scale-105">
                        {move || {
                            if is_valid() {
                                view! {
                                    <div class="bg-green-400 text-white p-4 rounded-3xl text-center font-black shadow-lg">
                                        "✨ VOLL COOLER NAME!"
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="bg-red-400 text-white p-4 rounded-3xl text-center font-black shadow-lg animate-bounce">
                                        "🛑 STOPP! SO NICHT."
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>

                    <button
                        class="w-full py-5 rounded-full font-black text-xl text-white shadow-xl transform active:scale-95 transition-all"
                        class:bg-orange-500=move || is_valid()
                        class:hover:bg-orange-600=move || is_valid()
                        class:bg-gray-300=move || !is_valid()
                        class:cursor-not-allowed=move || !is_valid()
                    >
                        "POSTEN! 🚀"
                    </button>
                </div>
            </div>
        </div>
    }
}