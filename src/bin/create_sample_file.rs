use std::{
    fs::File,
    io::{BufWriter, Write},
};

fn main() -> std::io::Result<()> {
    let file_name = "sample.txt";

    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);

    let target_size = 1024;

    let mut written = 0;
    let content = "This is a sample text for generating a large file. \n";

    while written < target_size {
        writer.write_all(content.as_bytes())?;
        written += content.len();
    }

    println!("File '{}' created with size {} bytes", file_name, written);
    Ok(())
}
