use std::io::{self, Write};
use std::path::PathBuf;
use std::{env, fs};

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
    let mut args = args.into_iter();
    args.next(); // Skip executable path

    let mut command = Command::None;
    let mut positional_args = Vec::new();

    // Single-pass parse loop
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

/// Handles UI feedback and actual execution.
/// Accepting a writer allows us to test the console output of commands if needed
pub fn execute_command<W>(command: &Command, mut writer: W) -> io::Result<()>
where
    W: Write,
{
    let base_dir = env::current_dir().unwrap_or_default();
    match command {
        Command::Init => {
            writeln!(writer, "Initializing repository...")?;
            let rgit_dir = base_dir.join(".rgit");

            fs::create_dir(&rgit_dir)?;
            fs::create_dir(&rgit_dir.join("objects"))?;
            fs::create_dir(&rgit_dir.join("refs"))?;
        }
        Command::Help => {
            writeln!(writer, "Rgit is a mock implementation of git in Rust")?;
            writeln!(writer, "Usage [FLAGS] [ARGS]")?;
        }
        Command::None => {}
    }
    Ok(())
}
