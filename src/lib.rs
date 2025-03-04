pub mod fetch;
pub mod opcode;

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
    
        println!("| Stack (top 8 values):");
        let stack_len = self.stack.len();
        for i in 0..8 {
            if i < stack_len {
                print!("0x{:02X} ", self.stack[stack_len - 1 - i]);
            } else {
                print!("-- ");
            }
        }
        println!();
    }
    
    pub fn print_data(&self) {
        for (i, d) in self.data.iter().enumerate() {
            if i % 8 == 0 {
                println!()
            } else if i > 38 {
                break;
            }
            print!("0x{d:02X} ");
        }
        println!();
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.registers[Register::PC as usize] as usize >= self.data.len() {
            self.halt = true;
        }

        let op: OpCode = self.fetch()?;

        print!("| Current opcode {:?} (0x{:X}) ", op, op as u8);

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

        println!();
        Ok(())
    }

    fn handle_halt(&mut self) {
        self.halt = true;
    }

    fn handle_nop(&self) {}

    fn handle_add(&mut self) -> Result<(), String> {
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a + b;

        if let Ok(register) = Register::try_from(a as u8) {
            self.registers[register as usize] = result;
            print!("Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", register, a, b, result);
        } else {
            self.registers[Register::SP as usize] = result;
            print!("0x{:X}, 0x{:X} -> 0x{:X}", a, b, result);
        }

        Ok(())
    }

    fn handle_sub(&mut self) -> Result<(), String> {
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a - b;

        if let Ok(register) = Register::try_from(a as u8) {
            self.registers[register as usize] = result;
            print!("Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", register, a, b, result);
        } else {
            self.registers[Register::SP as usize] = result;
            print!("0x{:X}, 0x{:X} -> 0x{:X}", a, b, result);
        }

        Ok(())
    }

    fn handle_mul(&mut self) -> Result<(), String> {
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a * b;

        if let Ok(register) = Register::try_from(a as u8) {
            self.registers[register as usize] = result;
            print!("Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", register, a, b, result);
        } else {
            self.registers[Register::SP as usize] = result;
            print!("0x{:X}, 0x{:X} -> 0x{:X}", a, b, result);
        }

        Ok(())
    }

    fn handle_div(&mut self) -> Result<(), String> {
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a / b;

        if let Ok(register) = Register::try_from(a as u8) {
            self.registers[register as usize] = result;
            print!("Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", register, a, b, result);
        } else {
            self.registers[Register::SP as usize] = result;
            print!("0x{:X}, 0x{:X} -> 0x{:X}", a, b, result);
        }

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
