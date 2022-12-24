use anyhow::Result;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn read_text_file_internal(path: &Path) -> io::Result<String> {
    let mut content = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn write_text_file_internal(path: &Path, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    write!(&mut file, "{}", content)?;
    Ok(())
}

// Convenience method to read from a text file
pub fn read_text_file(path: &Path) -> Result<String> {
    Ok(read_text_file_internal(path)?)
}

// Convenience method to write to a text file
pub fn write_text_file(path: &Path, content: &str) -> Result<()> {
    Ok(write_text_file_internal(path, content)?)
}
