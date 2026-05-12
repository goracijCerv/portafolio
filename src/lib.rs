use leptos::prelude::*;


pub mod components;
pub mod utils;

 use components::{
    loading::Loading,
    navigation::Navigation,
    hero::Hero,
    terminal::Terminal,
    skills::Skils,
    projects::Projects
 };

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Loading />
        <BackGround />
        <Navigation />
        <main>
            <Hero />
            <Marquee />
            <Terminal/>
            <Skils/>
            <Projects />
        </main>
    }
}

#[component] 
fn BackGround() -> impl IntoView {
    view! {
        <div class="fixed inset-0 pointer-events-none z-0 overflow-hidden" aria-hidden="true">
            <div class="aurora-ribbon aurora-ribbon-1"></div>
            <div class="aurora-ribbon aurora-ribbon-2"></div>
            <div class="aurora-ribbon aurora-ribbon-3"></div>
            <div class="fixed inset-0 opacity-[0.025] pointer-events-none z-[1]"
                style="background-image: linear-gradient(30deg, var(--aurora-purple) 12%, transparent 12.5%, transparent 87%, var(--aurora-purple) 87.5%), linear-gradient(150deg, var(--aurora-purple) 12%, transparent 12.5%, transparent 87%, var(--aurora-purple) 87.5%), linear-gradient(60deg, var(--aurora-blue) 25%, transparent 25.5%, transparent 75%, var(--aurora-blue) 75%); background-size: 80px 140px; background-position: 0 0, 0 0, 40px 70px;">
            </div>

        </div>
    }
}

#[component]
fn Marquee() -> impl IntoView {
    let skills = ["ANGULAR","REACT","TYPESCRIPT","GO","C#","PYTHON","SQL"];
    view! {
        <div class=" relative py-10 overflow-hidden border-y border-gray-deep relative z-[2] bg-void-light/30 backdrop-blur-sm" aria-hidden="true">
            <div class="flex whitespace-nowrap animate-marquee">
                 <div class ="flex items-center gap-12 px-6">
                    {skills.iter().map(|s| view! {
                        <span class="font-mono text-3xl md:text-5xl font-bold tracking-tighter opacity-20 hover:opacity-100 transition-opacity aurora-text">
                            {*s}
                        </span>
                        <div class="w-2 h-2 rotate-45 bg-aurora-blue"></div>
                    }).collect::<Vec<_>>()}
                 </div>
                 <div class ="flex items-center gap-12 px-6">
                    {skills.iter().map(|s| view! {
                        <span class="font-mono text-3xl md:text-5xl font-bold tracking-tighter opacity-20 hover:opacity-100 transition-opacity aurora-text">
                            {*s}
                        </span>
                        <div class="w-2 h-2 rotate-45 bg-aurora-purple"></div>
                    }).collect::<Vec<_>>()}
                 </div>
            </div>
        </div>
    }
}