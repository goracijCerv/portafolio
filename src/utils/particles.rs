use leptos::prelude::*;
use leptos::ev; // <- ¡Importamos el sistema de eventos de Leptos!
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::CanvasRenderingContext2d;

const COLORS: [&str; 4] = [
    "var(--aurora-purple)",
    "var(--aurora-blue)",
    "var(--aurora-green)",
    "var(--aurora-purple-glow)",
];

struct Particle {
    x: f64,
    y: f64,
    size: f64,
    speed_x: f64,
    speed_y: f64,
    opacity: f64,
    color: &'static str,
    connections: usize,
}

pub fn init_particles(canvas_ref: NodeRef<leptos::html::Canvas>) {
    Effect::new(move |_| {
        let Some(canvas) = canvas_ref.get() else { return };
        
        let window = web_sys::window().unwrap();
        
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        // 1. RESIZE CON LEPTOS IDIOMÁTICO (Sin Closure::wrap)
        window_event_listener(ev::resize, {
            let canvas = canvas.clone();
            move |_| {
                if let Some(w) = web_sys::window() {
                    let width = w.inner_width().unwrap().as_f64().unwrap();
                    let height = w.inner_height().unwrap().as_f64().unwrap();
                    canvas.set_width(width as u32);
                    canvas.set_height(height as u32);
                }
            }
        });
        
        // Forzamos un resize inicial
        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        let is_mobile = width < 768.0;
        let count = if is_mobile { 25 } else { 50 };
        let connection_dist = if is_mobile { 80.0 } else { 100.0 };
        let max_connections = 3;

        let mut particles: Vec<Particle> = (0..count).map(|i| {
            let color_idx = i % COLORS.len();
            Particle {
                x: js_sys::Math::random() * canvas.width() as f64,
                y: js_sys::Math::random() * canvas.height() as f64,
                size: js_sys::Math::random() * 1.5 + 0.5,
                speed_x: (js_sys::Math::random() - 0.5) * 0.5,
                speed_y: (js_sys::Math::random() - 0.5) * 0.5,
                opacity: js_sys::Math::random() * 0.5 + 0.3,
                color: COLORS[color_idx],
                connections: 0,
            }
        }).collect();

        // 2. MOUSE EVENTS CON LEPTOS IDIOMÁTICO
        let mouse = Rc::new(RefCell::new((None::<f64>, None::<f64>)));

        window_event_listener(ev::mousemove, {
            let mouse = mouse.clone();
            move |e| {
                *mouse.borrow_mut() = (Some(e.client_x() as f64), Some(e.client_y() as f64));
            }
        });

        window_event_listener(ev::mouseleave, {
            let mouse = mouse.clone();
            move |_| {
                *mouse.borrow_mut() = (None, None);
            }
        });

        // 3. BUCLE DE ANIMACIÓN
        let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
        let g = f.clone();
        let mut frame_count = 0;

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            frame_count += 1;
            
            if is_mobile && frame_count % 2 != 0 {
                let _ = web_sys::window().unwrap().request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
                return;
            }

            ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            let mouse_pos = *mouse.borrow();

            for p in &mut particles { p.connections = 0; }

            for i in 0..particles.len() {
                let (px, py) = {
                    let p = &mut particles[i];
                    
                    if let (Some(mx), Some(my)) = mouse_pos {
                        let dx = mx - p.x;
                        let dy = my - p.y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist < 100.0 {
                            let force = (100.0 - dist) / 100.0;
                            p.x -= dx * force * 0.015;
                            p.y -= dy * force * 0.015;
                        }
                    }

                    p.x += p.speed_x;
                    p.y += p.speed_y;

                    if p.x < 0.0 || p.x > canvas.width() as f64 { p.speed_x *= -1.0; }
                    if p.y < 0.0 || p.y > canvas.height() as f64 { p.speed_y *= -1.0; }

                    ctx.begin_path();
                    let _ = ctx.arc(p.x, p.y, p.size, 0.0, std::f64::consts::PI * 2.0);
                    ctx.set_global_alpha(p.opacity);
                    ctx.set_fill_style_str(p.color);
                    ctx.fill();

                    (p.x, p.y)
                };

                let p_color = particles[i].color;
                
                for j in (i + 1)..particles.len() {
                    if particles[i].connections >= max_connections || particles[j].connections >= max_connections { continue; }
                    
                    let op = &particles[j];
                    let dx = px - op.x;
                    let dy = py - op.y;
                    let dist_sq = dx * dx + dy * dy;

                    if dist_sq < connection_dist * connection_dist {
                        let dist = dist_sq.sqrt();
                        ctx.begin_path();
                        let line_opacity = 0.15 * (1.0 - dist / connection_dist);
                        ctx.set_global_alpha(line_opacity); 
                        ctx.set_stroke_style_str(p_color); 
                        ctx.set_line_width(0.6);
                        ctx.move_to(px, py);
                        ctx.line_to(op.x, op.y);
                        ctx.stroke();

                        particles[i].connections += 1;
                        particles[j].connections += 1;
                    }
                }
            }

            ctx.set_global_alpha(1.0);
            let _ = web_sys::window().unwrap().request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref());
        }) as Box<dyn FnMut()>));

        // Arrancamos el bucle. Al no tener on_cleanup, esto vivirá felizmente el tiempo que la pestaña siga abierta.
        let _ = window.request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref());
    });
}