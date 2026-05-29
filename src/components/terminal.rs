use leptos::prelude::*;

#[component]
pub fn Terminal() -> impl IntoView {
    view! {
        <section id="acerca" class="relative py-24 px-6 md:px-12 z-[2]" aria-label="Sección Acerca de mí">

            <div class="max-w-[1100px] mx-auto">

                // TÍTULO
                <div class="flex items-center gap-4 mb-12">
                    <h2 class="font-oxanium text-3xl md:text-5xl font-bold text-text-heading">
                        "01. " <span class="aurora-text">"ACERCA"</span>
                    </h2>
                    // La línea decorativa ahora respetará el ancho máximo
                    <div class="flex-1 h-px bg-gradient-to-r from-aurora-purple/50 to-transparent"></div>
                </div>

                // 2. GRID DE CONTENIDO (Terminal + Timeline)

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 lg:gap-16 items-center">

                    // CARD TERMINAL
                    <div class="terminal reveal-left w-full shadow-xl">
                        // Barra superior
                        <div class="terminal-bar">
                            <div class="flex gap-2">
                                <div class="w-3 h-3 rounded-full bg-red-500/80 shadow-[0_0_8px_rgba(239,68,68,0.5)]"></div>
                                <div class="w-3 h-3 rounded-full bg-yellow-500/80 shadow-[0_0_8px_rgba(234,179,8,0.5)]"></div>
                                <div class="w-3 h-3 rounded-full bg-green-500/80 shadow-[0_0_8px_rgba(34,197,94,0.5)]"></div>
                            </div>
                            <div class="mx-auto text-[10px] font-mono text-white/80 tracking-widest uppercase opacity-80">
                                "horacio@root-system:~"
                            </div>
                        </div>

                        // Contenido de la Terminal
                        <div class="p-6 md:p-8 text-sm md:text-base font-mono leading-relaxed text-gray">

                            // Comando 1: whoami
                            <div class="mb-4">
                                <span class="text-aurora-green-light">"horacio@root-system"</span>
                                <span class="text-text-main">":"</span>
                                <span class="text-aurora-blue-light">"~"</span>
                                <span class="text-text-main">"$ cat whoami.txt"</span>
                            </div>

                            // Respuesta 1
                            <div class="mb-8 pl-4 border-l-2 border-aurora-purple/30 text-gray">
                                <p class="mb-3">
                                    "¡Hola! Soy Horacio, ingeniero en Ciencias Computacionales."
                                </p>
                                <p class="mb-3">
                                    "Me especializo en construir herramientas y experiencias digitales escalables, abarcando todo el espectro del desarrollo, desde el frontend hasta el backend."
                                </p>
                                <p>
                                    "Cuando no estoy escribiendo código o creando interfaces, probablemente me encuentres jugando videojuegos (gamer de corazón) o investigando datos curiosos (Team Triceratops & Spinosaurus 🦕)."
                                </p>
                            </div>

                            // Comando 2: skills
                            <div class="mb-4">
                                <span class="text-aurora-green-light">"horacio@root-system"</span>
                                <span class="text-text-main">":"</span>
                                <span class="text-aurora-blue-light">"~"</span>
                                <span class="text-text-main">"$ ./fetch_core_modules.sh"</span>
                            </div>

                            // Respuesta 2
                            <div class="mb-8 pl-4 grid grid-cols-1 sm:grid-cols-2 gap-2 text-aurora-purple">
                                <div class="flex items-center gap-2">
                                    <span class="text-aurora-blue-light">">"</span> "Desarrollo Full‑Stack"
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="text-aurora-blue-light">">"</span> "UI/UX"
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="text-aurora-blue-light">">"</span> "Experiencia con IA"
                                </div>
                                <div class="flex items-center gap-2">
                                    <span class="text-aurora-blue-light">">"</span> "Resolución de problemas"
                                </div>
                            </div>

                            // Línea de entrada activa
                            <div class="flex items-center">
                                <span class="text-aurora-green-light">"horacio@root-system"</span>
                                <span class="text-text-main">":"</span>
                                <span class="text-aurora-blue-light">"~"</span>
                                <span class="text-text-main ml-1 mr-2">"$"</span>
                                <span class="typing-cursor"></span>
                            </div>
                        </div>
                    </div>

                    // TIMELINE

                    <div class="reveal-right w-full lg:pl-10 flex flex-col h-full justify-between">
                        <div class="relative before:content-[''] before:absolute before:left-0 before:top-0 before:bottom-0 before:w-px before:bg-gradient-to-b before:from-aurora-purple before:via-aurora-blue before:to-aurora-green">
                            <TimeLineItem
                                year="2023 — PRESENTE"
                                role="Jr Full-Stack Developer"
                                company="IT-Seekers"
                                desc="Experiencia en desarrollo de APIs en Express y .NET; en el frontend se usó Angular y React."
                                color="blue"
                            />
                            <TimeLineItem
                                year="2022 — 2023"
                                role="Practicante"
                                company="Sharevolts"
                                desc="Desarrollo con Power Automate para aplicaciones de prueba y gestión de entornos empresariales con Microsoft 365."
                                color="green"
                            />

                        </div>

                        <div class="mt-8 pt-6 flex sm:justify-start justify-center">
                            <a
                                target="_blank"
                                href="public/cv-horacio.pdf"
                                class="crystal-btn-primary flex items-center justify-center gap-3 no-underline w-full sm:w-auto"
                            >
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
                                </svg>
                                <span class="font-bold tracking-wider font-mono text-xs">"DESCARGAR CURRÍCULUM"</span>
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn TimeLineItem(
    #[prop(into)] year: String,
    #[prop(into)] role: String,
    #[prop(into)] company: String,
    #[prop(into)] desc: String,
    #[prop(into)] color: String,
) -> impl IntoView {
    let dot_color = match color.as_str() {
        "purple" => "bg-aurora-purple-glow",
        "blue" => "bg-aurora-blue-light",
        "green" => "bg-aurora-green-light",
        _ => "bg-gray-light",
    };

    let role_color = match color.as_str() {
        "purple" => "text-aurora-purple-light",
        "blue" => "text-aurora-blue-light",
        "green" => "text-aurora-green-light",
        _ => "text-text-main",
    };

    view! {
        <div class="pl-8 pb-8 relative group">
            // Diamante en la línea
            <div class={format!("absolute -left-[4.5px] top-1 w-2.5 h-2.5 rounded-full border-2 border-void transition-all duration-300 group-hover:scale-150 {}", dot_color)}></div>

            // Contenido
            <div class="font-mono text-[10px] tracking-[2px] text-gray mb-1">{year}</div>
            <div class={format!("font-figtree text-lg md:text-xl font-bold {}", role_color)}>{role}</div>
            <div class="text-sm text-gray mb-1.5 font-medium">{company}</div>
            <div class="text-sm text-gray leading-relaxed">{desc}</div>
        </div>
    }
}
