#![allow(unused)]
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use novavm::opcode::OpCode;
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
    } else if OpCode::try_from(word).is_ok() {
        Ok(Part::OpCode(OpCode::try_from(word).unwrap()))
    } else if Register::try_from(word).is_ok() {
        Ok(Part::Register(Register::try_from(word).unwrap()))
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

        for line in lines.flatten() {
            // Convert file.asm contents into binary lines
            // Each instruction will be converted into its own line
            // ADD 0x82, 0x5 -> 0x50 0x82 0x50
            // HALT          -> 0x00

            // Skip comments
            if line.starts_with(';') {
                continue;
            }

            let res: Result<Vec<Part>, String> = line
                .split_whitespace()
                .map(parse_word)
                .collect();

            match res {
                Ok(parsed_parts) => parts.extend(parsed_parts),
                Err(e) => eprintln!("Error parsing line: {}", e),
            }
        }

        for part in parts {
            match part {
                Part::OpCode(op_code) => {
                    // eprintln!("Op\t{:?}\t(0b{:08b})", op_code, op_code as u8);
                    println!("0x{:04X}", op_code as u8);
                }
                Part::Register(register) => {
                    // eprintln!("Reg\t{:?}\t(0b{:08b})", register, register as u8);
                    println!("0x{:04X}", register as u8);
                }
                Part::Base10(x) => {
                    // eprintln!("Base10\t{:?}\t(0b{:016b})", x, x);
                    println!("0x{:04X}", x);
                }
                Part::Hex(x) => {
                    // eprintln!("Hex\t{:?}\t(0b{:016b})", x, x);
                    println!("0x{:04X}", x);
                }
                Part::Binary(x) => {
                    // eprintln!("Binary\t{:?}", x);
                    println!("0x{:04X}", x);
                }
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
