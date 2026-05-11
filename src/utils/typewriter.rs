use std::time::Duration;
use leptos::prelude::*;
use leptos::task::spawn_local;

pub fn use_typewriter(frases: Vec<&'static str>) -> ReadSignal<String> {
    let (text, set_text) = signal(String::new());

    spawn_local(async move {
        if frases.is_empty() { return; }

        let mut frase_idx = 0;
        let mut char_idx = 0;
        let mut deleting = false;

        loop {
            let frase = frases[frase_idx];
            let frase_len = frase.chars().count();

            let current = frase.chars().take(char_idx).collect::<String>();
            set_text.set(current);

            // 2. Calcular la siguiente acción y cuánto tiempo debemos esperar para verla
            let delay = if !deleting {
                if char_idx == frase_len {
                    // Si ya escribimos toda la palabra, pausamos largo y empezamos a borrar
                    deleting = true;
                    Duration::from_millis(2500) // 2.5 segundos viendo la palabra completa
                } else {
                    // Si no, seguimos escribiendo
                    char_idx += 1;
                    Duration::from_millis(75) // Velocidad de tipeo
                }
            } else {
                if char_idx == 0 {
                    // Si ya borramos todo, pausamos corto y cambiamos de palabra
                    deleting = false;
                    frase_idx = (frase_idx + 1) % frases.len();
                    Duration::from_millis(600) // 0.6 segundos en blanco antes de escribir
                } else {
                    // Si no, seguimos borrando (más rápido que al escribir)
                    char_idx -= 1;
                    Duration::from_millis(25) // Velocidad de borrado
                }
            };

            // 3. Dormir hasta el siguiente ciclo
            gloo_timers::future::sleep(delay).await;
        }
    });

    text
}