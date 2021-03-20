// Default Implementation
trait CookWithImp: std::fmt::Debug {
    fn cook(&self) {
        println!("I cook {:?}", self);
    }
}

#[derive(Debug)]
struct Kebab;
// impl Default for Poori {
//     fn default() -> Self {
//         Self { count: 5 }
//     }
// }

#[derive(Debug)]
struct Boti;

impl Kebab {
    fn new() -> Self {
        Self
    }
}

impl Boti {
    fn new() -> Self {
        Self
    }
}

// Basic Trait
trait Cook {
    fn cook(&self);
    // fn clean(&self) {}
}

impl Cook for Kebab {
    fn cook(&self) {
        println!("I cook Kebab");
        println!("I cook Kebab");
        println!("I cook Kebab");
        println!("I cook Kebab");
        println!("I cook Kebab");
        println!("I cook Kebab");
    }
}
impl Cook for Boti {
    fn cook(&self) {
        println!("I cook Boti");
    }
}
#[derive(Debug)]
struct Halwa;
impl Halwa {
    fn new() -> Self {
        Self
    }
}
impl CookWithImp for Halwa {}

#[derive(Debug)]
struct Poori {
    count: u32,
}
impl Poori {
    fn new() -> Self {
        Self { count: 0 }
    }
}
impl CookWithImp for Poori {}

use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

impl Add for Poori {
    type Output = Poori;

    fn add(self, rhs: Self) -> Self::Output {
        let count = self.count + rhs.count;
        Poori { count }
    }
}
impl Sub for Poori {
    type Output = Poori;

    fn sub(self, rhs: Self) -> Self::Output {
        let count = self.count - rhs.count;
        Poori { count }
    }
}

fn main() {
    let b = BBQ { dish: 8 };
    let c = BBQ { dish: "str" };
    let d = BBQ { dish: CUstom };
}
use std::fmt::Debug;

#[derive(Debug)]
struct CUstom;
#[derive(Debug)]
struct BBQ<T, F, G>
where
    T: Debug,
{
    dish: T,
    k: F,
    g: G,
}

impl<F> BBQ<F>
where
    F: Debug,
{
    fn cook(&self) {
        println!("I cook {:?}", self.dish);
    }
    fn new(dish: F) -> Self {
        println!("");
        Self { dish }
    }
}
// impl BBQ<i32> {
//     fn cook_i32(&self) {
//         println!("I cook {:?} sdfasdfkljll", self.dish);
//     }
// }
// impl<T: Debug> Drop for BBQ<T> {
//     fn drop(&mut self) {
//         println!("I AM BEING TAKEN AWAY FROM MEMORY!!!!!!!")
//     }
// }

// enum List {
//     None,
//     Next(List),
// }
// enum List {
//     None,
//     Next(Box<List>),
// }

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};
// use std::rc::Rc;

// fn main() {
//     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
//     let b = Cons(3, Rc::clone(&a));
//     let c = Cons(4, Rc::clone(&a));
// }
