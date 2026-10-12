//! Documentação pública do `leptos-nebula`, feita com o `leptos-press`.
//!
//! O `build.rs` lê `content/`, `examples/` e `api/` e gera `press.rs` (páginas, rotas, nav);
//! aqui ficam só a raiz do app e os componentes específicos deste site.

use leptos::prelude::*;
use leptos_press::{ExampleBox, PressApp};

#[cfg(not(target_arch = "wasm32"))]
pub mod site_output;

/// Código gerado pelo `leptos-press-build`: `SITE`, uma função por página e `routes()`.
/// Os exemplos de `examples/` entram como módulos daqui (`#[path]`).
#[allow(clippy::all, unused)]
mod press {
    include!(concat!(env!("OUT_DIR"), "/press.rs"));
}

/// Usado pelo código gerado para `<DocExample />`: o exemplo vivo e, abaixo, o código-fonte.
pub fn doc_example(
    title: Option<&'static str>,
    column: bool,
    example: fn() -> AnyView,
    code: AnyView,
) -> AnyView {
    view! { <ExampleBox title=title column=column code=code>{example()}</ExampleBox> }.into_any()
}

#[component]
pub fn App() -> impl IntoView {
    view! { <PressApp site=&press::SITE routes=press::routes /> }
}

/// Documento HTML do SSG (`feature = "ssr"`): hidrata o mesmo `App` no navegador.
#[cfg(feature = "ssr")]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    use leptos_meta::MetaTags;

    view! {
        <!DOCTYPE html>
        <html lang="pt-BR">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <HydrationScripts options=options.clone() />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

#[cfg(feature = "csr")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn csr() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
