use leptos::prelude::*;
use leptos::task::spawn_local;
use std::time::Duration;

#[component]
pub fn Loading() -> impl IntoView {
    let messages =vec![
        "INICIALIZANDO....",
        "CARGANDO MODULOS...",
        "CALIBRANDO....",
        "ACCESO CONSEDIDO." 
    ];

    let (msg, set_msg) = signal("INICIALIZANDO....".to_string());
    let (is_visible, set_is_visible) = signal(false);

    spawn_local(async move {
        for i in messages {
            set_msg.set(i.to_string());
            gloo_timers::future::sleep(Duration::from_millis(450)).await;
        }

        //temporisador para acceder cuando se llega al ultimo mensaje
        gloo_timers::future::sleep(Duration::from_millis(800)).await;

        set_is_visible(true);
    });

    view! {
        <div 
            class ="fixed inset-0 bg-void z-[99999] flex flex-col items-center justify-center transition-all duration-1000"
            class:hidden= is_visible
            style:opacity= move || if is_visible.get(){"0"} else {"1"}
            style:visibility= move || if is_visible.get(){"hidden"} else {"visible"}
        >
            //Icono de cristal que gira mientras carga
            <div class="w-20 h-20 relative animate-spin-slow ">
                //superior
                <div class="absolute inset-0 border-2 border-transparent border-t-aurora-purple border-r-aurora-blue rotate-45 animate-pulse-loader"></div>
                //inferior
                <div class="absolute inset-0 border-2 border-transparent border-b-aurora-green border-l-aurora-purple-light -rotate-45 animate-pulse-loader" style="animation-delay: -1s;"></div>
            </div>

            //barra de carga
            <div class="w-[200px] h-[2px] bg-gray-deep mt-6 relative overflow-hidden rounded-sm"> 
                <div class="absolute top-0 left-[-100%] w-full h-full bg-gradient-to-r from-aurora-purple via-aurora-blue to aurora-green animate-load-fill"></div>
            </div>

            //Texto cambiante 
            <p class="mt-6 font-mono text-[11px] tracking-[3px] text-gray-light">
                {move || msg.get()}
            </p>
        </div>
    }
} 