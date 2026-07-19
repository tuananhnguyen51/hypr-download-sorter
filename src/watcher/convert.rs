use camino::Utf8PathBuf;
use notify::{
    Event, EventKind,
    event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
};

use super::event::{FileEvent, FileEventKind};

/// Convert a `notify::Event` into one or more internal `FileEvent`s.
///
/// `notify` may report multiple paths for a single event (for example a rename),
/// so this function returns a vector of internal events.
#[must_use]
pub fn convert_event(event: Event) -> Vec<FileEvent> {
    let kind = convert_kind(&event.kind);

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
fn convert_kind(kind: &EventKind) -> FileEventKind {
    match kind {
        EventKind::Create(CreateKind::Any)
        | EventKind::Create(CreateKind::File)
        | EventKind::Create(CreateKind::Folder) => FileEventKind::Created,

        EventKind::Modify(ModifyKind::Any)
        | EventKind::Modify(ModifyKind::Data(_))
        | EventKind::Modify(ModifyKind::Metadata(_)) => FileEventKind::Modified,

        EventKind::Modify(ModifyKind::Name(
            RenameMode::Any | RenameMode::Both | RenameMode::From | RenameMode::To,
        )) => FileEventKind::Renamed,

        EventKind::Remove(RemoveKind::Any)
        | EventKind::Remove(RemoveKind::File)
        | EventKind::Remove(RemoveKind::Folder) => FileEventKind::Removed,

        _ => FileEventKind::Other,
    }
}
