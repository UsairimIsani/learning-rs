mod broker;
mod client;
mod service;
mod chicken {

    pub mod hen {
        pub fn cl() {
            println!("CLUCK CLUCK! CL");
        }
        fn col() {
            println!("CLUCK CLUCK! COl");
        }
        pub fn doda() {
            self::col(); // current modules
        }
        pub fn tamata() {
            super::cluck() // parent module
        }
    }

    pub(crate) fn cluck() {
        println!("CLUCK CLUCK!"); // only visible in the lib
    }
}
fn hella() {
    println!("HellAA");
    pub fn kaddu() {
        println!("HellAA");
    }
    fn paratha() {
        println!("HellAA");
    }
}
use core::time;
use std::cell::RefCell;

use chicken::hen::cl;
fn main() {
    // cl();
    // chicken::hen::doda();
    // chicken::cluck();
    // crate::chicken::hen::cl(); // root module

    (0..).for_each(|x| {
        println!("{}", x);
        std::thread::sleep(std::time::Duration::from_secs(1));
    });
    let a = |x: i32, y: u32, z| 8;
    a(2, 5, (1, 2, 6u8));
    let a = RefCell::new(2);
}
// fn chicken<Closure>(s: Closure)
// where
//     Closure: Fn(i32) -> u32,
// {
//     s(2);
// }
