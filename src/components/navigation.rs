use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::window;
use leptos_use::{ColorMode, UseColorModeOptions, UseColorModeReturn, use_color_mode, use_color_mode_with_options};

#[component]
pub fn Navigation() -> impl IntoView {
    let secciones = [
        "inicio",
        "acerca",
        "habilidades",
        "proyectos",
        "contacto",
    ];

    let (active_section, set_active_section) = signal("inicio".to_string());
    
    // 1. NUEVA SEÑAL: Controla si el menú móvil está abierto o cerrado
    let (is_menu_open, set_is_menu_open) = signal(false);
    
    spawn_local(async move {
        let win = window().expect("Solo funciona en navegador");

        let on_scroll = move |_: web_sys::Event| {
            let document = window().unwrap().document().unwrap();
            for id in secciones {
                if let Some(el) = document.get_element_by_id(id) {
                    let rect = el.get_bounding_client_rect();
                    if rect.top() >= -200.0 && rect.top() <= 400.0 {
                        set_active_section.set(id.to_string());
                        break;
                    }
                }
            }
        };

        let clousre = wasm_bindgen::closure::Closure::wrap(Box::new(on_scroll) as Box<dyn FnMut(_)>);
        win.add_event_listener_with_callback("scroll", clousre.as_ref().unchecked_ref()).unwrap();
        clousre.forget(); //esto deja vivo siempre el evento
    });

    view! {
        // --- NAVEGACIÓN DESKTOP (Intacta) ---
        <nav class="fixed right-7 top-1/2 -translate-y-1/2 z-[100] hidden md:flex flex-col gap-6 items-center" aria-label="Sección de navegación">
            <div class="mb-4">
                <ThemeToggle />
            </div>
            {   
                secciones.into_iter().map(|s| {
                    
                    view!{
                        <div class="group relative flex items-center justify-end">
                            <span class="absolute right-10 px-3 py-1 rounded bg-void-light border border-aurora-purple/20
                                        text-[10px] font-mono tracking-[0.2em] text-aurora-purple-light
                                        opacity-0 group-hover:opacity-100 transition-all duration-300 pointer-events-none
                                        translate-x-2 group-hover:translate-x-0 shadow-xl whitespace-nowrap uppercase">
                                {s}
                            </span>
                            <a  
                                class="nav-dot"
                                href=format!("#{}",s)
                                class:active=move || active_section.get() == s
                                on:click= move |_| set_active_section.set(s.to_string())
                                data-section=s
                                aria-label={format!("{} sección", s)}
                            ></a>
                        </div>
                    }
                }).collect_view()
            }
            <div class="absolute -z-10 w-[1px] h-full bg-gradient-to-b from-transparent via-gray-deep to-transparent opacity-20"></div>
        </nav>

        // --- BOTÓN HAMBURGUESA MÓVIL ---
        // Se queda fijo arriba a la derecha. Tiene z-[110] para estar siempre por encima de todo.
        <button 
            class="md:hidden fixed top-6 right-6 z-[110] w-12 h-12 flex items-center justify-center rounded-full bg-void-light/80 backdrop-blur-md border border-gray-deep text-text-main shadow-lg transition-transform hover:scale-105"
            on:click=move |_| set_is_menu_open.update(|open| *open = !*open) // Alterna entre true/false
            aria-label="Menú de navegación"
        >
            {move || if is_menu_open.get() {
                // Ícono de "X" cuando está abierto
                view! {
                    <svg class="h-6 w-6 text-aurora-purple-light" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                }.into_any()
            } else {
                // Ícono de 3 líneas cuando está cerrado
                view! {
                    <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                    </svg>
                }.into_any()
            }}
        </button>

        // --- PANTALLA COMPLETA DEL MENÚ MÓVIL (Overlay) ---
        // Usamos una clase dinámica para cambiar la opacidad y los eventos dependiendo de `is_menu_open`
        <div 
            class=move || format!("md:hidden fixed inset-0 z-[105] bg-void/95 backdrop-blur-2xl flex flex-col items-center justify-center gap-10 transition-all duration-500 ease-in-out {}", 
                if is_menu_open.get() { "opacity-100 pointer-events-auto" } else { "opacity-0 pointer-events-none" }
            )
        >
            // Colocamos el interruptor de tema hasta arriba del menú móvil
            <ThemeToggle />
            
            <div class="flex flex-col items-center gap-8">
                {   
                    secciones.into_iter().map(|s| {
                        
                        view!{
                            <a  
                                href=format!("#{}", s)
                                class="text-2xl font-mono tracking-[0.2em] uppercase transition-all duration-300"
                                class=("text-aurora-purple-light scale-110", move || active_section.get() == s)
                                class=("text-text-main hover:text-aurora-purple-glow", move || active_section.get() != s)
                                on:click=move |_| {
                                    set_active_section.set(s.to_string());
                                    // 2. ¡MUY IMPORTANTE! Al hacer clic en un enlace, cerramos el menú
                                    set_is_menu_open.set(false);
                                }
                            >
                                {s}
                            </a>
                        }
                    }).collect_view()
                }
            </div>
        </div>
    }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    
    let opciones = UseColorModeOptions::default()
        .attribute("class") // Inyecta la clase en el <html>
        .storage_key("color-scheme"); // Coincide con nuestro script de arriba

    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode_with_options(opciones);

    let alternar_tema = move |_| {
        if mode.get() == ColorMode::Dark {
            set_mode.set(ColorMode::Light);
        } else {
            set_mode.set(ColorMode::Dark);
        }
    };

    view! {
        // CORRECCIÓN: Convertimos el botón en un círculo perfecto (w-8 h-8)
        // en lugar de usar padding rectangular (px-4 py-2) para que quepa perfecto en ambas barras.
        <button 
            on:click=alternar_tema
            class="w-8 h-8 flex items-center justify-center rounded-full border border-gray-deep bg-void-light text-text-main transition-transform duration-300 hover:scale-110 shadow-lg"
            aria-label="Alternar tema de color"
        >
            {move || if mode.get() == ColorMode::Dark {
                view! { 
                    <span class="text-aurora-blue-light text-sm">"☀️"</span> 
                }.into_any()
            } else {
                view! { 
                    <span class="text-aurora-purple-light text-sm">"🌙"</span> 
                }.into_any()
            }}
        </button>
    }
}