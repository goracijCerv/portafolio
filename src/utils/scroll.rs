use leptos::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use std::cell::Cell;
use std::rc::Rc;

pub fn use_scroll_progress() -> ReadSignal<f64> {
    let (progress, set_progress) = signal(0.0);

    Effect::new(move |_| {
        #[cfg(not(target_arch = "wasm32"))]
        return;

        #[cfg(target_arch = "wasm32")]
        {
            let Some(window) = web_sys::window() else {
                return;
            };

            let ticking = Rc::new(Cell::new(false));

            let window_scroll = window.clone();
            let ticking_scroll = ticking.clone();

            let scroll_closure = Closure::wrap(Box::new(move || {
                if ticking_scroll.get() {
                    return;
                }

                ticking_scroll.set(true);

                let window_raf = window_scroll.clone();
                let ticking_raf = ticking_scroll.clone();
                let set_progress = set_progress;

                let raf_callback = move || {
                    let Some(pct) = get_scroll_percentage(&window_raf) else {
                        ticking_raf.set(false);
                        return;
                    };

                    set_progress.set(pct);
                    ticking_raf.set(false);
                };

                let raf = Closure::once_into_js(raf_callback);

                let _ = window_scroll.request_animation_frame(
                    raf.as_ref().unchecked_ref(),
                );
            }) as Box<dyn FnMut()>);

            let _ = window.add_event_listener_with_callback(
                "scroll",
                scroll_closure.as_ref().unchecked_ref(),
            );

            on_cleanup(move || {
                let _ = window.remove_event_listener_with_callback(
                    "scroll",
                    scroll_closure.as_ref().unchecked_ref(),
                );
            });
        }
    });

    progress
}

fn get_scroll_percentage(window: &web_sys::Window) -> Option<f64> {
    let documet = window.document()?;
    let html = documet.document_element()?;

    let scroll_top = html.scroll_top() as f64;
    let scroll_height = html.scroll_height() as f64;
    let client_height = html.client_height() as f64;
    let max_scroll = (scroll_height - client_height).max(1.0);

    Some((scroll_top / max_scroll * 100.0).clamp(0.0, 100.0))
    
}