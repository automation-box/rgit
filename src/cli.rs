#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Init,
    Help,
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CliConfig {
    pub command: Command,
    pub positional_args: Vec<String>,
}

pub fn parse_args<I>(args: I) -> CliConfig
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    args.next(); // Skip binary path

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
