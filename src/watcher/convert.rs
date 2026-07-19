use camino::Utf8PathBuf;
use notify::{
    Event, EventKind,
    event::{CreateKind, ModifyKind},
};

use super::event::{FileEvent, FileEventKind};

/// Convert a `notify::Event` into one or more internal `FileEvent`s.
///
/// `notify` may report multiple paths for a single event (for example a rename),
/// so this function returns a vector of internal events.
#[must_use]
pub fn convert_event(event: Event) -> Vec<FileEvent> {
    let Some(kind) = convert_kind(&event.kind) else {
        return Vec::new();
    };

    event
        .paths
        .into_iter()
        .filter_map(|path| {
            if path.is_dir() {
                return None;
            }

            Utf8PathBuf::from_path_buf(path)
                .ok()
                .map(|path| FileEvent::new(path, kind))
        })
        .collect()
}
/// Convert `notify::EventKind` into our simplified event type.
#[must_use]
fn convert_kind(kind: &EventKind) -> Option<FileEventKind> {
    match kind {
        EventKind::Create(CreateKind::File) => Some(FileEventKind::Created),

        EventKind::Modify(ModifyKind::Data(_)) => Some(FileEventKind::Modified),

        _ => None,
    }
}
