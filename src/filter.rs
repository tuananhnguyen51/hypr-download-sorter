use camino::Utf8Path;

#[must_use]
pub fn should_process(path: &Utf8Path) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8Path;

    #[test]
    fn accepts_normal_files() {
        assert!(should_process(Utf8Path::new("a.pdf")));
        assert!(should_process(Utf8Path::new("cat.png")));
        assert!(should_process(Utf8Path::new("movie.mp4")));
    }

    #[test]
    fn rejects_hidden_files() {
        assert!(!should_process(Utf8Path::new(".gitignore")));
        assert!(!should_process(Utf8Path::new(".hidden")));
    }

    #[test]
    fn rejects_temporary_files() {
        assert!(!should_process(Utf8Path::new("video.mp4.crdownload")));
        assert!(!should_process(Utf8Path::new("archive.zip.part")));
        assert!(!should_process(Utf8Path::new("tmp.tmp")));
        assert!(!should_process(Utf8Path::new("swap.swp")));
    }
}
