use std::marker::PhantomData;

mod CPU;
mod AddressingMode;
mod opcodes;

fn main() {
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