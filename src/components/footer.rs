use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    let current_year = js_sys::Date::new_0().get_full_year().to_string();

    view!{
        <footer class="relative z-2 border-t border-gray-deep bg-void/80 backdrop-blur-md pt-10 pb-24 lg:pb-10 px-6 mt-20">
            <div class="max-w-[1200px] mx-auto flex flex-col md:flex-row justify-between items-center gap-6 text.center md:text-left">

                <div class="flex flex-col gap-2">
                    <div class="fotn-figtree text-lg font-bold text-text-heading flex items-center justify-center md:justify-start gap-2">
                        "HORACIO " <span class="text-aurora-purple-light">"// PORTAFOLIO"</span>
                    </div>
                    <div class="font-mono text-[10px] tracking-widest text-gray uppercase flex items-center justify-center md:justify-start gap-2">
                        <span>{format!("© {} Todos los derechos reservados.", current_year)}</span>
                    </div>
                </div>

                <div class="flex gap-4">
                    //Github 
                    <a href="https://github.com/goracijCerv" target="_blank" rel="noopener noreferrer" 
                       class="w-10 h-10 rounded-full border border-gray-deep flex items-center justify-center text-gray-mid hover:text-aurora-purple-light hover:border-aurora-purple-light hover:-translate-y-1 hover:shadow-[0_4px_15px_rgba(123,44,191,0.2)] transition-all duration-300"
                       aria-label="GitHub">
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                        </svg>
                    </a>

                    <a href="https://linkedin.com/in/horacio-cervantes-p-9663a8251" target="_blank" rel="noopener noreferrer"
                       class="w-10 h-10 rounded-full border border-gray-deep flex items-center justify-center text-gray-mid hover:text-aurora-blue-light hover:border-aurora-blue-light hover:-translate-y-1 hover:shadow-[0_4px_15px_rgba(58,134,255,0.2)] transition-all duration-300"
                       aria-label="LinkedIn">
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                            <path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path>
                            <rect x="2" y="9" width="4" height="12"></rect>
                            <circle cx="4" cy="4" r="2"></circle>
                        </svg>
                    </a>

                    <a href="https://x.com/CasualSoul01" target="_blank" rel="noopener noreferrer"
                    class="w-10 h-10 rounded-full border border-gray-deep flex items-center justify-center text-gray-mid hover:text-text-heading hover:border-text-heading hover:-translate-y-1 hover:shadow-[0_4px_15px_rgba(15,23,42,0.15)] dark:hover:shadow-[0_4px_15px_rgba(255,255,255,0.15)] transition-all duration-300"
                    aria-label="X">
                        <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" fill="currentColor" class="bi bi-twitter-x" viewBox="0 0 24 24">
                            <path d="M12.6.75h2.454l-5.36 6.142L16 15.25h-4.937l-3.867-5.07-4.425 5.07H.316l5.733-6.57L0 .75h5.063l3.495 4.633L12.601.75Zm-.86 13.028h1.36L4.323 2.145H2.865z"/>
                        </svg>
                    </a>


                </div>

            </div>

            <div class="absolute bottom-0 left-1/2 -translate-x-1/2 w-1/2 h-[2px] bg-gradient-to-r from-transparent via-aurora-purple to-transparent opacity-30 blur-[2px]"></div>
        </footer>
    }

}