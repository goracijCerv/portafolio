use leptos::prelude::*;
use leptos_router::components::Router;

pub mod components;
pub mod utils;

// use components::{
// };

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <BackGround />
            <main>
                <Marquee />
            </main>
        </Router>
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
                style="background-image: linear-gradient(30deg, #7b2cbf 12%, transparent 12.5%, transparent 87%, #7b2cbf 87.5%), linear-gradient(150deg, #7b2cbf 12%, transparent 12.5%, transparent 87%, #7b2cbf 87.5%), linear-gradient(60deg, #3a86ff 25%, transparent 25.5%, transparent 75%, #3a86ff 75%); background-size: 80px 140px; background-position: 0 0, 0 0, 40px 70px;">
            </div>

        </div>
    }
}

#[component]
fn Marquee() -> impl IntoView {
    let skills = ["ANGULAR","REACT","TYPESCRIPT","GO","C#","PYTHON","SQL"];
    view! {
        <div class="py-8 overflow-hidden border-y border-gray-deep relative z-[2]" aria-hidden="true">
            <div class="inline-block whitespace-nowrap animate-marquee">
                 {skills.iter().map(|s| view! {
                    <span class="font-mono text-5xl font-bold text-transparent mr-8"
                          style="-webkit-text-stroke:1px #2d2d44;">
                        {*s}
                    </span>
                    
                 }).collect::<Vec<_>>()}
                 {skills.iter().map(|s| view! {
                    <span class="font-mono text-5xl font-bold text-transparent mr-8"
                          style="-webkit-text-stroke:1px #2d2d44;">
                        {*s}
                    </span>
                    
                 }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}