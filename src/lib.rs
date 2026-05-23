use std::io::Write;

/// Represents the evaluated command-line state for the application.
#[derive(Debug, PartialEq, Eq)]
pub struct CliConfig {
    pub positional_args: Vec<String>,
}

/// Parses command-line arguments and handles immediate side-effects like printing help.
///
/// # Arguments
/// * `args` - An iterator yielding the command-line tokens (including the binary name).
/// * `writer` - Any output stream implementing `Write` where command feedback is sent.
pub fn parse_args<I, W>(args: I, mut writer: W) -> CliConfig
where
    I: IntoIterator<Item = String>,
    W: Write,
{
    let mut args_iter = args.into_iter();

    // Hint to the vector how much memory to allocate up front to minimize reallocations
    let size_hint = args_iter.by_ref().size_hint().0;
    let mut positional_args = Vec::with_capacity(size_hint);

    // Skip the executable path (always the first element in CLI args)
    args_iter.next();

    for arg in args_iter {
        match arg.as_str() {
            "init" => {
                writeln!(writer, "Initializing repository...").unwrap();
            }
            "-h" | "--help" => {
                writeln!(writer, "Rgit is mock implementation of git in Rust").unwrap();
                writeln!(writer, "Usage [FLAGS] [ARGS]").unwrap();
            }
            positional => {
                positional_args.push(positional.to_string());
            }
        }
    }

    CliConfig { positional_args }
}
