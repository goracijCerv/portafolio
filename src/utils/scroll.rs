use leptos::prelude::*;
use leptos::ev; // Eventos nativos de Leptos
use std::cell::Cell;
use std::rc::Rc;
use leptos::task::spawn_local;
use wasm_bindgen::prelude::*;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit, Element};

pub fn use_scroll_progress() -> ReadSignal<f64> {
    let (progress, set_progress) = signal(0.0);

    // Creamos nuestro "portero" para el Throttling
    let ticking = Rc::new(Cell::new(false));

    // window_event_listener de Leptos maneja la memoria y los hilos por nosotros
    window_event_listener(ev::scroll, move |_| {
        if ticking.get() {
            return; // Ignoramos si ya estamos esperando un frame
        }

        ticking.set(true);
        let ticking_for_raf = ticking.clone();

        // Usamos el request_animation_frame nativo de Leptos (¡adiós wasm_bindgen!)
        request_animation_frame(move || {
            if let Some(window) = web_sys::window() {
                if let Some(pct) = calculate_scroll_percentage(&window) {
                    set_progress.set(pct);
                }
            }
            // Abrimos la puerta para el siguiente evento
            ticking_for_raf.set(false);
        });
    });

    progress
}

// Función matemática pura (sin cambios)
fn calculate_scroll_percentage(window: &web_sys::Window) -> Option<f64> {
    let document = window.document()?;
    let html = document.document_element()?;

    let scroll_top = html.scroll_top() as f64;
    let scroll_height = html.scroll_height() as f64;
    let client_height = html.client_height() as f64;
    
    let max_scroll = (scroll_height - client_height).max(1.0);
    Some((scroll_top / max_scroll * 100.0).clamp(0.0, 100.0))
}


pub fn use_reveal_observer() {
    spawn_local(async move {
        let _ = leptos::prelude::request_animation_frame(move || {
            let document = web_sys::window().unwrap().document().unwrap();
             //se ejecutara cuando entre en la pantalla
            let clousre = Closure::wrap(Box::new(move |entries: js_sys::Array|  {
                for entry in entries.iter() {
                    let  entry: IntersectionObserverEntry = entry.unchecked_into();
                    if entry.is_intersecting() {
                        let target = entry.target();
                        if let Some(el) = target.dyn_ref::<Element>(){
                            let _ = el.class_list().add_1("visible");
                        }
                    }
                }
            }) as Box<dyn FnMut(js_sys::Array)>);

            let options = IntersectionObserverInit::new();
            options.set_threshold(&js_sys::Array::of1(&JsValue::from_f64(0.08)));
            options.set_root_margin("0px 0px -40px 0px");

            let observer = IntersectionObserver::new_with_options(
                clousre.as_ref().unchecked_ref(), 
                &options).unwrap();
            
            if let Ok(elements) = document.query_selector_all(".reveal, .reveal-left, .reveal-right") {
                for i in 0..elements.length() {
                    if let Some(node) = elements.item(i) {
                        if let Ok(el) = node.dyn_into::<Element>() {
                            observer.observe(&el);
                        }
                    }
                }
            }

            clousre.forget();
        });
    });
}