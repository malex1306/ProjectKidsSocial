use leptos::prelude::*;
use leptos_fluent::tr;
use crate::models::AppState;

#[component]
pub fn LoginPage() -> impl IntoView {
    let set_state = use_context::<WriteSignal<AppState>>()
        .expect("AppState sollte im Context sein");

    view! {
        <div class="relative min-h-screen bg-deep-black font-body text-white flex items-center justify-center px-6">
            <div class="scanlines opacity-20"></div>
            
            <div class="relative z-10 w-full max-w-md">
                <div class="bg-deep-black border-neo-lime p-10 sticker-tilt-l shadow-[20px_20px_0px_#000]">
                    <div class="mb-10 text-center">
                        <h2 class="text-5xl font-black font-heading text-neon-lime mb-2 tracking-tighter uppercase">
                            {move || tr!("auth-login-title")}
                        </h2>
                        <div class="h-2 w-full bg-hot-pink border-4 border-black mb-6"></div>
                    </div>

                    <div class="space-y-8">
                        <div>
                            <label class="block font-mono text-xs font-black mb-3 text-slate-400">
                                {move || tr!("auth-username")}
                            </label>
                            <input 
                                type="text" 
                                class="w-full bg-slate-900 border-4 border-black p-4 text-white font-black text-xl focus:border-neon-lime focus:outline-none transition-all"
                                placeholder="..."
                            />
                        </div>
                        <div>
                            <label class="block font-mono text-xs font-black mb-3 text-slate-400">
                                {move || tr!("auth-password")}
                            </label>
                            <input 
                                type="password" 
                                class="w-full bg-slate-900 border-4 border-black p-4 text-white font-black text-xl focus:border-neon-lime focus:outline-none transition-all"
                                placeholder="..."
                            />
                        </div>

                        <button 
                            on:click=move |_| set_state.set(AppState::MainFeed)
                            class="w-full py-6 bg-neon-lime text-black font-black text-2xl uppercase border-neo hover:translate-x-1 hover:translate-y-1 hover:shadow-none transition-all"
                        >
                            {move || tr!("auth-login-submit")}
                        </button>
                    </div>

                    <div class="mt-10 text-center border-t-4 border-black pt-6">
                        <button 
                            on:click=move |_| set_state.set(AppState::Register)
                            class="text-xs font-black text-slate-500 hover:text-white transition-colors"
                        >
                            {move || tr!("auth-no-account")} " -> JOIN THE CREW"
                        </button>
                    </div>
                </div>

                // Decorative Sticker
                <div class="absolute -top-10 -right-10 bg-hot-pink border-4 border-black p-4 rotate-12 font-black text-2xl animate-glitch">
                    "GG WP!"
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let set_state = use_context::<WriteSignal<AppState>>()
        .expect("AppState sollte im Context sein");

    view! {
        <div class="relative min-h-screen bg-deep-black font-body text-white flex items-center justify-center px-6">
            <div class="scanlines opacity-20"></div>
            
            <div class="relative z-10 w-full max-w-lg">
                <div class="bg-deep-black border-neo-pink p-12 sticker-tilt-r shadow-[25px_25px_0px_#afff00]">
                    <div class="mb-10 text-center">
                        <h2 class="text-6xl font-black font-heading text-white mb-2 tracking-tighter uppercase italic">
                            {move || tr!("auth-register-title")}
                        </h2>
                        <div class="h-4 bg-neon-lime border-4 border-black -rotate-1"></div>
                    </div>

                    <div class="space-y-6">
                        <div class="grid grid-cols-2 gap-4">
                            <div>
                                <label class="block font-mono text-[10px] font-black mb-2 text-slate-500 text-left">
                                    {move || tr!("auth-username")}
                                </label>
                                <input 
                                    type="text" 
                                    class="w-full bg-slate-900 border-4 border-black p-4 text-white font-black text-sm focus:border-hot-pink focus:outline-none"
                                />
                            </div>
                            <div>
                                <label class="block font-mono text-[10px] font-black mb-2 text-slate-500 text-left">
                                    "ALTER_"
                                </label>
                                <input 
                                    type="number" 
                                    class="w-full bg-slate-900 border-4 border-black p-4 text-white font-black text-sm focus:border-hot-pink focus:outline-none"
                                />
                            </div>
                        </div>

                        <div>
                            <label class="block font-mono text-[10px] font-black mb-2 text-slate-500 text-left">
                                {move || tr!("auth-email")}
                            </label>
                            <input 
                                type="email" 
                                class="w-full bg-slate-900 border-4 border-black p-4 text-white font-black text-sm focus:border-hot-pink focus:outline-none"
                                placeholder="..."
                            />
                        </div>

                        <div>
                            <label class="block font-mono text-[10px] font-black mb-2 text-slate-500 text-left">
                                {move || tr!("auth-password")}
                            </label>
                            <input 
                                type="password" 
                                class="w-full bg-slate-900 border-4 border-black p-4 text-white font-black text-sm focus:border-hot-pink focus:outline-none"
                            />
                        </div>

                        <div class="bg-yellow-400 text-black border-4 border-black p-4 font-black text-[10px] tracking-tight flex items-center gap-4">
                            <span class="text-2xl">"🛡️"</span>
                            <span>"INFO: DEINE ELTERN ERHALTEN EINE BESTÄTIGUNGS-MAIL."</span>
                        </div>

                        <button 
                            on:click=move |_| set_state.set(AppState::ParentalCheck)
                            class="w-full py-8 bg-hot-pink text-white font-black text-3xl uppercase border-neo hover:translate-x-2 hover:translate-y-2 hover:shadow-none transition-all shadow-[8px_8px_0px_#000]"
                        >
                            {move || tr!("auth-register-submit")}
                        </button>
                    </div>

                    <div class="mt-10 text-center">
                        <button 
                            on:click=move |_| set_state.set(AppState::Login)
                            class="text-xs font-black text-hot-pink underline decoration-4 underline-offset-4 hover:text-white transition-colors"
                        >
                            {move || tr!("auth-have-account")}
                        </button>
                    </div>
                </div>

                // Decorative Sticker
                <div class="absolute -bottom-10 -left-10 bg-neon-lime border-4 border-black p-6 -rotate-12 font-black text-black">
                   "CREW ACCESS_01"
                </div>
            </div>
        </div>
    }
}
#[component]
pub fn ParentalCheckPage() -> impl IntoView {
    let set_state = use_context::<WriteSignal<AppState>>()
        .expect("AppState sollte im Context sein");

    view! {
        <div class="relative min-h-screen bg-deep-black font-body text-white flex items-center justify-center px-6">
            <div class="scanlines opacity-20"></div>
            
            <div class="relative z-10 w-full max-w-xl">
                <div class="bg-deep-black border-neo-lime p-12 sticker-tilt-l shadow-[30px_30px_0px_#000]">
                    <div class="mb-10 flex items-center gap-6">
                       <div class="text-6xl animate-bounce">"⚠️"</div>
                       <h2 class="text-6xl font-black font-heading text-neon-lime mb-2 tracking-tighter uppercase italic">
                            {move || tr!("parental-check-title")}
                        </h2>
                    </div>

                    <p class="text-2xl font-black text-white leading-tight mb-12 bg-slate-900 p-6 border-l-8 border-neon-lime">
                        {move || tr!("parental-check-description")}
                    </p>

                    <div class="space-y-8">
                        <button 
                            on:click=move |_| set_state.set(AppState::MainFeed)
                            class="w-full py-10 bg-neon-lime text-black font-black text-4xl uppercase border-neo hover:translate-x-2 hover:translate-y-2 hover:shadow-none transition-all shadow-[12px_12px_0px_#000]"
                        >
                            {move || tr!("parental-check-button")}
                        </button>
                        
                        <button 
                            on:click=move |_| set_state.set(AppState::Landing)
                            class="w-full py-4 text-slate-500 font-black text-sm uppercase hover:text-white transition-colors"
                        >
                            "<- ZURÜCK ZUR STARTSEITE"
                        </button>
                    </div>
                </div>

                // Decorative Sticker
                <div class="absolute -top-12 -right-12 bg-electric-blue border-4 border-black p-6 rotate-12 font-black text-black">
                   "LEVEL_UP!"
                </div>
            </div>
        </div>
    }
}
