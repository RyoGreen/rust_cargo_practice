use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Write};

fn merge_chunks(chunk_files: Vec<String>, output_file: &str) -> Result<()> {
    let mut output = OpenOptions::new()
        .create(true)
        .write(true)
        .open(output_file)?;

    for chunk_file in chunk_files {
        let mut chunk = File::open(chunk_file)?;
        let mut buffer = Vec::new();
        chunk.read_to_end(&mut buffer)?;
        output.write_all(&buffer)?;
    }

    Ok(())
}
