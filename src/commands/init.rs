use crate::errors::RgitError;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn handle<W: Write>(base_dir: &Path, mut writer: W) -> Result<(), RgitError> {
    writeln!(writer, "Initializing repository...")?;

    let rgit_dir = base_dir.join(".rgit");
    ensure_dir_exists(&rgit_dir)?;
    ensure_dir_exists(&rgit_dir.join("objects"))?;
    ensure_dir_exists(&rgit_dir.join("refs"))?;

    let head_path = rgit_dir.join("HEAD");
    fs::write(&head_path, "ref: refs/heads/main\n").map_err(|e| RgitError::StorageFailure {
        path: head_path,
        source: e,
    })?;

    writeln!(writer, "Repository initialized successfully.")?;
    Ok(())
}

fn ensure_dir_exists(dir: &Path) -> Result<(), RgitError> {
    match fs::create_dir(dir) {
        Ok(_) => Ok(()),
        Err(err) => match err.kind() {
            io::ErrorKind::AlreadyExists => Err(RgitError::DirectoryAlreadyExists),
            io::ErrorKind::PermissionDenied => Err(RgitError::PermissionDenied),
            _ => Err(RgitError::StorageFailure {
                path: dir.to_path_buf(),
                source: err,
            }),
        },
    }
}
