use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let file_path = Path::new("./read.txt");
    let mut file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(e) => panic!("Unable to open the file {}: {}", file_path.display(), e),
    };
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => println!(
            "\nLoaded File content succesfully {}: \n{}",
            file_path.display(),
            content
        ),
        Err(e) => panic!("Unable to read the file {} : {}", file_path.display(), e),
    }

    let reversed = content.chars().rev().collect::<String>();
    let out_path = Path::new("./output.txt");
    match fs::write(out_path, &reversed) {
        Ok(_) => println!(
            "\nSuccessfully wrote reversed contents to file {}: \n{}",
            out_path.display(),
            reversed
        ),
        Err(e) => println!("Unable to write contents on {}: {}", out_path.display(), e),
    }
}
