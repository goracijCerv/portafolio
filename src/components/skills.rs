use leptos::prelude::*;

#[component]
pub fn Skils() -> impl IntoView {
    let skills = vec![
        ("Javascript", "JS", "#f7df1e"),
        ("Typescript", "TS", "#3178c6"),
        ("Python", "Py", "#4B8BBE"),
        ("React", "Re", "#61DBFB"),
        ("Node", "No", "#68A063"),
        ("Go", "Go", "#00ADD8"),
        ("SQL", "SQL","#ec7000"),
        ("Angular", "An", "#C3002F")

    ];
     view! {
        <section id="habilidades" class="relative py-24 px-6 md:px-12 z-[2]" aria-label="Sección de Habilidades">
         <div class="max-w-[1000px] mx-auto">
                
                // TÍTULO DE LA SECCIÓN (Alineado a la derecha para contrastar con "ACERCA")
                <div class="flex items-center gap-4 mb-16 reveal-right">
                    <div class="flex-1 h-px bg-gradient-to-l from-aurora-blue/50 to-transparent"></div>
                    <h2 class="font-oxanium text-3xl md:text-5xl font-bold text-text-heading">
                        <span class="aurora-text">"HABILIDADES"</span> " .02"
                    </h2>
                </div>

                // CONTENEDOR DEL PANAL (Honeycomb)
                <div class="flex flex-wrap justify-center gap-4 md:gap-6 max-w-[850px] mx-auto reveal">
                    {
                        skills.into_iter().map(|(name, ico,color)| {
                            view! {
                                <div class="hex-skill group">
                                    // Un brillo interior muy sutil que se activa en hover
                                    <div class="absolute inset-1 opacity-10 group-hover:opacity-30 transition-opacity duration-300 pointer-events-none"
                                         style="clip-path: polygon(50% 0%, 100% 25%, 100% 75%, 50% 100%, 0% 75%, 0% 25%); background: var(--aurora-blue);">
                                    </div>
                                    
                                    // Textos del hexágono
                                    <div class="relative z-2 text-center flex flex-col items-center justify-center">
                                        // Nombre de la tecnología
                                        <div class="font-figtree font-bold text-sm md:text-base text-text-main " style={format!("color: {}",color)}>
                                            {name}
                                        </div>
                                        
                                        // Categoría
                                        <div class="font-mono text-[9px] tracking-widest  mt-1 uppercase group-hover:text-aurora-purple-glow transition-colors">
                                            {ico}
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect_view()
                    }
                </div>

                // LUZ DE FONDO (Le da un brillo místico a toda la sección)
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-aurora-green/5 blur-[100px] rounded-full pointer-events-none -z-2"></div>
            </div>
        </section>
     }
}