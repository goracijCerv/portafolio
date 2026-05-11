use leptos::prelude::*;
use crate::utils::{particles::init_particles, typewriter::use_typewriter, scroll::use_reveal_observer,};


#[component]
pub fn Hero() -> impl IntoView {
    use_reveal_observer();

    let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
    init_particles(canvas_ref);

    let typewriter_text = use_typewriter(vec![
        "Desarollador web...",
        "Gamer...",
        "Optimizando para la experiencia humana...",
        "Estudiante constante...",
        "Desplegar todo el espectro...",
        "Dinosuario favorito el ticeraptor y spinosuario...",
    ]);

    view! {
        <header id="inicio" class="relative min-h-screen flex items-center justify-center overflow-hidden">

            <canvas
                node_ref=canvas_ref
                id="particles-canvas"
                class="absolute inset-0 w-full h-full pointer-events-none z-[0]"
                aria-hidden=true>
            </canvas>

            <div class="relative z-[2] max-w-[1200px] mx-auto px-8 w-full">

                 <div class="reveal inline-flex items-center gap-2 px-4 py-1.5 rounded-full border font-mono text-[10px] tracking-[3px] text-aurora-purple-light mb-6 mt-2"
                            style="background-color: color-mix(in srgb, var(--aurora-purple) 10%, transparent); border-color: color-mix(in srgb, var(--aurora-purple) 30%, transparent);">
                        <span class="w-1.5 h-1.5 rounded-full bg-aurora-green-light shadow-[0_0_8px_var(--aurora-green)] animate-blink"></span>
                        <span>"PORTAFOLIO ONLINE // PORTAFOLIO V1.0"</span>
                 </div>

                //Titulo
                <h1 class="reveal font-sans text-[clamp(3.5rem,12vw,9rem)] font-extrabold tracking-[3px] leading-[0.95] my-6 text-text-heading">
                    <span class="glitch" data-text="HORACIO">"HORACIO"</span>
                    <br/>
                    <span class="aurora-text text-[0.5em] font-light tracking-[4px]">"ING. CIENCIAS COMPUTACIONALES"</span>
                </h1>

                //maquina de escribir
                <div class="reveal mt-5 min-h-[1.5em]">
                    <span class="font-mono text-[clamp(0.9rem,2vw,1.2rem)] text-gray typing-cursor" role="status" aria-live="polite">
                        {move || typewriter_text.get()}
                    </span>
                </div>

                //DESCRIPCION
                <p class=" reveal max-w-[480px] mt-6 text-gray-ligt text-sm md:text-base leading-relaxed font-figtree">
                    "Creando expereicnias y herramientas digitales a través de todo el espectro del código."
                </p>

                //Botones
                <div class="reveal flex gap-4 flex-wrap mt-10">
                    <a href="#proyectos" class="crystal-btn-primary">"[ REVISAR PROYECTOS ]"</a>
                    <a href="#contacto" class="crystal-btn border border-gray-mid text-text-main hover:border-aurora-purple-light hover:text-aurora-purple-glow">
                        "[ CONECTAR ]"
                    </a>
                </div>

                //BENTOS GRID

                <div class="reveal grid grid-cols-4 gap-4 mt-12 max-md:grid-cols-2 mb-2">
                    <BentoCard value="10+" label="Proyectos" color="purple" />
                    <BentoCard value="2" label="Años Exp." color="blue" />
                    <BentoCard value="8" label="Lenguajes" color="green" />
                    //Carta de disponibilidad
                    <div class="reveal glass rounded-2xl p-5 flex items-center gap-3 hover:border-aurora-purple-light/50 hover:-translate-y-1 transition-all duration-300 shadow-sm">
                        <span class="w-2 h-2 rounded-full bg-aurora-green-light shadow-[0_0_10px_var(--aurora-green)] animate-blink"></span>
                        <div>
                            <div class="font-mono text-aurora-green-light text-[0.9rem] font-bold">"DISPONIBLE"</div>
                            <div class="font-mono text-[8px] tracking-[2px] text-gray uppercase mt-1">"Open to work"</div>
                        </div>
                    </div>
                </div>

            </div>
                //iNDICADOR DE SCROOL
            <div class="absolute bottom-10 left-1/2 -translate-x-1/2 flex felx-col item-center gap-2 opacity-40">
                <span class="font-mono text-[9px] tracking-[3px] text-gray-mid uppercase">"Scroll"</span>
                <div class="w-px h-12 bg-gradient-to-b from-aurora-purple to-transparent animate-scroll-pulse"></div>
            </div>
        </header>
    }
}

#[component]
fn BentoCard( #[prop(into)] value: String, #[prop(into)] label: String, #[prop(into)] color: String) -> impl  IntoView {
    let color_class = match color.as_str() {
        "purple" => "text-aurora-purple",
        "blue" => "text-aurora-blue",
        "green" => "text-aurora-green",
        _ => "text-text-main",
    };

    view! {
        <div class="glass rounded-2xl p-5 flex flex-col justify-center hover:border-aurora-purple-light/50 hover:-translate-y-1 transition-all duration-300 shadow-sm">
            <div class={format!("font-sans text-3xl md:text-4xl font-extrabold {}", color_class)}>{value}</div>
            <div class="font-mono text-[8px] md:text-[9px] tracking-[2px] text-gray uppercase mt-1">{label}</div>
        </div>
    }
}