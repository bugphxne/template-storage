use std::path::{Component, Path, PathBuf};

pub fn safe_join(base: &Path, rel: &str) -> std::io::Result<PathBuf> {
    let rel_path = Path::new(rel);

    if rel_path.is_absolute() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "absolute path not allowed",
        ));
    }

    for comp in rel_path.components() {
        if let Component::ParentDir = comp {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "parent dir .. not allowed",
            ));
        }
    }

    Ok(base.join(rel_path))
}
