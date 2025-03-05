pub mod fetch;
pub mod opcode;
pub mod syscall;

use fetch::Fetch;
use opcode::OpCode;
use syscall::Syscall;

/// Registers:
///     A     - First register
///     B     - Second register
///     C     - Third register
///     M     - Multipurpose register
///     SP    - Stack Pointer
///     PC    - Program Pointer
///     FLAGS - Flags
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

/// Machine
///     Registers: Holds the registers
///     Memory   : Holds the mutable data
///     Data     : Holds the immutable data
///     Halt     : Should the program halt
///     Debug    : Prints debug information
///              | TODO?: have all debug info
///              |        written to a file
///              |        rather than to stdout 
///              |        as to not be confused with 
///              |        output of the program
pub struct Machine {
    registers: [u16; Self::REGISTER_COUNT],
    memory: [u8; Self::MEMORY_LENGTH],
    data: [u8; Self::DATA_LENGTH],
    pub halt: bool,
    pub debug: bool,
}

impl Machine {
    /// How many registers does this machine have?
    pub const REGISTER_COUNT: usize = Register::RegisterCount as usize;
    /// Mutable memory length
    pub const MEMORY_LENGTH: usize = 4096;
    /// Immutable memory length
    pub const DATA_LENGTH: usize = 1024;

    /// Creates a new, empty machine, halts on first instruction
    pub fn new() -> Self {
        Self {
            registers: [0; Self::REGISTER_COUNT],
            memory: [0; Self::MEMORY_LENGTH],
            data: [0; Self::DATA_LENGTH],
            halt: false,
            debug: false,
        }
    }

    /// Sets the mutable memory of the file from a slice of u8
    pub fn set_memory(&mut self, new_data: &[u8]) {
        if new_data.len() > self.memory.len() {
            panic!("Memory length exceeds memory capacity");
        }
        self.memory[..new_data.len()].copy_from_slice(new_data);
    }

    /// Sets the immutable memory of the file from a slice of u8
    pub fn set_data(&mut self, new_data: &[u8]) {
        if new_data.len() > self.data.len() {
            panic!("Data length exceeds data capacity");
        }
        self.data[..new_data.len()].copy_from_slice(new_data);
    }

    /// Enables debug information for the machine
    pub fn enable_debug(&mut self) {
        self.debug = true;
    }

    /// Gets a value from memory at program counter
    fn fetch<'a, T>(&mut self) -> Result<T, String>
    where
        T: Fetch<'a>,
    {
        let pc = self.registers[Register::PC as usize] as usize;
        let d_point = self.memory[pc];
        let v: T = T::try_from(d_point).map_err(|_| format!("could not fetch"))?;
        self.registers[Register::PC as usize] += 1;
        Ok(v)
    }

    /// Prints the Machine's state
    pub fn print_state(&self) {
        let [a, b, c, m, sp, pc, flags] = self.registers;
        println!("|-----------------------------------------|");
        println!("| A  {a:04X} | B  {b:04X} | C     {c:04X} | M {m:04X} |");
        println!("| SP {sp:04X} | PC {pc:04X} | FLAGS {flags:04X} |");
    }

    /// Takes a `step` in the machine, increment the program counter by one and take an action
    /// The step function is seperated into the following main steps
    /// 1. Check if debug is set, if so, print state
    /// 2. Check out of bounds error: if the program counter is larger than the memory length, halt
    /// 3. Get current opcode and act accordingly
    /// 4. If that opcode fails, return.
    pub fn step(&mut self) -> Result<(), String> {

        if self.debug {
            self.print_state();
        }

        if self.registers[Register::PC as usize] as usize > self.memory.len() {
            self.halt = true;
            return Err(format!("Out of bounds"));
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

    /// Halt the program
    fn handle_halt(&mut self) {
        self.halt = true;
        if self.debug {
            println!("| OpCode HALT")
        };
    }

    /// Take no action, `No operation`
    fn handle_nop(&self) {
        if self.debug {
            println!("| OpCode NOP");
        }
    }

    /// Take in a register and two values
    /// Adds the values together and puts it into specified register
    fn handle_add(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a + b;
        self.registers[r as usize] = result;
        if self.debug {
            println!("| ADD: Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", r, a, b, result);
        }
        Ok(())
    }

    /// Take in a register and two values
    /// Subs the values together and puts it into specified register
    fn handle_sub(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a - b;
        self.registers[r as usize] = result;
        if self.debug {
            println!("| SUB: Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", r, a, b, result);
        }
        Ok(())
    }

    /// Take in a register and two values
    /// Multiplies the values together and puts it into specified register
    fn handle_mul(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a * b;
        self.registers[r as usize] = result;
        if self.debug {
            println!("| MUL: Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", r, a, b, result);
        }
        Ok(())
    }

    /// Take in a register and two values
    /// Divides the values together and puts it into specified register
    fn handle_div(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        let a: u16 = self.fetch()?;
        let b: u16 = self.fetch()?;
        let result = a / b;
        self.registers[r as usize] = result;
        if self.debug {
            println!("| DIV: Reg {:?} 0x{:X}, 0x{:X} -> 0x{:X}", r, a, b, result);
        }
        Ok(())
    }

    /// Pop what ever the value was in SP into `r`
    fn handle_pop(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        self.registers[r as usize] = self.registers[Register::SP as usize];
        if self.debug {
            println!("| POP: Reg {:?}", r);
        }
        Ok(())
    }

    /// Push what ever the value was in `r` into SP
    fn handle_push(&mut self) -> Result<(), String> {
        let r: Register = self.fetch()?;
        self.registers[Register::SP as usize] = self.registers[r as usize];
        if self.debug {
            println!("| POP: PUSH {:?}", r);
        }
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
        if self.debug {
            println!("| SWAP: Reg {:?} ({}) Reg {:?} ({})", r1, v1, r2, v2);
        }
        Ok(())
    }

    /// Call syscall with provided args
    fn handle_syscall(&mut self) -> Result<(), String> {
        let syscall: Syscall = self.fetch()?;

        syscall.handle(self)?;

        Ok(())
    }
}
