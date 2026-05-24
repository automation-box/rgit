use std::io::Write;
use std::{fs, path::PathBuf};

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

pub fn parse_args<I, W>(args: I, mut writer: W) -> CliConfig
where
    I: IntoIterator<Item = String>,
    W: Write,
{
    let mut args_iter = args.into_iter();
    let size_hint = args_iter.by_ref().size_hint().0;

    let mut positional_args = Vec::with_capacity(size_hint);
    let mut command = Command::None;

    args_iter.next(); // Skip executable path

    for arg in args_iter {
        match arg.as_str() {
            "init" => {
                writeln!(writer, "Initializing repository...").unwrap();
                command = Command::Init;
            }
            "-h" | "--help" => {
                writeln!(writer, "Rgit is mock implementation of git in Rust").unwrap();
                writeln!(writer, "Usage [FLAGS] [ARGS]").unwrap();
                command = Command::Help;
            }
            positional => {
                positional_args.push(positional.to_string());
            }
        }
    }

    CliConfig {
        command,
        positional_args,
    }
}

/// This function handles the actual execution. Keeping it separate means
/// we can pass it a custom directory when testing file system operations!
pub fn execute_command(command: &Command, base_dir: PathBuf) -> std::io::Result<()> {
    if let Command::Init = command {
        let rgit_dir = base_dir.join(".rgit");
        fs::create_dir(rgit_dir)?;
    }
    Ok(())
}
