use leptos::prelude::*;
use leptos_use::use_document_visibility;
use leptos_use::{use_favicon_with_options, UseFaviconOptions};

pub fn favicon_dinamico() {
    // 1. Obtenemos la señal reactiva
    let visibility = use_document_visibility();
    
    // 2. Preparamos el controlador del favicon (asumiendo que los iconos están en la raíz /)
    let (_, set_icon) = use_favicon_with_options(
        UseFaviconOptions::default().base_url("/")
    );

    // 3. El Efecto reactivo: Se ejecutará solito cada vez que 'visibility' cambie
    Effect::new(move |_| {
        if visibility.get() == web_sys::VisibilityState::Visible {
            // El usuario regresó o está viendo la página
            set_icon.set(Some("public/favicon.ico".into()));
            
            // Opcional: También puedes cambiar el título aquí si quieres
            document().set_title("Horacio | Portafolio"); 
        } else {
            // El usuario se fue a otra pestaña o minimizó
            set_icon.set(Some("public/favicon-inactive.ico".into()));
            
            // Opcional:
            document().set_title("¡Vuelve pronto!"); 
        }
    });
}