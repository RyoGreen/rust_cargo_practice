use rust_cargo_practice::file_ops::split;

fn main() {
    let file_name = "sample.txt".to_string();
    match split::split_file(&file_name) {
        Ok(chunk) => {
            println!("{:?}", chunk);
        }
        Err(e) => {
            eprintln!("Error {}", e);
        }
    }
}
