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

fn main() -> Result<(), String> {
    // Obtain file path from commandline
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage {:?} [path-to-file].asm > output.proj", env::current_exe());
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

            let res: Vec<Part> = line
                .split_whitespace()
                .map(|word| {
                    let word = word.trim();
                    match word {
                        _ if word.starts_with('$') => {
                            let x = u16::from_str_radix(&word[1..], 10)
                                .expect(&format!("could not get base10 from {}", word));
                            Part::Base10(x)
                        }
                        _ if word.starts_with("0x") => {
                            let x = u16::from_str_radix(&word[2..], 16)
                                .expect(&format!("could not get hex from {}", word));
                            Part::Hex(x)
                        }
                        _ if OpCode::try_from(word).is_ok() => {
                            Part::OpCode(OpCode::try_from(word).unwrap())
                        }
                        _ if Register::try_from(word).is_ok() => {
                            Part::Register(Register::try_from(word).unwrap())
                        }
                        _ if u16::from_str_radix(word, 10).is_ok() => {
                            Part::Base10(u16::from_str_radix(word, 10).unwrap())
                        }
                        _ => panic!("unknown `{}`", word),
                    }
                })
                .collect();

            parts.extend(res);
        }

        for part in parts {
            match part {
                Part::OpCode(op_code) => print!("{:04X}", op_code as usize),
                Part::Register(register) => print!("{:04X}", register as usize),
                Part::Base10(x) => print!("{:04X}", x),
                Part::Hex(x) => print!("{:04X}", x),
                Part::Binary(x) => print!("{:04X}", x),
            }
        }
    }

    println!();

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
