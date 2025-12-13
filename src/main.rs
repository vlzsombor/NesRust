use std::marker::PhantomData;

mod CPU;
mod AddressingMode;
mod opcodes;

use std::time::Duration;
use futures::future::join_all;
use futures_util::FutureExt;
use tokio::time::sleep;

async fn example_async() -> String {
    sleep(Duration::from_secs(11)).await;
    "Daten geladen".to_string()
}

async fn example_async2() -> String {
    sleep(Duration::from_secs(4)).await;
    "Daten geladen2".to_string()
}

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
    // Variante 3: Vec mit join_all
    let foo = 1;
    let bar = 2;
    either!(foo == bar => println!("it is true"); println!("it is false"));
    either!(foo != bar => println!("it is true"); println!("it is false"));
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