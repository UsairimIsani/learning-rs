use std::{
    fs::File,
    io::{stdin, Read, Write},
    path::PathBuf,
};

fn main() {
    let mut file_name = String::new();

    stdin().read_line(&mut file_name).unwrap();

    let mut file = File::open(&mut file_name).expect("Couldn't Open File !");

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();
    //
    let content = content.chars().rev().collect::<String>();

    let file_name = PathBuf::from(file_name);

    let name = file_name.file_name().unwrap().to_str().unwrap();

    let name = name.trim().chars().rev().collect::<String>();
    let mut new_file = File::create(name).unwrap();
    new_file.write(&content.as_bytes()).unwrap();
}
