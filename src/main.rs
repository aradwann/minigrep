// lets implement cargo run -- searchstring examplefilename.txt

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("searching for {}", query);
    println!("from the file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("should have been able to read the file");
    println!("with text:\n {contents}");

}
