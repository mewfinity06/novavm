pub mod fetch;
pub mod opcode;
pub mod syscall;

use fetch::Fetch;
use opcode::OpCode;

#[derive(Debug, Copy, Clone)]
pub enum Register {
    A,
    B,
    C,
    M,
    SP,
    PC,
    FLAGS,
    RegisterCount,
}

pub struct Machine {
    registers: [u16; Self::REGISTER_COUNT],
    data: [u8; Self::DATA_LENGTH],
    stack: Vec<u8>,
    pub halt: bool,
}

impl Machine {
    const REGISTER_COUNT: usize = Register::RegisterCount as usize;
    const DATA_LENGTH: usize = 2048;

    pub fn new() -> Self {
        Self {
            registers: [0; Self::REGISTER_COUNT],
            data: [0; Self::DATA_LENGTH],
            stack: Vec::with_capacity(Self::DATA_LENGTH),
            halt: false,
        }
    }

    pub fn set_data(&mut self, new_data: &[u8]) {
        if new_data.len() > self.data.len() {
            panic!("Data length exceeds memory capacity");
        }
        self.data[..new_data.len()].copy_from_slice(new_data);
    }

    fn fetch<'a, T>(&mut self) -> Result<T, String>
    where
        T: Fetch<'a>,
    {
        let pc = self.registers[Register::PC as usize] as usize;
        let d_point = self.data[pc];
        let v: T = T::try_from(d_point).map_err(|_| format!("could not fetch"))?;
        self.registers[Register::PC as usize] += 1;
        Ok(v)
    }

    pub fn print_state(&self) {
        let [a, b, c, m, sp, pc, flags] = self.registers;
        println!("|-----------------------------------------|");
        println!("| A  {a:04X} | B  {b:04X} | C     {c:04X} | M {m:04X} |");
        println!("| SP {sp:04X} | PC {pc:04X} | FLAGS {flags:04X} |");
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.registers[Register::PC as usize] as usize >= self.data.len() {
            self.halt = true;
        }

        let op: OpCode = self.fetch()?;

        match op {
            OpCode::HALT => self.handle_halt(),
            OpCode::NOP => self.handle_nop(),
            OpCode::ADD => self.handle_add()?,
            OpCode::SUB => self.handle_sub()?,
            OpCode::MUL => self.handle_mul()?,
            OpCode::DIV => self.handle_div()?,
            OpCode::POP => self.handle_pop()?,
            OpCode::PUSH => self.handle_push()?,
        }
        Ok(())
    }

    fn handle_halt(&mut self) {
        println!("| OpCode HALT");
        self.halt = true;
    }

    /// Should this function be printed every time?
    fn handle_nop(&self) {
        println!("| OpCode NOP");
    }

    fn handle_add(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a + b;
        println!("| ADD: Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", r, a, b, result);
        self.registers[r as usize] = result;

        Ok(())
    }

    fn handle_sub(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a - b;
        println!("| SUB: Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", r, a, b, result);
        self.registers[r as usize] = result;

        Ok(())
    }

    fn handle_mul(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a * b;
        println!("| MUL: Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", r, a, b, result);
        self.registers[r as usize] = result;

        Ok(())
    }

    fn handle_div(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a / b;
        println!("| DIV: Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", r, a, b, result);
        self.registers[r as usize] = result;

        Ok(())
    }

    fn handle_pop(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;

        if self.stack.len() < 2 {
            return Err("Stack underflow".to_string());
        }

        let low_byte = self.stack.pop().unwrap() as u16;
        let high_byte = self.stack.pop().unwrap() as u16;
        let value = (high_byte << 8) | low_byte;

        self.registers[r as usize] = value;
        print!("POP 0x{:X} into {:?}", value, r);

        Ok(())
    }

    fn handle_push(&mut self) -> Result<(), String> {
        let value: u16 = self.fetch()?;

        if self.stack.len() >= Self::DATA_LENGTH - 2 {
            return Err("Stack overflow".to_string());
        }

        self.stack.push((value >> 8) as u8); // High byte
        self.stack.push((value & 0xFF) as u8); // Low byte

        print!("PUSH 0x{:X}", value);
        Ok(())
    }
}
