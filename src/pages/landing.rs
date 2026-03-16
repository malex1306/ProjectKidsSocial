use leptos::prelude::*;
use leptos_fluent::tr;
use crate::models::AppState;

#[component]
pub fn LandingPage() -> impl IntoView {
    let set_state = use_context::<WriteSignal<AppState>>()
        .expect("AppState sollte im Context sein");

    view! {
        <div class="relative min-h-screen bg-deep-black font-body text-white overflow-x-hidden selection:bg-neon-lime selection:text-black uppercase">
            // Clean scanlines instead of noise
            <div class="scanlines opacity-20"></div>
            
            // Edgy Navigation
            <nav class="fixed top-0 w-full z-100 bg-deep-black/95 border-b-8 border-black px-6 py-6 lg:py-8">
                <div class="max-w-7xl mx-auto flex justify-between items-center">
                    <div class="flex items-center gap-3 px-6 py-2 bg-neon-lime text-black border-neo sticker-tilt-l hover:rotate-0 transition-transform cursor-pointer">
                        <span class="text-4xl font-black font-heading leading-none">"S"</span>
                        <div class="text-2xl font-black tracking-tighter">
                            {move || tr!("landing-title")}
                        </div>
                    </div>
                    <div class="flex items-center gap-6">
                        <button 
                            on:click=move |_| set_state.set(AppState::Login)
                            class="hidden md::block font-bold text-sm text-slate-400 hover:text-white transition-colors"
                        >
                            {move || tr!("landing-login-button")}
                        </button>
                        <button 
                            on:click=move |_| set_state.set(AppState::Register)
                            class="px-6 py-2.5 rounded-xl font-black text-sm bg-white text-slate-950 hover:bg-cyan-400 transition-all shadow-xl active:scale-95"
                        >
                            {move || tr!("landing-register-button")}
                        </button>
                    </div>
                </div>
            </nav>

            // Hero Section: Aggressive & Raw
            <section class="relative pt-60 pb-32 px-6 min-h-screen flex items-center">
                // Background Doodles
                <div class="absolute inset-0 pointer-events-none opacity-10">
                    <div class="absolute top-1/4 left-10 text-9xl font-heading text-outline rotate-12">"SOCIAL"</div>
                    <div class="absolute bottom-1/4 right-10 text-9xl font-heading text-outline -rotate-12">"RULES"</div>
                </div>

                <div class="max-w-7xl mx-auto w-full grid lg:grid-cols-12 gap-16 items-center">
                    
                    // Text Content
                    <div class="lg:col-span-7 space-y-12 text-center lg:text-left relative z-20">
                        <div class="inline-flex items-center gap-3 px-6 py-2 bg-slate-900 border-neo-lime font-mono text-xs font-black tracking-[0.3em] sticker-tilt-l">
                            <span class="w-3 h-3 bg-neon-lime animate-pulse"></span>
                            "STATUS: 4.2K_TEENS_ONLINE"
                        </div>

                        <div class="relative space-y-4">
                            <h1 class="text-7xl md:text-[9rem] font-black leading-[0.85] font-heading text-white uppercase select-none">
                                <span class="block hover:animate-glitch">"DEIN"</span>
                                <span class="block text-neon-lime italic -ml-2 filter drop-shadow-[6px_6px_0px_#000]">"SPACE."</span>
                            </h1>
                            
                            <h1 class="text-6xl md:text-[8rem] font-black leading-[0.85] font-heading text-white uppercase select-none pt-4">
                                <span class="block hover:animate-glitch">"DEINE"</span>
                                <div class="inline-block bg-hot-pink text-white px-6 py-2 mt-2 sticker-tilt-r border-neo shadow-[6px_6px_0px_#000] text-5xl md:text-8xl">
                                    "REGELN."
                                </div>
                            </h1>
                        </div>

                        <div class="relative">
                            <p class="text-xl md:text-3xl text-white font-black leading-tight tracking-tight max-w-xl mx-auto lg:mx-0 p-4 bg-neon-lime text-black border-neo sticker-tilt-l inline-block">
                                {move || tr!("landing-hero-subtitle")}
                            </p>
                        </div>

                        <div class="flex flex-col sm:flex-row gap-8 justify-center lg:justify-start pt-12">
                            <button 
                                on:click=move |_| set_state.set(AppState::Register)
                                class="group relative inline-block px-12 py-8 bg-neon-lime text-black font-black text-3xl uppercase border-neo-lime hover:translate-x-2 hover:translate-y-2 hover:shadow-none transition-all"
                            >
                                <span class="relative z-10">{move || tr!("landing-start-button")}</span>
                                <div class="absolute inset-0 bg-hot-pink translate-x-4 translate-y-4 -z-10 group-hover:translate-x-0 group-hover:translate-y-0 transition-transform"></div>
                            </button>
                            
                            <button 
                                on:click=move |_| set_state.set(AppState::Login)
                                class="inline-block px-12 py-8 bg-black text-white font-black text-3xl uppercase border-neo hover:bg-slate-900 transition-all"
                            >
                                {move || tr!("landing-login-button")}
                            </button>
                        </div>
                    </div>

                    // Hero Visual: Enhanced Sticker-Bomb
                    <div class="lg:col-span-5 relative mt-20 lg:mt-0">
                        <div class="relative w-full aspect-square md:max-w-lg mx-auto">
                            // Layers of stickers for depth
                            <div class="absolute -top-16 -right-16 w-36 h-36 bg-hot-pink border-4 border-black font-black flex items-center justify-center text-5xl rotate-[25deg] z-0 hover:rotate-0 transition-transform">"💥"</div>
                            <div class="absolute -top-10 left-10 w-28 h-28 bg-white text-black border-4 border-black font-black flex items-center justify-center text-xl -rotate-12 z-0">"LEGIT"</div>
                            <div class="absolute top-1/2 -left-20 w-44 h-20 bg-electric-blue text-black border-4 border-black font-black flex items-center justify-center text-sm -rotate-90 z-0">"LIMITED EDITION"</div>
                            
                            // Main Image Container
                            <div class="relative h-full w-full bg-slate-900 border-8 border-black p-4 z-10 shadow-[20px_20px_0px_rgba(0,0,0,1)]">
                                 <img 
                                    src="/C:/Users/alexa/.gemini/antigravity/brain/8894be1d-3a34-43a1-8ca2-de2689ce3872/hero_neo_punk_sticker_bomb_1773678799647.png" 
                                    alt="Neo Punk Hero" 
                                    class="w-full h-full object-cover grayscale brightness-125 hover:grayscale-0 transition-all duration-700"
                                />
                                // Overlays on image
                                <div class="absolute top-10 left-10 border-4 border-neon-lime px-4 py-1 text-neon-lime font-black text-xs sticker-tilt-l">"REC"</div>
                            </div>

                            // Verified Sticker
                            <div class="absolute -bottom-12 -right-10 bg-neon-lime border-8 border-black text-black p-8 z-30 sticker-tilt-r shadow-[10px_10px_0px_#000]">
                                <div class="text-3xl font-black leading-none mb-1">"🛡️ SAFE"</div>
                                <div class="text-xs font-mono font-black border-t-2 border-black pt-2">"NO ADULTS_ALLOWED"</div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Feature Grid: Massive & Raw
            <section class="py-40 px-6 border-y-8 border-black bg-white/[0.02] relative">
                <div class="max-w-7xl mx-auto">
                    <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-16">
                        // Feature 1
                        <div class="bg-deep-black border-8 border-black p-12 group hover:translate-x-2 hover:translate-y-2 hover:shadow-none transition-all shadow-[12px_12px_0px_#afff00]">
                            <div class="w-24 h-24 bg-white text-black flex items-center justify-center text-5xl border-8 border-black mb-10 group-hover:bg-hot-pink group-hover:text-white transition-all sticker-tilt-l">"🔒"</div>
                            <h3 class="text-5xl font-black mb-6 tracking-tighter">
                                {move || tr!("landing-feature-1-title")}
                            </h3>
                            <p class="text-slate-400 font-black text-xl leading-snug">
                                {move || tr!("landing-feature-1-desc")}
                            </p>
                        </div>

                        // Feature 2: High Saturation Break
                        <div class="bg-hot-pink border-8 border-black p-12 group hover:translate-x-2 hover:translate-y-2 hover:shadow-none transition-all shadow-[12px_12px_0px_#000] sticker-tilt-r">
                            <div class="w-24 h-24 bg-black text-white flex items-center justify-center text-5xl border-8 border-white mb-10 group-hover:bg-neon-lime group-hover:text-black transition-all">"⚡"</div>
                            <h3 class="text-5xl font-black mb-6 tracking-tighter text-white">
                                {move || tr!("landing-feature-2-title")}
                            </h3>
                            <p class="text-white font-black text-xl leading-snug">
                                {move || tr!("landing-feature-2-desc")}
                            </p>
                        </div>

                        // Feature 3
                        <div class="bg-deep-black border-8 border-black p-12 group hover:translate-x-2 hover:translate-y-2 hover:shadow-none transition-all shadow-[12px_12px_0px_#00f0ff] sm:col-span-2 lg:col-span-1">
                            <div class="w-24 h-24 bg-electric-blue text-black flex items-center justify-center text-5xl border-8 border-black mb-10 group-hover:rotate-180 transition-all">"🎮"</div>
                            <h3 class="text-5xl font-black mb-6 tracking-tighter">
                                {move || tr!("landing-feature-3-title")}
                            </h3>
                            <p class="text-slate-400 font-black text-xl leading-snug">
                                {move || tr!("landing-feature-3-desc")}
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Footer
            <footer class="py-32 px-6 bg-black text-white relative">
                <div class="max-w-7xl mx-auto flex flex-col items-center text-center gap-16">
                    <div class="text-8xl md:text-[12rem] font-black font-heading tracking-tighter text-neon-lime sticker-tilt-l hover:rotate-0 transition-transform select-none">
                         "SAFESOCIAL"
                    </div>
                    <div class="flex flex-wrap justify-center gap-12 font-black text-sm tracking-[0.3em]">
                        <a href="#" class="hover:text-hot-pink hover:line-through transition-all">"DATENSCHUTZ"</a>
                        <a href="#" class="hover:text-hot-pink hover:line-through transition-all">"IMPRESSUM"</a>
                        <a href="#" class="hover:text-hot-pink hover:line-through transition-all">"HILFE"</a>
                    </div>
                    <div class="text-slate-600 font-mono text-xs border-neo-lime px-6 py-2">
                        {move || tr!("app-footer")}
                    </div>
                </div>
            </footer>
        </div>
    }
}