#![allow(unused)]
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use unescape::unescape;

use novavm::opcode::OpCode;
use novavm::syscall::Syscall;
use novavm::Register;

/// TODO:
/// - Implement a macrosystem
/// ```asm
/// !define_macro FOO => 0x71
/// !define_macro BAR => 0x09
/// ; Multiline macro
/// !define_macro BAZ => \
///     PUSH !FOO        \
///     POP   A          \
///     EQ    A !FOO
///
/// ADD !FOO !BAR ;
/// ```
/// - Better comments
/// ```asm
/// PUSH A ; Comment like this is impossible at this point
/// ;;
///     Multiline comment
/// ;;
/// ```
pub struct TODO;

#[derive(Debug, Clone, Copy)]
enum Part {
    OpCode(OpCode),
    Register(Register),
    Syscall(Syscall),
    Base10(u16),
    Hex(u16),
    Binary(u16),
}

fn parse_word(word: &str) -> Result<Part, String> {
    let word = word.trim();
    if word.starts_with('$') {
        let x = u16::from_str_radix(&word[1..], 10)
            .map_err(|_| format!("could not get base10 from {}", word))?;
        Ok(Part::Base10(x))
    } else if word.starts_with("0x") {
        let x = u16::from_str_radix(&word[2..], 16)
            .map_err(|_| format!("could not get hex from {}", word))?;
        Ok(Part::Hex(x))
    } else if let Ok(op) = OpCode::try_from(word) {
        Ok(Part::OpCode(op))
    } else if let Ok(reg) = Register::try_from(word){
        Ok(Part::Register(reg))
    } else if let Ok(syscall) = Syscall::try_from(word) {
        Ok(Part::Syscall(syscall))
    } else if u16::from_str_radix(word, 10).is_ok() {
        Ok(Part::Base10(u16::from_str_radix(word, 10).unwrap()))
    } else {
        Err(format!("unknown `{}`", word))
    }
}

fn main() -> Result<(), String> {
    // Obtain file path from commandline
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Usage {:?} [path-to-file].asm > output.proj",
            env::current_exe()
        );
        std::process::exit(1);
    }

    let path_to_file = args.get(1).unwrap();

    if let Ok(lines) = read_lines(path_to_file) {
        let mut parts: Vec<Part> = Vec::with_capacity(100); // Pre-allocate with an estimated capacity
        let mut data_section: Vec<u8> = Vec::new();
        let mut in_data_section = false;

        for line in lines.flatten() {
            // Skip comments
            if line.starts_with(';') {
                continue;
            }

            if line.starts_with("[[DATA]]") {
                in_data_section = true;
                let data_content = line["[[DATA]]".len()..].trim().trim_matches('"');
                if let Some(unescaped) = unescape(data_content) {
                    data_section.extend_from_slice(unescaped.as_bytes());
                } else {
                    data_section.extend_from_slice(data_content.as_bytes());
                }
                data_section.push(0); // Add null termination
                continue;
            }

            if in_data_section {
                if let Some(unescaped) = unescape(line.trim()) {
                    data_section.extend_from_slice(unescaped.as_bytes());
                } else {
                    data_section.extend_from_slice(line.trim().as_bytes());
                }
                data_section.push(0); // Add null termination
                continue;
            }

            let res: Result<Vec<Part>, String> = line.split_whitespace().map(parse_word).collect();

            match res {
                Ok(parsed_parts) => parts.extend(parsed_parts),
                Err(e) => eprintln!("Error parsing line: {}", e),
            }
        }

        for part in parts {
            match part {
                Part::OpCode(op_code) => {
                    println!("0x{:04X}", op_code as u8);
                }
                Part::Register(register) => {
                    println!("0x{:04X}", register as u8);
                }
                Part::Base10(x) => {
                    println!("0x{:04X}", x);
                }
                Part::Hex(x) => {
                    println!("0x{:04X}", x);
                }
                Part::Binary(x) => {
                    println!("0x{:04X}", x);
                }
                Part::Syscall(syscall) => {
                    println!("0x{:04X?}", syscall as u8);
                }
            }
        }

        if !data_section.is_empty() {
            println!("[[DATA]]");
            for byte in data_section {
                println!("0x{:02X}", byte);
            }
        }
    }
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
