//! Binário do SSG (feature `ssr`): gera um HTML por rota, copia os arquivos estáticos e termina.
//! Lê `[package.metadata.leptos]` do `Cargo.toml` do app; os caminhos (`site-root`) são relativos
//! à raiz do workspace, como no `cargo leptos`, então o processo muda para lá antes de começar.

use docs_app::{shell, site_output};
use leptos::prelude::*;
use leptos_axum::generate_route_list_with_ssg;

#[tokio::main]
async fn main() {
    let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    std::env::set_current_dir(&workspace).expect("entrar na raiz do workspace");
    let conf =
        get_configuration(Some("apps/docs/Cargo.toml")).expect("ler [package.metadata.leptos]");
    let options = conf.leptos_options;
    let site_root = std::path::PathBuf::from(options.site_root.as_ref());

    let (_routes, generator) = generate_route_list_with_ssg({
        let options = options.clone();
        move || shell(options.clone())
    });
    generator.generate(&options).await;

    site_output::directory_index(&site_root).expect("reorganizar as páginas em pastas");
    site_output::copy_press_static(&site_root).expect("copiar llms.txt e as cópias .md");
    println!("site estático em {}", site_root.display());
}
