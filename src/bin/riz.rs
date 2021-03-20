use std::io;
fn main() {
    let a = String::from("Rizwan");
    let b = String::from("Usairim");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read");

    println!("");
    let v = vec![a, b, guess];
    for i in &v {
        println!("{}", i);
    }
}
// use std::io;
// fn main(){
//     println!("choose a color between rgb");
//     let t= vec!["red", "green", "blue"];
//     let mut num= String::new();
//     io::stdin()
//      .read_line( &mut num)
//      .expect("failed to read line");
//      Ok(_)=>{
//          println!("you entered {}", num);
//      }
//      Err(..) => println!("you failed");

//     }
//     }
