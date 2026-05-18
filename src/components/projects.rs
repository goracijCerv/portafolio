use leptos::{prelude::*};

#[derive(Clone)]
struct Project {
    id: &'static str,
    title: &'static str,
    desc: &'static str,
    tags: &'static [&'static str],
    github: &'static str,
    demo: &'static str,
    theme: &'static str
}

const PROJECTS: [Project; 6] = [
    Project {
        id: "PRJ-001", title: "Videojuego VR",
        desc: "Videojuego en realidad virtual con fines terapeuticos para niños con autismo.",
        tags: &["#C", "Unity", "VR"],
        github: "https://github.com/goracijCerv/AppTesinaVr",
        demo: "https://youtu.be/DjuSDeGa7Qk",
        theme: "purple",
    },
    Project {
        id: "PRJ-002", title: "CLOAK",
        desc: "CLI que ayuda a hacer backups de repositorios de git.",
        tags: &["GIT", "GO"],
        github: "https://github.com/goracijCerv/cloak",
        demo: "",
        theme: "blue",
    },
    Project {
        id: "PRJ-003", title: "Api en GO",
        desc: "Una api hecha con la libreria standar de go.",
        tags: &["SQLite", "SQL", "GO"],
        github: "https://github.com/goracijCerv/students-api",
        demo: "",
        theme: "green",
    },
    Project {
        id: "PRJ-004", title: "PORTAFOLIO",
        desc: "Sistema operativo web experimental impulsado por Leptos.",
        tags: &["Leptos", "Rust", "WebGL", "WASM"],
        github: "https://github.com/yourusername/aurora-os",
        demo: "https://aurora-os.vercel.app",
        theme: "purple",
    },
    Project {
        id: "PRJ-005", title: "Backed Sistema con Autorización",
        desc: "Backed de una api donde se tiene permisos de roles y se deben de authentificar para poder realizar dichas operaciones.",
        tags: &["#C", "API", "ASPNET.Identity", "EntityFrameWork"],
        github: "https://github.com/goracijCerv/AuthECAPI",
        demo: "",
        theme: "blue",
    },
    Project {
        id: "PRJ-006", title: "Fronted Sistema con Autorización",
        desc: "El fronted que consume la api con autorización.",
        tags: &["Angular"],
        github: "https://github.com/goracijCerv/AuthCClient",
        demo: "",
        theme: "green",
    },
];

#[component]
pub fn Projects() -> impl  IntoView {
    view!{
        <section id="proyectos" class="p-32 px-6 md:px-12 relative z-2" aria-label="Proyectos">
            <div class="max-w-[1200px] mx-auto">
                
                <div class="reveal mb-16">
                    <h2 class="font-figtree text-3xl md:text-5xl lg:text-6xl font-extrabold leading-tracking tracking-tight text-text-heading">
                        "03. ALGUNOS " <span class="aurora-text">"PROYECTOS"</span> 
                    </h2>
                </div>
                //Grid de proyectos
                <div class="grid gap-8 grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
                    {PROJECTS.into_iter().enumerate().map(|(i,p)| {
                       view! {
                          <ProjectCard project=p index=i/>
                       }
                    }).collect_view()}
                </div>

            </div>
        </section>
    }
}


