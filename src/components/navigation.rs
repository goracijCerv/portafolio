use leptos::prelude::*;
use leptos::task::spawn_local;
use wasm_bindgen::JsCast;
use web_sys::window;
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
        <nav class="fixed right-7 top-1/2 -translate-y-1/2 z-[100] hidden md:flex flex-col gap-8 items-center" aria-label="Sección de navegación">
            {   
                secciones.into_iter().map(|s| {
                    let s_clone = s.to_string();
                    let is_active = move || active_section.get() == s_clone;
                    
                    view!{
                        <div class="group relative flex items-center justify-end">
                            //tooltip
                            <span class="absolute right-10 px-3 py-1 rounded bg-void-light border border-aurora-purple/20
                                        text-[10px] font-mono tracking-[0.2em] text-aurora-purple-light
                                        opacity-0 group-hover:opacity-100 transition-all duration-300 pointer-events-none
                                        translate-x-2 group-hover:translate-x-0 shadow-xl whitespace-nowrap uppercase">
                                {s}
                            </span>

                            //diamante
                            <a  
                                class="nav-dot"
                                href=format!("#{}",s)
                                class:active=is_active
                                on:click= move |_| set_active_section.set(s.to_string())
                                data-section=s
                                aria-label={format!("{} seción", s)}
                            ></a>
                        </div>
                    }
                    
                }).collect_view()
            }
            <div class="absolute -z-10 w-[1px] h-full bg-gradient-to-b from-transparent via-gray-deep to-transparent opacity-20"></div>
        </nav>

        // Mobile bottom nav
        <nav class="fixed bottom-0 left-0 right-0 bg-void/92 backdrop-blur-xl border-t border-gray-deep z-50 lg:hidden flex justify-around py-3.5"
            aria-label="Mobile navigation">
            <a href="#inicio" class="font-mono text-[9px] tracking-wider text-gray-light hover:text-aurora-purple-glow transition-colors">INICIO</a>
            <a href="#acerca" class="font-mono text-[9px] tracking-wider text-gray-light hover:text-aurora-purple-glow transition-colors">ACERCA</a>
            <a href="#habilidades" class="font-mono text-[9px] tracking-wider text-gray-light hover:text-aurora-purple-glow transition-colors">HABILIDADES</a>
            <a href="#proyectos" class="font-mono text-[9px] tracking-wider text-gray-light hover:text-aurora-purple-glow transition-colors">PROJECTOS</a>
            <a href="#contacto" class="font-mono text-[9px] tracking-wider text-gray-light hover:text-aurora-purple-glow transition-colors">CONTACTO</a>
        </nav>
    }
}