//! Pós-processamento da saída estática (só no host).

use std::fs;
use std::io;
use std::path::Path;

/// Copia `llms.txt`, `llms-full.txt` e as cópias `.md` (geradas pelo `build.rs`) para `dest`.
pub fn copy_press_static(dest: &Path) -> io::Result<()> {
    copy_dir(Path::new(env!("PRESS_STATIC_DIR")), dest)
}

fn copy_dir(from: &Path, to: &Path) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

/// O gerador estático escreve `guide/start.html`; um servidor de arquivos comum só resolve
/// `/guide/start` se houver `guide/start/index.html`. Move cada `x.html` (≠ `index.html`) para
/// `x/index.html`. `pkg/` (JS/wasm) fica como está.
pub fn directory_index(root: &Path) -> io::Result<()> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            if entry.file_name() != "pkg" {
                directory_index(&path)?;
            }
            continue;
        }
        let is_page = path.extension().is_some_and(|e| e == "html")
            && path.file_stem().is_some_and(|s| s != "index");
        if is_page {
            let dir = path.with_extension("");
            fs::create_dir_all(&dir)?;
            fs::rename(&path, dir.join("index.html"))?;
        }
    }
    Ok(())
}
