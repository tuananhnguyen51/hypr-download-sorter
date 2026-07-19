use camino::Utf8Path;

#[must_use]
pub fn should_process(path: &Utf8Path) -> bool {
    if !path.is_file() {
        return false;
    }

    let Some(name) = path.file_name() else {
        return false;
    };

    if name.starts_with('.') {
        return false;
    }

    !matches!(
        path.extension(),
        Some("tmp" | "part" | "crdownload" | "swp")
    )
}
