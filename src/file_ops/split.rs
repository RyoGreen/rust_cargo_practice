use anyhow::{Context, Result};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

const CHUNK_SIZE: usize = 1024;
const OUTPUT_DIR: &str = "output_chunks";

pub fn split_file(input_file: &str) -> Result<()> {
    let input_path = Path::new(input_file);
    let file_extension = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .context("Failed to determine file extension")?;

    if !Path::new(OUTPUT_DIR).exists() {
        fs::create_dir_all(OUTPUT_DIR)?;
    }

    let mut file = File::open(input_file)?;
    let mut buffer = vec![0; CHUNK_SIZE];
    let mut chunk_index = 0;
    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let chunk_filename = format!("{}/chunk_{}.{}", OUTPUT_DIR, chunk_index, file_extension);
        let mut chunk_file = File::create(&chunk_filename)?;
        chunk_file.write_all(&buffer[..bytes_read])?;
        chunk_index += 1;
    }
    Ok(())
}
