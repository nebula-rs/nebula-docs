//! Gera `$OUT_DIR/press.rs` e `$OUT_DIR/press-static/` a partir de `content/`.

#[path = "build/components.rs"]
mod components;

use components::{DocApi, DocExample};
use leptos_press_build::Site;

fn main() {
    println!("cargo:rerun-if-changed=build/components.rs");
    println!("cargo:rerun-if-changed=examples");
    println!("cargo:rerun-if-changed=api");

    // Onde os arquivos estáticos (llms*.txt, .md) ficam, para o `press-static` e o bin do SSG.
    let out = std::env::var("OUT_DIR").unwrap_or_default();
    println!("cargo:rustc-env=PRESS_STATIC_DIR={out}/press-static");

    Site::new("content")
        .title("leptos-nebula")
        .description("Componentes Leptos com temas por tokens — o que o Quasar é para o Vue.")
        .nav([("Guia", "/guide/"), ("Componentes", "/components/")])
        .group("guide", "Guia")
        .group("components", "Componentes")
        .sidebar_from_folders()
        .component(DocExample)
        .component(DocApi)
        .build()
        .unwrap_or_else(|e| panic!("{e}"));
}
