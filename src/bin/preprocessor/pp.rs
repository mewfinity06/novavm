use novavm::opcode::OpCode;
use novavm::syscall::Syscall;
use novavm::Register;

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

            parts.extend(line.split(' ').map(parse_word).filter_map(|x| x.ok()));
        }
        let parsed_parts: Vec<u8> = parts
            .iter()
            .map(parse_part_into_u16)
            .map(|x| (x & 0xFF) as u8)
            .collect();

        self.memory.extend_from_slice(&parsed_parts);
        self.data.extend_from_slice(&data_section);

        Ok(())
    }

    pub fn print(self) {
        for x in self.memory {
            print!("0x{x:04X?}");
        }
        println!();
        println!("[[DATA]]");
        for x in self.data {
            print!("0x{x:04X?}");
        }
    }
}

fn parse_word(s: &str) -> Result<Part, String> {
    if let Ok(op) = OpCode::try_from(s) {
        Ok(Part::OpCode(op))
    } else if let Ok(r) = Register::try_from(s) {
        Ok(Part::Register(r))
    } else if let Ok(s) = Syscall::try_from(s) {
        Ok(Part::Syscall(s))
    } else if let Some(x) = s.strip_prefix('$') {
        let parsed =
            u16::from_str_radix(x, 10).expect(&format!("could not parse `{}` into Base10", x));
        Ok(Part::Base10(parsed))
    } else if let Some(x) = s.strip_prefix('%') {
        let parsed =
            u16::from_str_radix(x, 64).expect(&format!("could not parse `{}` into Base64", x));
        Ok(Part::Base64(parsed))
    } else if let Some(x) = s.strip_prefix("0x") {
        let parsed =
            u16::from_str_radix(x, 16).expect(&format!("could not parse `{}` into hex", x));
        Ok(Part::Hex(parsed))
    } else {
        // panic!("Unknown word `{s}`");
        Err(format!("unknown word `{s}`"))
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
