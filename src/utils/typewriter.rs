use leptos::prelude::*;
use std::time::Duration;

pub fn use_typewriter(frases: Vec<&'static str>) -> ReadSignal<String> {
   let (text,set_text) = signal(String::new());
   Effect::new(move |_| {
     let frases = frases.clone();
     spawn_local(async move {
        let mut frase_idx = 0usize;
        let mut char_idx = 0usize;
        let mut deleting = false;

        loop {
            let frase = frases[frase_idx];

            if deleting {
                if char_idx > 0 {
                    char_idx -= 1;
                }
                set_text.set(frase[..char_idx].to_string());
            } else {
                if char_idx < frase.len() {
                    char_idx += 1;
                }
                set_text.set(frase[..char_idx].to_string());
            }

            let delay = if !deleting && char_idx == frase.len() {
                deleting = true;
                Duration::from_millis(2200)
            } else if deleting && char_idx == 0 {
                deleting = false;
                frase_idx = (frase_idx + 1) %  frases.len();
                Duration::from_millis(500)
            } else {
                Duration::from_millis(if deleting { 28 } else {75})
            };

            gloo_timers
        }
     });
   });
   text
}