use novavm::opcode::OpCode;
use novavm::syscall::Syscall;
use novavm::Register;
use unescape::unescape;

#[derive(Debug, Clone, Copy)]
enum Part {
    OpCode(OpCode),
    Register(Register),
    Syscall(Syscall),
    Base10(u16),
    Base64(u16),
    Hex(u16),
    Binary(u16),
    None,
}

#[derive(Debug)]
pub struct PreProcessor {
    /// Input
    lines: Vec<String>,
    /// Memory
    memory: Vec<u8>,
    /// Data
    data: Vec<u8>,
}

impl PreProcessor {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            lines,
            memory: Vec::new(),
            data: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        let mut parts: Vec<Part> = Vec::new();

        let mut data_section: Vec<u8> = Vec::new();
        let mut in_data_section = false;

        for line in &self.lines {
            // Skip comments
            if line.starts_with(';') {
                continue;
            }

            if line.starts_with("[[DATA]]") {
                in_data_section = true;
                let data_contents = line["[[DATA]]".len()..].trim();
                data_section.extend_from_slice(data_contents.as_bytes());
                data_section.push(0);
                continue;
            }

            if in_data_section {
                data_section.extend_from_slice(line.trim().as_bytes());
                data_section.push(0);
                continue;
            }

            parts.extend(line.split(' ').map(parse_word));
        }
        let parsed_parts: Vec<_> = parts
            .iter()
            .map(parse_part_into_u16)
            .flat_map(u16::to_be_bytes)
            .collect();

        self.memory.extend_from_slice(&parsed_parts);
        self.data.extend_from_slice(&data_section);

        Ok(())
    }

    pub fn print(self) {
        for x in self.memory {
            println!("Ox{x:X?}");
        }
        println!("[[DATA]]");
        for x in self.data {
            println!("0x{x:X?}");
        }
    }
}

fn u16_to_u8(x: u16) -> (u8, u8) {
    todo!()
}

fn parse_word(s: &str) -> Part {
    if let Ok(op) = OpCode::try_from(s) {
        Part::OpCode(op)
    } else if let Ok(r) = Register::try_from(s) {
        Part::Register(r)
    } else if let Ok(s) = Syscall::try_from(s) {
        Part::Syscall(s)
    } else if let Some(x) = s.strip_prefix('$') {
        let parsed =
            u16::from_str_radix(x, 10).expect(&format!("could not parse `{}` into Base10", x));
        Part::Base10(parsed)
    } else if let Some(x) = s.strip_prefix('%') {
        let parsed =
            u16::from_str_radix(x, 64).expect(&format!("could not parse `{}` into Base64", x));
        Part::Base64(parsed)
    } else if let Some(x) = s.strip_prefix("0x") {
        let parsed =
            u16::from_str_radix(x, 16).expect(&format!("could not parse `{}` into hex", x));
        Part::Hex(parsed)
    } else {
        panic!("Unknown word `{s}`");
    }
}

fn parse_part_into_u16(p: &Part) -> u16 {
    match *p {
        Part::OpCode(o) => o as u16,
        Part::Register(r) => r as u16,
        Part::Syscall(s) => s as u16,
        Part::Base10(x) => x,
        Part::Base64(x) => x,
        Part::Hex(x) => x,
        _ => unimplemented!("{:?}", p),
    }
}
