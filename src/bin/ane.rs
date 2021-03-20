use std::{fs, io::stdin};
fn main() {}
fn assignment2(read_path: &String) {
    let file_content = fs::read_to_string(read_path).expect("Something went wrong. File not read.");
    println!("File content read: {:?}", file_content);

    // copy ref value
    let mut write_filename = read_path.clone();
    // remove .txt
    for _ in 0..4 {
        write_filename.pop();
    }
    // reverse the name
    let mut write_path: String = write_filename.chars().rev().collect();
    write_path.push_str(&".txt");
    // reverse content
    let write_content = file_content.chars().rev().collect::<String>();
    // write file
    fs::write(write_path, write_content).expect("File write failed.");
}
fn assignment() {
    let v: Vec<&str> = vec!["alif", "bay", "pay", "tay"];

    let mut input_val = String::new();
    print!("Enter an int: ");
    let a = stdin().read_line(&mut input_val);
    let trimmed_ip = input_val.trim();
    let mut t = 1u64;

    // convert string to int
    match trimmed_ip.parse::<u64>() {
        Ok(i) => {
            println!("Your entered {}", i);
            t = i;
        }
        Err(..) => println!("Error... booommmmmm. It's not an int. YOU IDIOT!"),
    }

    // loop
    for i in v.iter() {
        for j in 0..t {
            println!("Vec value: {}, Ittr: {}", i, j + 1);
        }
    }
}
pub fn sabzi_walay() {
    let a = Vegetables::Aalu;

    match a {
        Vegetables::Aalu => println!("Aalu waalaay"),
        Vegetables::Bhindi => println!("Bhindi lelo"),
        _ => println!("Kuch bhi nh"),
    }
}

enum Vegetables {
    Aalu,
    Bhindi,
    Onion,
}

// impl Vegetables {

// }
