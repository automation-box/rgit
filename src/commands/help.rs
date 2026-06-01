use crate::errors::RgitError;
use std::io::Write;

pub fn handle<W: Write>(mut writer: W) -> Result<(), RgitError> {
    writeln!(writer, "Rgit is a mock implementation of git in Rust")?;
    writeln!(writer, "Usage: rgit [COMMAND]")?;
    Ok(())
}
