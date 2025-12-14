use std::marker::PhantomData;

mod CPU;
mod AddressingMode;
mod opcodes;

use std::time::Duration;
use futures::future::join_all;
use futures_util::FutureExt;
use tokio::time::sleep;

use tokio;
macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        } else {
            $false_expr
        }
    }
}

#[tokio::main]
async fn main() {
    let mut b = Bird{
        size: 2,
        weight: 32
    };

    test3(&mut b.weight);
    println!("{}", b.weight)
}
fn test3(v: &mut u8) -> u8 {
    *v = 3;
    *v
}
fn testfn2(mut f :Bird) {
    testfn(&mut f);
}

fn testfn(f : &mut impl Flyable){
    Bird::fly();
    f.flySelf();
}

struct Bird{
    weight: u8,
    size: u16
}

struct Plane{
    weight: u8,
    plane: u32
}
trait Flyable {
    fn fly() -> u8;

    fn flySelf(&self) -> u8;
    const DEFAULT: u32 = 0;
}


impl Flyable for Bird {
    fn fly() -> u8 {
        println!("im flying lol");
        3
    }

    fn flySelf(&self) -> u8 {
        println!("im flying lol self");
        3
    }
}
impl Flyable for Plane{
    fn fly() -> u8 {
        println!("plane");
        44
    }

    fn flySelf(&self) -> u8 {
        println!("plane self");
        44
    }
}