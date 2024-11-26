use rust_cargo_practice::file_ops::split;

fn main() {
    let file_name = "sample.txt".to_string();
    let ip_address = "127.0.0.1".to_string();
    match split::split_file(&file_name, &ip_address) {
        Ok(chunk) => {
            println!("{:?}", chunk.file_name);
        }
        Err(e) => {
            eprintln!("Error {}", e);
        }
    }
}
