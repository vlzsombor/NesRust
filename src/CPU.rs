use std::collections::HashMap;
use std::ops::AddAssign;
use bitflags::{bitflags, Flags};
use crate::AddressingMode::AddressingMode;
use crate::opcodes;


bitflags! {
    /// # Status Register (P) http://wiki.nesdev.com/w/index.php/Status_flags
    ///
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    ///
    pub struct CpuFlags: u8 {
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIV           = 0b10000000;
    }
}
pub struct CPU{
    pub register_a: u8,
    pub status: CpuFlags,
    pub register_x: u8,
    pub register_y: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    memory: [u8; 0xFFFF]
}

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;
trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16
    {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16)
    {
        let hi = (data >> 8) as u8;
        let lo = (data &0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
}
impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    fn mem_write(&mut self, addr: u16, data: u8){
        self.memory[addr as usize] = data;
    }
}
impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: CpuFlags::from_bits_truncate(0b100100),
            stack_pointer: STACK_RESET,
            program_counter: 0,
            register_x: 0,
            register_y: 0,
            memory: [0; 0xffff]
        }
    }

    //Memory
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8){
        self.memory[addr as usize] = data;
    }

    pub fn load_and_run(&mut self, program: Vec<u8>){
        self.load(program);
//        self.reset();
        self.program_counter = self.mem_read_u16(0xFFFC);
        self.run();
    }

    pub fn reset(&mut self){
        self.register_a = 0;
        self.register_x = 0;
        self.status = CpuFlags::from_bits_truncate(0b100100);
        self.stack_pointer = STACK_RESET;
        self.program_counter = self.mem_read_u16(0xfffc);
    }

    pub fn load(&mut self, program: Vec<u8>){
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xfffc, 0x8000);
    }


    pub fn run(&mut self){
        let ref oc: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;
        loop{
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;
            let program_counter_state = self.program_counter;

            let opcode = oc.get(&code).expect(&format!("OpCode {:x} is not recognized", code));


            match code{
                0x00 => return,
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 =>{
                    self.lda(&opcode.mode);
                }
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
//                    self.program_counter += 1;
                }
                _ => todo!()
            }

            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }

    fn lda(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);
        self.set_register_a(value);
    }

    fn set_register_a(&mut self, value: u8){
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn tax(&mut self){
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x)
    }
    fn update_zero_and_negative_flags(&mut self, result: u8){
        if result == 0 {
            self.status.insert(CpuFlags::ZERO)
        } else {
            self.status.remove(CpuFlags::ZERO)
        }
        if result & 0b1000_0000 != 0 {
            self.status.insert(CpuFlags::NEGATIV);
        } else {
            self.status.remove(CpuFlags::NEGATIV);
        }
    }
    fn inx(&mut self){
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x)
    }

    fn sta(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16{
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            },
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            },
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_y);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }


}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status.bits() & 0b0000_0010 == 0);
//        assert!(cpu.status.contains(CpuFlags::ZERO));
        assert_eq!(cpu.status.bits() & 0b1000_0000, 0);
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A,0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }
    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa,0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
}