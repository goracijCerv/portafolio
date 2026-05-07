use std::time::Duration;

use leptos::prelude::*;
use leptos::reactive::signal::{ReadSignal, signal};
use leptos::task::spawn_local;


pub fn use_typewriter(frases: Vec<&'static str>) -> ReadSignal<String> {
    let (text, set_text) = signal(String::new());
    let (running, set_runnig) = signal(false);
    let (initialized, set_initialized)  =signal(false);

    Effect::new(move |_| {
        if initialized.get() {
            return;
        }

        set_initialized.set(true);
        let frases = frases.clone();
        spawn_local(async move {
            let mut frase_idx = 0;
            let mut char_idx = 0;
            let mut deleting = false;

            while running.get() {
                let frase = frases[frase_idx];
                let frase_len = frase.chars().count();

                let current = frase.chars().take(char_idx).collect::<String>();
                set_text.set(current);

                if deleting {
                    if char_idx > 0 {
                        char_idx -= 1;
                    }
                }else if char_idx < frase_len {
                    char_idx += 1;
                }

                let delay = if !deleting && char_idx == frase_len {
                    deleting = true;
                    Duration::from_millis(2200)
                }else if deleting && char_idx == 0 {
                    deleting = false;
                    frase_idx = (frase_idx + 1 ) % frases.len();
                    Duration::from_millis(500)
                }else {
                    Duration::from_millis(if deleting {28} else {75})
                };

                gloo_timers::future::sleep(delay).await;
            }
        });
    });

    on_cleanup(move || {
        set_runnig(false);
    });

    text
}