#[component]
fn ProjectCard(#[prop(into)] project: Project, #[prop(into)] index: usize) -> impl IntoView {
    let card_ref = NodeRef::<leptos::html::Article>::new();

    let (transform, set_transform) = signal("perspective(1000px) rotateX(0deg) translateY(0px)".to_string());
    let (transition, set_transition) = signal("transform 0.5s ease-out".to_string());

    let (dot_color,text_glow,tag_class) = match project.theme {
        "purple" => ("bg-aurora-purple shadow-[0_0_8px_var(--aurora-purple)]", "group-hover:text-aurora-purple-light", "bg-aurora-purple/10 border-aurora-purple/30 text-aurora-purple"),
        "blue" => ("bg-aurora-blue shadow-[0_0_8px_var(--aurora-blue)]", "group-hover:text-aurora-blue-light", "bg-aurora-blue/10 border-aurora-blue/30 text-aurora-blue"),
        "green" => ("bg-aurora-green shadow-[0_0_8px_var(--aurora-green)]", "group-hover:text-aurora-green-light", "bg-aurora-green/10 border-aurora-green/30 text-aurora-green"),
        _ => ("bg-gray-mid", "group-hover:text-text-heading", "bg-gray-deep border-gray-mid text-text-main"),
    };

    let on_mouse_move = move |e:web_sys::MouseEvent| {
        if web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap() < 768.0 {return;}

        if let Some(card) = card_ref.get() {
            let rect = card.get_bounding_client_rect();
            let x = e.client_x() as f64 - rect.left();
            let y = e.client_y() as f64 - rect.top();

            let rx = (rect.height() / 2.0 -y) /15.0;
            let ry = (x - rect.width() /2.0) /15.0;

            set_transform.set(format!("perspective(1000px) rotateX({:.2}deg) rotateY({:.2}deg) translateY(-8px)", rx, ry));
            set_transition.set("transform 0.1s ease-out".to_string());

        }
    };

    let on_mouse_leave = move |_| {
        set_transform.set("perspective(1000px) rotateX(0deg) translateY(0px)".to_string());
        set_transition.set("transform 0.5s ease-out".to_string());
    };

    view!{
        <article
            node_ref = card_ref
            class="project-card reveal group cursor-ponter"
            style:animation-delay=format!("{}ms",index*100)
            style:transform=move || transform.get()
            style:transition=move || transition.get()
            on:mousemove=on_mouse_move
            on:mouseleave=on_mouse_leave
        >
            <div class=" relative z-[1] bg-void-light m-[1px] rounded-2xl p-6 md:p-8 h-full flex flex-col">
                
                <div class="flex justify-between items-center mb-6">
                    <span class="font-mono text-[10px] tracking-widset text-gray">{project.id}</span>
                    <div class="flex gap-1">
                        <div class={format!("w-1.5 h-1.5 rounded-full animate-blink {}",dot_color)}></div>
                        <div class={format!("w-1.5 h-1.5 rounded-full animate-blink opacity-50 {}",dot_color)} style="animation-delay: 0.3s"></div>
                    </div>
                </div>

                //Titulo y descripcion
                <h3 class={format!("font-figtree text-xl font-bold mb-3 transition-colors duration-300 {}",text_glow)}>
                    {project.title}
                </h3>
                <p class="text-sm text-gray leading-relaxed mb-6 flex-grow">
                    {project.desc}
                </p>

                //Tags
                <div class="flex flex-wrap gap-2 mb-8">
                    {project.tags.iter().map(|t| view!{
                        <span class={format!("px-2 py-1 rounded-md font-mono text-[9px] tracking-wider border {}",tag_class)}>
                            {*t}
                        </span>
                    }).collect_view()}
                </div>

                //links inferiores
                <div class="flex gap-4 pt-4 border-t border-gray-deep mt-auto pointer-events-auto">
                    <Show when=move || !project.github.is_empty() fallback=|| ()>
                          <a href={project.github} target="_blank" rel="noopener noreferrer"
                            class="font-mono text-[10px] tracking-widset text-gray hover:aurora-text transition-colors flex items-center gap-1"
                          > 
                            <span class="text-aurora-blue-light">"["</span>"CODIGO" <span class="text-aurora-blue-light">"]"</span>
                         </a>
                    </Show>

                    <Show when=move || !project.demo.is_empty() fallback=|| ()>
                        <a href={project.demo} target="_blank" rel="noopener noreferrer"
                        class="font-mono text-[10px] tracking-widest text-gray hover:aurora-text transition-colors flex items-center gap-1">
                            <span class="text-aurora-green-light">"["</span> "DEMO" <span class="text-aurora-green-light">"]"</span>
                        </a>
                    </Show>
                </div>

            </div>
        </article>
    }
}