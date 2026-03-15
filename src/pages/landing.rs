use leptos::prelude::*;
use leptos_fluent::tr;
use crate::models::AppState;

#[component]
pub fn LandingPage() -> impl IntoView {
    let set_state = use_context::<WriteSignal<AppState>>()
        .expect("AppState sollte im Context sein");

    view! {
        <div class="relative min-h-screen bg-slate-950 font-sans text-white overflow-x-hidden selection:bg-cyan-500 selection:text-slate-950">
            // Decorative Layers
            <div class="noise-bg"></div>
            <div class="glow-mesh opacity-50"></div>
            
            // Branding & Navigation
            <nav class="fixed top-0 w-full z-100 bg-slate-950/80 backdrop-blur-xl border-b border-white/10 px-6 py-4">
                <div class="max-w-7xl mx-auto flex justify-between items-center">
                    <div class="flex items-center gap-3 group px-4 py-2 hover:bg-white/5 rounded-2xl transition-colors cursor-pointer">
                        <div class="w-10 h-10 bg-gradient-to-br from-indigo-600 to-pink-600 rounded-xl flex items-center justify-center shadow-lg transform rotate-3 group-hover:rotate-0 transition-transform duration-300">
                          <span class="text-xl font-black">"S"</span>
                        </div>
                        <div class="text-2xl font-black tracking-tighter text-white">
                            {move || tr!("landing-title")}
                        </div>
                    </div>
                    <div class="flex items-center gap-6">
                        <button class="hidden md:block font-bold text-sm text-slate-400 hover:text-white transition-colors">
                            {move || tr!("landing-login-button")}
                        </button>
                        <button class="px-6 py-2.5 rounded-xl font-black text-sm bg-white text-slate-950 hover:bg-cyan-400 transition-all shadow-xl active:scale-95">
                            {move || tr!("landing-register-button")}
                        </button>
                    </div>
                </div>
            </nav>

            // Hero Section
            <section class="relative pt-64 pb-24 px-6 min-h-[90vh] flex items-center">
                <div class="max-w-7xl mx-auto grid lg:grid-cols-2 gap-20 items-center">
                    <div class="space-y-12 text-center lg:text-left relative z-10">
                        <div class="inline-flex items-center gap-3 px-4 py-2 rounded-full bg-slate-900/50 border border-white/10 text-cyan-400 text-[11px] font-black tracking-widest uppercase backdrop-blur-md">
                            <span class="relative flex h-2 w-2">
                                <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-cyan-400 opacity-75"></span>
                                <span class="relative inline-flex rounded-full h-2 w-2 bg-cyan-400"></span>
                            </span>
                            "4.2K TEENS ONLINE"
                        </div>

                        <div class="space-y-4">
                            <h1 class="text-7xl md:text-9xl font-black leading-[0.9] tracking-tighter uppercase font-heading">
                                <span class="block text-white">"DEIN"</span>
                                <span class="text-transparent bg-clip-text bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500">"SPACE."</span>
                                <span class="block text-white underline decoration-indigo-600 decoration-[16px] underline-offset-[12px]">"REGELN."</span>
                            </h1>
                        </div>

                        <p class="text-xl md:text-2xl text-slate-400 max-w-xl mx-auto lg:mx-0 leading-relaxed font-medium">
                            {move || tr!("landing-hero-subtitle")}
                        </p>

                        <div class="flex flex-col sm:flex-row gap-6 justify-center lg:justify-start pt-8">
                            <button 
                                on:click=move |_| set_state.set(AppState::MainFeed)
                                class="w-full sm:w-auto px-12 py-6 bg-gradient-to-r from-indigo-600 to-pink-600 rounded-3xl font-black text-2xl hover:scale-105 active:scale-95 transition-all shadow-[0_20px_40px_-15px_rgba(79,70,229,0.3)]"
                            >
                                {move || tr!("landing-start-button")}
                            </button>
                        </div>
                    </div>

                    // Hero Visual
                    <div class="relative w-full aspect-square group">
                        <div class="absolute -inset-10 bg-gradient-to-r from-indigo-600 via-purple-600 to-pink-600 rounded-[4rem] blur-[80px] opacity-20 group-hover:opacity-40 transition-opacity duration-1000"></div>
                        <div class="relative h-full w-full rounded-[4rem] border border-white/10 overflow-hidden shadow-2xl bg-slate-900 group-hover:border-white/20 transition-colors">
                             <img 
                                src="/hero.png" 
                                alt="Hero Visual" 
                                class="w-full h-full object-cover transform group-hover:scale-110 transition-transform duration-[2000ms]"
                            />
                            <div class="absolute inset-0 bg-gradient-to-t from-slate-950 via-transparent to-transparent opacity-80"></div>
                        </div>
                        
                        // Floating Badge
                        <div class="absolute -bottom-10 -left-10 bg-slate-900/90 backdrop-blur-2xl border border-white/10 p-6 rounded-[2rem] shadow-2xl animate-bounce-slow">
                            <div class="flex items-center gap-3">
                                <div class="w-12 h-12 bg-cyan-500/20 rounded-2xl flex items-center justify-center text-2xl">"🛡️"</div>
                                <div>
                                    <div class="text-cyan-400 font-black italic text-lg leading-tight">"Echt sicher."</div>
                                    <div class="text-[11px] text-slate-500 mt-1 uppercase tracking-widest font-black">"Moderiert von KIs"</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Features Grid
            <section class="py-32 px-6 relative overflow-hidden bg-white/[0.02]">
                <div class="max-w-7xl mx-auto relative z-10">
                    <div class="grid md:grid-cols-3 gap-10">
                        // Feature 1
                        <div class="bg-slate-900/40 p-10 rounded-[3rem] border border-white/5 hover:border-indigo-500/50 transition-all duration-500 group relative overflow-hidden">
                            <div class="absolute top-0 right-0 w-32 h-32 bg-indigo-600/5 blur-3xl -mr-16 -mt-16 group-hover:bg-indigo-600/10 transition-colors"></div>
                            <div class="w-20 h-20 bg-indigo-600/10 rounded-[2rem] flex items-center justify-center mb-8 group-hover:scale-110 group-hover:rotate-3 transition-all">
                                <span class="text-4xl text-indigo-400">"🔒"</span>
                            </div>
                            <h3 class="text-3xl font-black mb-4 text-white uppercase tracking-tight">
                                {move || tr!("landing-feature-1-title")}
                            </h3>
                            <p class="text-slate-400 leading-relaxed text-lg">
                                {move || tr!("landing-feature-1-desc")}
                            </p>
                        </div>

                        // Feature 2
                        <div class="bg-slate-900/40 p-10 rounded-[3rem] border border-white/5 hover:border-pink-500/50 transition-all duration-500 group relative overflow-hidden">
                            <div class="absolute top-0 right-0 w-32 h-32 bg-pink-600/5 blur-3xl -mr-16 -mt-16 group-hover:bg-pink-600/10 transition-colors"></div>
                            <div class="w-20 h-20 bg-pink-600/10 rounded-[2rem] flex items-center justify-center mb-8 group-hover:scale-110 group-hover:rotate-3 transition-all">
                                <span class="text-4xl text-pink-400">"⚡"</span>
                            </div>
                            <h3 class="text-3xl font-black mb-4 text-white uppercase tracking-tight">
                                {move || tr!("landing-feature-2-title")}
                            </h3>
                            <p class="text-slate-400 leading-relaxed text-lg">
                                {move || tr!("landing-feature-2-desc")}
                            </p>
                        </div>

                        // Feature 3
                        <div class="bg-slate-900/40 p-10 rounded-[3rem] border border-white/5 hover:border-cyan-500/50 transition-all duration-500 group relative overflow-hidden">
                            <div class="absolute top-0 right-0 w-32 h-32 bg-cyan-600/5 blur-3xl -mr-16 -mt-16 group-hover:bg-cyan-600/10 transition-colors"></div>
                            <div class="w-20 h-20 bg-cyan-600/10 rounded-[2rem] flex items-center justify-center mb-8 group-hover:scale-110 group-hover:rotate-3 transition-all">
                                <span class="text-4xl text-cyan-400">"🎮"</span>
                            </div>
                            <h3 class="text-3xl font-black mb-4 text-white uppercase tracking-tight">
                                {move || tr!("landing-feature-3-title")}
                            </h3>
                            <p class="text-slate-400 leading-relaxed text-lg">
                                {move || tr!("landing-feature-3-desc")}
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Footer
            <footer class="py-24 px-6 border-t border-white/5 bg-slate-950">
                <div class="max-w-7xl mx-auto flex flex-col md:flex-row justify-between items-center gap-12">
                    <div class="flex items-center gap-4">
                        <div class="w-8 h-8 bg-white/10 rounded-lg flex items-center justify-center font-black">"S"</div>
                        <div class="text-2xl font-black tracking-tighter">"SafeSocial Kids"</div>
                    </div>
                    <div class="flex gap-10 text-slate-500 font-black uppercase text-[12px] tracking-[0.2em]">
                        <a href="#" class="hover:text-white transition-all">"Datenschutz"</a>
                        <a href="#" class="hover:text-white transition-all">"Impressum"</a>
                        <a href="#" class="hover:text-white transition-all">"Hilfe"</a>
                    </div>
                    <div class="text-slate-600 text-sm font-bold bg-white/5 px-4 py-2 rounded-full">
                        {move || tr!("app-footer")}
                    </div>
                </div>
            </footer>
        </div>
    }
}