pub mod fetch;
pub mod opcode;
pub mod syscall;

use fetch::Fetch;
use opcode::OpCode;
use syscall::Syscall;

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
    pub halt: bool,
}

impl Machine {
    pub const DATA_SECTION_LENGTH: usize = (Self::DATA_LENGTH as f32 * 0.01) as usize;
    const REGISTER_COUNT: usize = Register::RegisterCount as usize;
    const DATA_LENGTH: usize = 4096;

    pub fn new() -> Self {
        Self {
            registers: [0; Self::REGISTER_COUNT],
            data: [0; Self::DATA_LENGTH],
            halt: false,
        }
    }

    pub fn set_data(&mut self, new_data: &[u8]) {
        if new_data.len() > self.data.len() {
            panic!("Data length exceeds memory capacity");
        }
        self.data[..new_data.len()].copy_from_slice(new_data);
    }

    pub fn set_data_section(&mut self, data_section: &[u8]) {
        let data_len = self.data.len();
        let section_len = data_section.len();
        
        if section_len > Self::DATA_SECTION_LENGTH {
            panic!("Data section length exceeds defined section length");
        }
        
        if section_len > data_len {
            panic!("Data length exceeds memory capacity");
        }

        let start = data_len - Self::DATA_SECTION_LENGTH;
        let end = start + section_len;
        
        self.data[start..end].copy_from_slice(data_section);
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
            OpCode::SYSCALL => self.handle_syscall()?,
            OpCode::ADD => self.handle_add()?,
            OpCode::SUB => self.handle_sub()?,
            OpCode::MUL => self.handle_mul()?,
            OpCode::DIV => self.handle_div()?,
            OpCode::POP => self.handle_pop()?,
            OpCode::PUSH => self.handle_push()?,
            OpCode::SWAP => self.handle_swap()?,
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

    /// Pop what ever the value was in SP into `r`
    fn handle_pop(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        self.registers[r as usize] = self.registers[Register::SP as usize];
        Ok(())
    }

    /// Push what ever the value was in `r` into SP
    fn handle_push(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        self.registers[Register::SP as usize] = self.registers[r as usize];
        Ok(())
    }

    /// Swap values of `r1` and `r2`
    fn handle_swap(&mut self) -> Result<(), String> {
        let r1: Register = self.fetch()?;
        let r2: Register = self.fetch()?;

        let mut v1 = self.registers[r1 as usize];
        let mut v2 = self.registers[r2 as usize];

        let temp = v2;
        v2 = v1;
        v1 = temp;

        self.registers[r1 as usize] = v1;
        self.registers[r2 as usize] = v2;
        Ok(())
    }

    /// Call syscall with provided args
    fn handle_syscall(&mut self) -> Result<(), String> {
        let syscall: Syscall = self.fetch()?;

        syscall.handle(self)?;

        Ok(())
    }
}
