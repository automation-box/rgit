use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

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

/// Purely parses arguments into a configuration struct without side-effects.
pub fn parse_args<I>(args: I) -> CliConfig
where
    I: IntoIterator<Item = String>,
{
    let mut args_iter = args.into_iter();
    args_iter.next(); // Skip executable path

    let mut positional_args = Vec::new();
    let mut command = Command::None;

    for arg in args_iter {
        match arg.as_str() {
            "init" => {
                command = Command::Init;
            }
            "-h" | "--help" => {
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

/// Handles UI feedback and actual execution.
/// Accepting a writer allows us to test the console output of commands if needed!
pub fn execute_command<W>(command: &Command, base_dir: PathBuf, mut writer: W) -> io::Result<()>
where
    W: Write,
{
    match command {
        Command::Init => {
            writeln!(writer, "Initializing repository...")?;
            let rgit_dir = base_dir.join(".rgit");
            fs::create_dir(rgit_dir)?;
        }
        Command::Help => {
            writeln!(writer, "Rgit is a mock implementation of git in Rust")?;
            writeln!(writer, "Usage [FLAGS] [ARGS]")?;
        }
        Command::None => {}
    }
    Ok(())
}
