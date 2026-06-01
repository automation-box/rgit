use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum RgitError {
    #[error("directory already exists")]
    DirectoryAlreadyExists,

    #[error("not enough permissions")]
    PermissionDenied,

    #[error("storage failure at {path:?}")]
    StorageFailure {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    #[error(transparent)]
    Io(#[from] io::Error),
}
