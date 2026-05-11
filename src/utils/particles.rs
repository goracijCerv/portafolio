use leptos::prelude::*;
use leptos::ev;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{ JsCast};
use web_sys::CanvasRenderingContext2d;

const DARK_COLORS: [&str; 4] = [
    "#c77dff", // purple-glow
    "#4cc9f0", // blue-light
    "#3bf4fb", // green-light
    "#9d4edd", // purple-light (oscuro)
];

const LIGHT_COLORS: [&str; 4] = [
    "#7b2cbf", // purple
    "#2563eb", // blue-light (claro)
    "#0f766e", // green-light (claro)
    "#5a189a", // purple-light (claro)
];

struct Particle {
    x: f64,
    y: f64,
    size: f64,
    speed_x: f64,
    speed_y: f64,
    opacity: f64,
    color_idx: usize ,
    connections: usize,
}

pub fn init_particles(canvas_ref: NodeRef<leptos::html::Canvas>) {
    canvas_ref.on_load(move |canvas| {
        let window = web_sys::window().unwrap();
        let ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();

        canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
        canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

        //En caso de resize (cambia el tamaño de la ventana)
        window_event_listener(ev::resize, {
            let canvas = canvas.clone();
            move |_| {
                if let Some(w) = web_sys::window() {
                    canvas.set_width(w.inner_width().unwrap().as_f64().unwrap() as u32);
                    canvas.set_height(w.inner_height().unwrap().as_f64().unwrap() as u32);
                }
            }
        });

        let is_mobile = canvas.width() < 768;
        let count = if is_mobile {20} else {60};

        let particles: Vec<Particle> = (0..count).map(|i| {
            Particle {
                x: js_sys::Math::random() * canvas.width() as f64,
                y: js_sys::Math::random() * canvas.height() as f64,
                size: js_sys::Math::random() * 2.0 + 1.0,
                speed_x: (js_sys::Math::random() - 0.5) * 0.8,
                speed_y: (js_sys::Math::random() - 0.5) * 0.8,
                opacity: js_sys::Math::random() * 0.6 + 0.4,
                color_idx: i % 4,
                connections: 0
            }
        }).collect();

        let mouse  = Rc::new(RefCell::new((None::<f64>, None::<f64>)));
        window_event_listener(ev::mousemove, {
            let mouse = mouse.clone();
            move |e| {*mouse.borrow_mut() = (Some(e.client_x() as f64), Some(e.client_y() as f64));}
        });
        window_event_listener(ev::mouseleave, {
            let mouse = mouse.clone();
            move |_| { *mouse.borrow_mut() = (None,None);}
        });

        render_loop(canvas, ctx, particles, mouse, is_mobile);
    });
}

fn render_loop(canvas: web_sys::HtmlCanvasElement, ctx:CanvasRenderingContext2d, mut particles: Vec<Particle>, mouse: Rc<RefCell<(Option<f64>, Option<f64>)>>, is_mobile: bool) {
    request_animation_frame(move || {
        let connection_dist = if is_mobile {90.0} else {120.0};
        let max_connections = 3;

        let is_dark = web_sys::window()
            .unwrap()
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .map(|m| m.matches())
            .unwrap_or(true);

        let current_colors = if is_dark { DARK_COLORS } else { LIGHT_COLORS };
        
        ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        let mouse_pos = *mouse.borrow();

        for p in &mut particles { p.connections = 0; }

        for i in 0..particles.len() {
          let(px,py) = {
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
            ctx.set_fill_style_str(current_colors[p.color_idx]);
            ctx.fill();
            (p.x, p.y)
          };

          let p_color = current_colors[particles[i].color_idx];
          for j in (i+1)..particles.len() {
             if particles[i].connections >= max_connections || particles[j].connections >= max_connections {continue;}
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
                particles[i].connections += 1 ; 
                particles[j].connections += 1;
             }
          }
        }
        
        ctx.set_global_alpha(1.0);
        render_loop(canvas, ctx, particles, mouse, is_mobile);
    }); 
}