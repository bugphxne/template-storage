use std::path::{Component, Path, PathBuf};

pub fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}

pub fn path_to_forward_slash(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

pub fn get_relative_path(base: &Path, full: &Path) -> String {
    if let Ok(rel) = full.strip_prefix(base) {
        path_to_forward_slash(rel)
    } else {
        path_to_forward_slash(full)
    }
}

pub fn safe_join(base: &Path, rel: &str) -> std::io::Result<PathBuf> {
    let normalized = normalize_path(rel);
    let rel_path = Path::new(&normalized);

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
