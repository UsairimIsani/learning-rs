use std::io;
fn main() {
    // println!("choose a color between rgb");

    // let t = vec!["red", "green", "blue"];

    // let mut num = String::new();

    // let input = io::stdin().read_line(&mut num);
    // // .expect("failed to read line");

    // match input {
    //     Ok(s) => println!("{}", s),
    //     Err(_) => println!("sdfs"),
    // }

    // let x = Some(34);

    // let x: Option<u8> = None;

    // match x {
    //     Some(s) => println!("{}", s),
    //     None => println!("Jlsdf"),
    // }

    // // if let Some(s) = x {
    // //     println!("{}", s);
    // // }

    // // let mut y = (0..6).map(|s| Some(s)).collect::<Vec<Option<i32>>>();
    // // y.push(None);

    // // println!("{:?}\n", y);

    // // while let Some(s) = &y.iter().next() {
    // //     println!("{:?}", s);
    // // }
    // let c = {
    //     "sdf";
    // };
    // match c {
    //     () => println!("klsjkdf"),
    // }
    // // Unit Type
    // let c = { "sdf" }; // String Slice
    //                    // let c = if 4 > 3 { "sdf" } else { 2 }; // String Slice
    // let c = if 4 > 3 { "sdf" } else { "j;adf" }; // String Slice
    // let c = loop {
    //     break "sdf";
    // }; // String Slice

    let z = 'alabel: loop {
        'second: loop {
            break 'alabel "asd";
        }
    };
    println!("{}", z);
}
