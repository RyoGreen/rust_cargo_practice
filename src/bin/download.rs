use rust_cargo_practice::file_ops::merge;

fn main() {
    let chunk_files = vec!["chunk_0.txt".to_string(), "chunk_1.txt".to_string()];
    let output_file = "merged.txt";

    match merge::merge_chunks(chunk_files, output_file) {
        Ok(_) => println!("Successfully merged files into {}", output_file),
        Err(e) => eprintln!("Error {}", e),
    }
}
