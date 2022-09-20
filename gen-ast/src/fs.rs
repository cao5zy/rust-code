use std::io::{Result};
use std::fs::File;
use std::io::Write;

pub fn write_file(path: &str, content: &str)->Result<()> {

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
