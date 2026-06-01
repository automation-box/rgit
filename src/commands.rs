mod help;
mod init;

use crate::cli::Command;
use crate::errors::RgitError;
use std::io::Write;
use std::path::Path;

pub fn execute<W: Write>(command: &Command, base_dir: &Path, writer: W) -> Result<(), RgitError> {
    match command {
        Command::Init => init::handle(base_dir, writer),
        Command::Help => help::handle(writer),
        Command::None => Ok(()),
    }
}
