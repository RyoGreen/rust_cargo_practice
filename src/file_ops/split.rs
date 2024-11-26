use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
const CHUNK_SIZE: usize = 1024;
const OUTPUT_DIR: &str = "output_chunks";

pub struct FileInfo {
    pub hash: String,
    pub file_name: String,
    pub updated_use_address: String,
    pub file_chunk: Vec<FileChunk>,
}

pub struct FileChunk {
    pub file_name: String,
    pub chunk_hash: String,
}

pub fn split_file(input_file: &str, updated_user_address: &str) -> Result<FileInfo> {
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
    let mut chunks = Vec::new();
    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let chunk_filename = format!("{}/chunk_{}.{}", OUTPUT_DIR, chunk_index, file_extension);
        let mut chunk_file = File::create(&chunk_filename)?;
        chunk_file.write_all(&buffer[..bytes_read])?;

        let chunk_hash = calculate_hash(&buffer[..bytes_read]);

        chunks.push(FileChunk {
            file_name: chunk_filename.clone(),
            chunk_hash,
        });

        chunk_index += 1;
    }
    Ok(FileInfo {
        hash: calculate_hash(&fs::read(input_path)?),
        file_name: input_file.to_string(),
        updated_use_address: updated_user_address.to_string(),
        file_chunk: chunks,
    })
}

fn calculate_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
