mod svm;
mod nes;
mod CpuTest;

use std::fmt::{Display, Formatter};
macro_rules! log_it {
    // This pattern matches a single expression.
    // The `expr` fragment specifier indicates that we're looking for a valid Rust expression.
    ($var:expr) => {
        // We use the `stringify!` macro to turn the expression's code into a string.
        // We use the `println!` macro to print a formatted string.
        println!("{}: {:?}", stringify!($var), $var);
    };
}
fn main() {


    let x = 5;
    let y = &x; //set y to a reference to x

    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference y


    let mut cpu = CPU::new(); // create a CPU instance
    let program = vec![0xA9, 0x01, 0x8D, 0x00, 0x02]; // example program bytes

    let x = 10;
    let name = "Alice";

    // Macro calls:
    log_it!(x);
    log_it!(name);
    log_it!(x * 2);
}



















fn sing() {
    println!("la la la LA LA");
}
fn square(x: i32) -> i32 {
    if false{
        return 34;
    }
    x * x
}
fn add(x: i32, y: i32) -> i32 {
    x + y
}
pub fn my_method(func: impl Fn(i32) -> String) -> String {
    func(3232)
}
fn stringify(n: i32) -> String{
    n.to_string()
}
struct Person {
    name: String,
    age: i32,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
    }
}


struct Rectangle {
    x1: i32, y1: i32,
    x2: i32, y2: i32,
}

impl Rectangle {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self { x1, y1, x2, y2 }
    }

    pub fn x1(&self) -> i32 { self.x1 }
    pub fn y1(&self) -> i32 { self.y1 }
    pub fn x2(&self) -> i32 { self.x2 }
    pub fn y2(&self) -> i32 { self.y2 }

    pub fn length(&self) -> i32 {
        self.y2 - self.y1
    }

    pub fn width(&self)  -> i32 {
        self.x2 - self.x1
    }

    pub fn top_left(&self) -> (i32, i32) {
        (self.x1, self.y1)
    }

    pub fn bottom_right(&self) -> (i32, i32) {
        (self.x2, self.y2)
    }

    pub fn area(&self)  -> i32 {
        self.length() * self.width()
    }

    pub fn is_square(&self)  -> bool {
        self.width() == self.length()
    }
}

use std::fmt::*;
use crate::nes::CPU;
use crate::svm::dual_objective;

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {}), ({}, {})", self.x1, self.y2, self.x2, self.y2)
    }
}
