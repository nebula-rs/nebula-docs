//! Copia `llms.txt`, `llms-full.txt` e as cópias `.md` para uma pasta (hook do Trunk no modo CSR).
//! Uso: `press-static <pasta>`.

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let Some(dest) = std::env::args().nth(1) else {
        eprintln!("uso: press-static <pasta>");
        std::process::exit(2);
    };
    if let Err(e) = docs_app::site_output::copy_press_static(std::path::Path::new(&dest)) {
        eprintln!("press-static: {e}");
        std::process::exit(1);
    }
}

// Só existe para o `cargo check --target wasm32` do workspace não quebrar.
#[cfg(target_arch = "wasm32")]
fn main() {}
