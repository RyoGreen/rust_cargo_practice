use std::fs::File;
use std::io::{Read, Result, Write};

const CHUNK_SIZE: usize = 1024;

pub fn split_file(input_file: &str) -> Result<()> {
    let mut file = File::open(input_file)?;
    let mut buffer = vec![0; CHUNK_SIZE];
    let mut chunk_index = 0;

    while let Ok(bytes_read) = file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let chunk_filename = format!("chunk_{}.txt", chunk_index);
        let mut chunk_file = File::create(&chunk_filename)?;
        chunk_file.write_all(&buffer[..bytes_read])?;
        chunk_index += 1;
    }

    Ok(())
}
