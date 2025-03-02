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

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::A,
            2 => Self::B,
            3 => Self::C,
            4 => Self::M,
            5 => Self::SP,
            6 => Self::PC,
            7 => Self::FLAGS,
            _ => panic!("{} is not a valid register", value),
        }
    }
}

pub struct Machine {
    registers: [u16; Self::REGISTER_COUNT],
    data: [u8; Self::DATA_LENGTH],
    pub halt: bool,
}

impl Machine {
    const REGISTER_COUNT: usize = Register::RegisterCount as usize;
    const DATA_LENGTH: usize = 32;

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

    pub fn step(&mut self) -> Result<(), String> {
        if self.registers[Register::PC as usize] as usize >= self.data.len() {
            self.halt = true;
        }

        let op: OpCode = self.fetch();

        print!("| Current opcode {:?} (0x{:X}) ", op, op as u8);

        match op {
            OpCode::NOP => {}
            OpCode::HALT => self.halt = true,
            OpCode::ADD => {
                let a: u16 = self.fetch();
                let b: u16 = self.fetch();
                print!("0x{:X}, 0x{:X} -> 0x{:X}", a, b, a + b);
                self.registers[Register::SP as usize] = a + b;
            }
            OpCode::SUB => {
                let a: u16 = self.fetch();
                let b: u16 = self.fetch();
                print!("0x{:X}, 0x{:X} -> 0x{:X}", a, b, a - b);
                self.registers[Register::SP as usize] = a - b;
            }
            OpCode::MUL => {
                let a: u16 = self.fetch();
                let b: u16 = self.fetch();
                print!("0x{:X}, 0x{:X} -> 0x{:X}", a, b, a * b);
                self.registers[Register::SP as usize] = a * b;
            }
            OpCode::DIV => {
                let a: u16 = self.fetch();
                let b: u16 = self.fetch();
                print!("0x{:X}, 0x{:X} -> 0x{:X}", a, b, a / b);
                self.registers[Register::SP as usize] = a / b;
            }
            OpCode::POP => {
                let r: Register = self.fetch();
                self.registers[r as usize] = self.registers[Register::SP as usize];
                print!("0x{:X} -> {:?}", self.registers[r as usize], r);
            }
            OpCode::PUSH => {
                let r: Register = self.fetch();
                self.registers[Register::SP as usize] =
                    self.data[self.registers[r as usize] as usize] as u16;
                print!("{:?} -> {:X}", r, self.registers[Register::SP as usize]);
            }
        }

        println!();
        Ok(())
    }

    fn fetch<T>(&mut self) -> T
    where
        T: Fetch,
    {
        let d_point = self.data[self.registers[Register::PC as usize] as usize];
        let v: T = T::from(d_point);
        self.registers[Register::PC as usize] += 1;
        v
    }

    pub fn print_state(&self) {
        let [a, b, c, m, sp, pc, flags] = self.registers;
        println!("|-----------------------------------------|");
        println!("| A  {a:04X} | B  {b:04X} | C     {c:04X} | M {m:04X} |");
        println!("| SP {sp:04X} | PC {pc:04X} | FLAGS {flags:04X} |");
    }

    pub fn print_data(&self) {
        for (i, d) in self.data.iter().enumerate() {
            print!("0x{d:X} ");
            if i % 5 == 0 {
                println!();
            }
        }
    }
}
