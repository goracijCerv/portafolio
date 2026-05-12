use leptos::prelude::*;
use gloo_timers::future::sleep;
use std::time::Duration;
use crate::utils::scroll::use_reveal_observer;
use serde::Serialize;

#[derive(Serialize)]
struct FormularioGenerico {
    access_key: String,
    nombre_usuario: String,
    correo_usuario: String,
    asunto: String,
    commentario: String,
}

#[component]
pub fn Contact() -> impl IntoView {
    use_reveal_observer();

    let (name, set_name) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (subject, set_subject) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (errors, set_errors)  = signal(Vec::<String>::new());
    
    // Estados del envío
    let (submitting, set_submitting) = signal(false);
    let (submited, set_submited) = signal(false);
    // NUEVO: Estado para manejar errores del servidor
    let (server_error, set_server_error) = signal(false);

    let validate_email = |email: &str| -> bool {
        email.contains('@') && email.contains('.') && email.len() > 5
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let mut errs = Vec::new();

        if name.get().len() < 2 { errs.push("name".to_string()); }
        if !validate_email(&email.get()) { errs.push("email".to_string()); }
        if subject.get().len() < 3 { errs.push("subject".to_string()); }
        if message.get().len() < 10 { errs.push("message".to_string()); }

        set_errors.set(errs.clone());

        if !errs.is_empty() { return; }

        set_submitting.set(true);
        // Limpiamos cualquier error previo al intentar enviar de nuevo
        set_server_error.set(false);

        let data = FormularioGenerico {
            access_key: "c61fdc1a-8af0-4f05-9079-85e75de42e22".to_string(), 
            nombre_usuario: name.get().to_string(),
            correo_usuario: email.get().to_string(),
            asunto: subject.get().to_string(),
            commentario: message.get().to_string(), 
        };

        leptos::task::spawn_local(async move {
            let resp = enviar_a_servidor(data).await;

           
            
            if resp.is_ok() && resp.unwrap() {
                set_submited.set(true);
                
                sleep(Duration::from_millis(3000)).await;
                
                set_submited.set(false);
                set_name.set(String::new());
                set_email.set(String::new());
                set_subject.set(String::new());
                set_message.set(String::new());
                 set_submitting.set(false);
            } else {
                // NUEVO: Si falla, activamos el error
                set_submited.set(false);
                set_server_error.set(true);
                set_submitting.set(false);
            }
        });
    };

    let has_error = move |field:&str| errors.get().contains(&field.to_string());

    view! {
        <section id="contacto" class="py-32 relative z-[2]" aria-label="Sección de Contacto">
            <div class="max-w-[720px] mx-auto px-6 md:px-8 text-center">

                <div class="reveal">
                    <h2 class="font-figtree text-3xl md:text-5xl lg:text-6xl font-extrabold leading-tight tracking-tight text-text-heading">
                        "04. ENVIAR "<span class="aurora-text">"SEÑAL"</span>
                    </h2>
                    <p class="text-gray-mid mt-4 text-sm md:text-base font-figtree">
                        "Envíame un mensaje para trabajar juntos."
                    </p>
                </div>

                <form on:submit=on_submit class="glass rounded-2xl p-6 md:p-10 mt-12 text-left reveal shadow-xl" novalidate>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        
                        <div class="mb-2">
                            <label for="name-input" class="block font-mono text-[9px] tracking-[2px] uppercase mb-2 text-aurora-purple">"Nombre"</label>
                            <input type="text" id="name-input" prop:value=name 
                                on:input=move|e| set_name.set(event_target_value(&e))
                                class="w-full px-4 py-3 bg-void border rounded-lg font-mono text-[13px] text-text-main outline-none transition-all focus:border-aurora-purple focus:shadow-[0_0_15px_rgba(123,44,191,0.15)]"
                                class=("border-red-500", move || has_error("name"))
                                class=("border-gray-deep", move || !has_error("name"))
                                placeholder="Tu nombre" required minlength="2" autocomplete="name"
                            />
                            <Show when=move || has_error("name")>
                                <div class="font-mono text-[10px] text-red-500 mt-1.5">"El nombre debe tener al menos dos caracteres."</div>
                            </Show>
                        </div>

                        <div class="mb-2">
                            <label for="email-input" class="block font-mono text-[9px] tracking-[2px] uppercase mb-2 text-aurora-blue">"Email"</label>
                            <input type="email" id="email-input" prop:value=email 
                                on:input=move|e| set_email.set(event_target_value(&e))
                                class="w-full px-4 py-3 bg-void border rounded-lg font-mono text-[13px] text-text-main outline-none transition-all focus:border-aurora-blue focus:shadow-[0_0_15px_rgba(58,134,255,0.15)]"
                                class=("border-red-500", move || has_error("email"))
                                class=("border-gray-deep", move || !has_error("email"))
                                placeholder="tucorreo@dominio.com" required  autocomplete="email"
                            />
                            <Show when=move || has_error("email")>
                                <div class="font-mono text-[10px] text-red-500 mt-1.5">"Inserte un correo válido."</div>
                            </Show>
                        </div>

                    </div>
                    
                    <div class="mt-6 mb-2">
                        <label for="subject-input" class="block font-mono text-[9px] tracking-[2px] uppercase mb-2 text-aurora-green">"Asunto"</label>
                        <input type="text" id="subject-input" prop:value=subject
                            on:input=move |e| set_subject.set(event_target_value(&e))
                            class="w-full px-4 py-3 bg-void border rounded-lg font-mono text-[13px] text-text-main outline-none transition-all focus:border-aurora-green focus:shadow-[0_0_15px_rgba(46,196,182,0.15)]"
                            class=("border-red-500", move || has_error("subject"))
                            class=("border-gray-deep", move || !has_error("subject"))
                            placeholder="Asunto" required minlength="3" />
                        <Show when=move || has_error("subject")>
                            <div class="font-mono text-[10px] text-red-500 mt-1.5">"El asunto debe tener al menos 3 caracteres"</div>
                        </Show>
                    </div>

                    <div class="mt-6 mb-8">
                        <label for="message-input" class="block font-mono text-[9px] tracking-[2px] uppercase mb-2 text-aurora-purple">"Mensaje"</label>
                        <textarea id="message-input" prop:value=message rows="5"
                            on:input=move |e| set_message.set(event_target_value(&e))
                            class="w-full px-4 py-3 bg-void border rounded-lg font-mono text-[13px] text-text-main outline-none transition-all focus:border-aurora-purple focus:shadow-[0_0_15px_rgba(123,44,191,0.15)] resize-none"
                            class=("border-red-500", move || has_error("message"))
                            class=("border-gray-deep", move || !has_error("message"))
                            placeholder="Hola, mi dinosaurio favorito es..." required minlength="10"></textarea>
                        <Show when=move || has_error("message")>
                            <div class="font-mono text-[10px] text-red-500 mt-1.5">"El mensaje debe tener al menos 10 caracteres"</div>
                        </Show>
                    </div>

                    <button type="submit" disabled=submitting
                        class="w-full py-4 bg-gradient-to-r from-aurora-purple via-aurora-blue to-aurora-green rounded-lg font-mono text-xs tracking-widest text-white relative overflow-hidden transition-all hover:-translate-y-1 hover:shadow-[0_12px_30px_rgba(58,134,255,0.25)] disabled:opacity-50 disabled:cursor-wait disabled:hover:translate-y-0 group"
                    >
                        <div class="absolute inset-0 bg-white/20 translate-y-full group-hover:translate-y-0 transition-transform duration-300 ease-out"></div>
                        <span class="relative z-10 flex justify-center items-center gap-2">
                            {move || if submited.get() {
                                view!{ <span>"[ SEÑAL TRANSMITIDA ✓ ]"</span> }.into_any()
                            } else if submitting.get() {
                                view!{ 
                                    <div class="w-3 h-3 rounded-full border-2 border-white/30 border-t-white animate-spin"></div>
                                    <span>"[ ENCRIPTANDO... ]"</span> 
                                }.into_any()
                            } else {
                                view!{ <span>"[ TRANSMITIR MENSAJE ]"</span> }.into_any()
                            }}
                        </span>
                    </button>
                    
                    // NUEVO: Mensaje de error del servidor
                    <Show when=move || server_error.get()>
                        <div class="mt-4 text-center font-mono text-[10px] tracking-widest text-red-500 bg-red-500/10 border border-red-500/30 py-3 rounded-lg animate-pulse">
                            "[ ERROR: INTENTA DE NUEVO MÁS TARDE. ]"
                        </div>
                    </Show>

                </form>
            </div>
        </section>
    }
}

async fn enviar_a_servidor(datos: FormularioGenerico) -> Result<bool, reqwest::Error> {
    let cliente = reqwest::Client::new();

    let respuesta = cliente.post("https://api.web3forms.com/submit")
                    .json(&datos)
                    .send()
                    .await?;
    if respuesta.status().is_success() {
        Ok(true)
    }else{
        Ok(false)
    }
}