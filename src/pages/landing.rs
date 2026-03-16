use leptos::prelude::*;
use leptos_fluent::tr;
use crate::models::AppState;

#[component]
pub fn LandingPage() -> impl IntoView {
    let set_state = use_context::<WriteSignal<AppState>>()
        .expect("AppState sollte im Context sein");

    view! {
        <div class="relative min-h-screen bg-deep-black font-body text-white overflow-x-hidden selection:bg-neon-lime selection:text-black">
            // Raw Texture Layer
            <div class="noise-bg"></div>
            
            // Edgy Navigation
            <nav class="fixed top-0 w-full z-100 bg-deep-black/90 border-b-4 border-black px-6 py-4">
                <div class="max-w-7xl mx-auto flex justify-between items-center">
                    <div class="flex items-center gap-2 px-4 py-2 bg-neon-lime text-black border-neo sticker-tilt-l hover:rotate-0 transition-transform cursor-pointer">
                        <span class="text-3xl font-black font-heading leading-none">"S"</span>
                        <div class="text-xl font-black uppercase tracking-tighter">
                            {move || tr!("landing-title")}
                        </div>
                    </div>
                    <div class="flex items-center gap-4">
                        <button class="hidden sm:block font-black uppercase text-xs tracking-widest hover:text-neon-lime transition-colors">
                            {move || tr!("landing-login-button")}
                        </button>
                        <button class="px-6 py-3 bg-hot-pink text-white font-black uppercase text-xs border-neo hover:translate-x-1 hover:translate-y-1 hover:shadow-none transition-all">
                            {move || tr!("landing-register-button")}
                        </button>
                    </div>
                </div>
            </nav>

            // Hero Section: Asymmetrical & High Energy
            <section class="relative pt-40 pb-20 px-6 min-h-screen flex items-center">
                <div class="max-w-7xl mx-auto w-full grid lg:grid-cols-12 gap-12 items-center">
                    
                    // Text Column
                    <div class="lg:col-span-7 space-y-12 text-center lg:text-left relative z-20">
                        <div class="inline-block px-4 py-2 bg-black text-neon-lime border-neo-lime font-mono text-xs font-black uppercase tracking-[0.2em] transform -rotate-1">
                            "// 4.2K TEENS ONLINE_"
                        </div>

                        <div class="relative">
                            <h1 class="text-7xl md:text-9xl font-black leading-[0.85] tracking-tight uppercase font-heading text-white">
                                <span class="block hover:animate-glitch select-none">"DEIN"</span>
                                <span class="block text-neon-lime italic -ml-2 filter drop-shadow-[4px_4px_0px_#ff007a]">"SPACE."</span>
                                <span class="block bg-white text-black px-4 inline-block mt-2 sticker-tilt-r">"REGELN."</span>
                            </h1>
                        </div>

                        <p class="text-xl md:text-3xl text-slate-400 max-w-xl mx-auto lg:mx-0 font-bold leading-tight uppercase tracking-tight">
                            {move || tr!("landing-hero-subtitle")}
                        </p>

                        <div class="flex flex-col sm:flex-row gap-6 justify-center lg:justify-start pt-8">
                            <button 
                                on:click=move |_| set_state.set(AppState::MainFeed)
                                class="group relative inline-block px-12 py-8 bg-neon-lime text-black font-black text-3xl uppercase border-neo-lime hover:translate-x-2 hover:translate-y-2 hover:shadow-none transition-all"
                            >
                                <span class="relative z-10">{move || tr!("landing-start-button")}</span>
                                <div class="absolute inset-0 bg-hot-pink translate-x-3 translate-y-3 -z-10 group-hover:translate-x-0 group-hover:translate-y-0 transition-transform"></div>
                            </button>
                        </div>
                    </div>

                    // Visual Column: Sticker-Bomb Look
                    <div class="lg:col-span-5 relative mt-12 lg:mt-0">
                        <div class="relative w-full aspect-square md:max-w-md mx-auto">
                            // Stickers behind image
                            <div class="absolute -top-10 -right-10 w-32 h-32 bg-hot-pink border-4 border-black font-black flex items-center justify-center text-4xl rotate-12 z-0 animate-pulse">"🔥"</div>
                            <div class="absolute -bottom-10 -left-10 w-40 h-40 bg-electric-blue border-4 border-black font-black flex items-center justify-center text-2xl -rotate-12 z-0">"NEW ERA"</div>
                            
                            // Image Container
                            <div class="relative h-full w-full bg-slate-900 border-Neo thick border-black p-4 z-10">
                                 <img 
                                    src="/C:/Users/alexa/.gemini/antigravity/brain/8894be1d-3a34-43a1-8ca2-de2689ce3872/hero_neo_punk_sticker_bomb_1773678799647.png" 
                                    alt="Neo Punk Hero" 
                                    class="w-full h-full object-cover filter contrast-125 saturate-150"
                                />
                            </div>

                            // Floating Badge Sticker
                            <div class="absolute -bottom-8 right-0 bg-neon-lime border-neo text-black p-6 z-20 sticker-tilt-l">
                                <div class="text-xl font-black uppercase leading-none tracking-tighter">"🛡️ Echt sicher."</div>
                                <div class="text-[10px] font-mono font-black mt-2">"VERIZIED BY AI_MOD"</div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Feature Section: Neo-Brutalist Grid
            <section class="py-32 px-6 border-y-8 border-black bg-neon-lime/5">
                <div class="max-w-7xl mx-auto">
                    <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-12">
                        // Feature 1
                        <div class="bg-deep-black border-neo-lime p-10 group hover:-translate-y-2 transition-transform">
                            <div class="w-20 h-20 bg-white text-black flex items-center justify-center text-4xl border-neo mb-8 group-hover:bg-hot-pink group-hover:text-white transition-colors">"🔒"</div>
                            <h3 class="text-4xl font-black mb-4 uppercase tracking-tighter">
                                {move || tr!("landing-feature-1-title")}
                            </h3>
                            <p class="text-slate-400 font-bold text-lg leading-relaxed uppercase">
                                {move || tr!("landing-feature-1-desc")}
                            </p>
                        </div>

                        // Feature 2
                        <div class="bg-hot-pink border-neo p-10 group hover:-translate-y-2 transition-transform sticker-tilt-r">
                            <div class="w-20 h-20 bg-black text-white flex items-center justify-center text-4xl border-white-neo mb-8 group-hover:bg-neon-lime group-hover:text-black transition-colors">"⚡"</div>
                            <h3 class="text-4xl font-black mb-4 uppercase tracking-tighter text-white">
                                {move || tr!("landing-feature-2-title")}
                            </h3>
                            <p class="text-white/80 font-bold text-lg leading-relaxed uppercase">
                                {move || tr!("landing-feature-2-desc")}
                            </p>
                        </div>

                        // Feature 3
                        <div class="bg-deep-black border-neo-lime p-10 group hover:-translate-y-2 transition-transform sm:col-span-2 lg:col-span-1">
                            <div class="w-20 h-20 bg-neon-lime text-black flex items-center justify-center text-4xl border-neo mb-8 group-hover:rotate-12 transition-transform">"🎮"</div>
                            <h3 class="text-4xl font-black mb-4 uppercase tracking-tighter">
                                {move || tr!("landing-feature-3-title")}
                            </h3>
                            <p class="text-slate-400 font-bold text-lg leading-relaxed uppercase">
                                {move || tr!("landing-feature-3-desc")}
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Footer
            <footer class="py-20 px-6 bg-black text-white">
                <div class="max-w-7xl mx-auto flex flex-col items-center text-center gap-12">
                    <div class="text-6xl font-black font-heading tracking-tighter text-neon-lime sticker-tilt-l">
                         "SAFESOCIAL_"
                    </div>
                    <div class="flex flex-wrap justify-center gap-8 font-black uppercase text-xs tracking-widest">
                        <a href="#" class="hover:text-hot-pink transition-colors">"Datenschutz"</a>
                        <a href="#" class="hover:text-hot-pink transition-colors">"Impressum"</a>
                        <a href="#" class="hover:text-hot-pink transition-colors">"Hilfe"</a>
                    </div>
                    <div class="text-slate-500 font-mono text-xs uppercase">
                        {move || tr!("app-footer")}
                    </div>
                </div>
            </footer>
        </div>
    }
}