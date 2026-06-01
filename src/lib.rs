use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// The distinct actions rgit can take.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Init,
    Help,
    None,
}

/// Represents the evaluated command-line state for the application.
#[derive(Debug, PartialEq, Eq)]
pub struct CliConfig {
    pub command: Command,
    pub positional_args: Vec<String>,
}

/// Top-level application errors.
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

/// Purely parses arguments into a configuration struct without side-effects.
pub fn parse_args<I>(args: I) -> CliConfig
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();

    // Skip executable path
    args.next();

    let mut command = Command::None;
    let mut positional_args = Vec::new();

    for arg in args {
        match arg.as_str() {
            "init" => command = Command::Init,
            "-h" | "--help" => command = Command::Help,
            _ => positional_args.push(arg),
        }
    }

    CliConfig {
        command,
        positional_args,
    }
}

/// Handles command execution and UI output.
///
/// `base_dir` is injected for deterministic behavior and easier testing.
pub fn execute_command<W>(
    command: &Command,
    base_dir: &Path,
    mut writer: W,
) -> Result<(), RgitError>
where
    W: Write,
{
    match command {
        Command::Init => {
            writeln!(writer, "Initializing repository...")?;

            let rgit_dir = base_dir.join(".rgit");

            ensure_directory_exists(&rgit_dir)?;
            ensure_directory_exists(&rgit_dir.join("objects"))?;
            ensure_directory_exists(&rgit_dir.join("refs"))?;

            let head_path = rgit_dir.join("HEAD");
            fs::write(&head_path, "ref: refs/heads/main\n").map_err(|e| {
                RgitError::StorageFailure {
                    path: head_path,
                    source: e,
                }
            })?;

            writeln!(writer, "Repository initialized successfully.")?;
        }

        Command::Help => {
            writeln!(writer, "Rgit is a mock implementation of git in Rust")?;
            writeln!(writer, "Usage: rgit [COMMAND]")?;
        }

        Command::None => {}
    }

    Ok(())
}

fn ensure_directory_exists(dir: &Path) -> Result<(), RgitError> {
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
