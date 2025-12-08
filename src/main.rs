mod CPU;
mod AddressingMode;
mod opcodes;

fn main() {
    println!("hello");
    println!("bello");

    let mut h11 = Hund{
        name: "vc".to_string(),
        size: 10,
        memory: [10; 0xFFFF]
    };
    let mut hund = Hund::new();

    for m in h11.memory {
        println!("{}", m)
    }
    testfn(&mut h11);
    testfn2(&h11)
}


fn testfn2(h: &Hund) {

}


fn testfn(h: &mut Hund) -> &Hund {
    h.size = 33;
    println!("{}", h.size);
    println!("{}", h.name);
    h
}


struct Hund{
    pub size: u8,
    pub name: String,
    memory: [u8; 0xFFFF]
}


impl Hund{

    pub fn new() -> Self{
        Hund{
            name: "vc".to_string(),
            size: 10,
            memory: [1; 0xFFFF]
        }
    }
}