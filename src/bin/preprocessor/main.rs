#![allow(unused)]
use pp::PreProcessor;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use unescape::unescape;

mod pp;

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
///```
pub struct TODO;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        return Err(format!(
            "Usage {} input.asm > output.proj",
            env::current_exe().unwrap().display()
        ));
    }

    let file_path = args.get(1).unwrap();

    let lines: Vec<_> = match read_lines(file_path) {
        Ok(l) => l.flatten().collect(),
        Err(e) => return Err(format!("{:?}", e)),
    };

    let mut pp = PreProcessor::new(lines);

    pp.parse()?;
    pp.print();

    Ok(())
}

/*
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
        let pp = PreProcessor::new(lines.flatten());

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
                    println!("0x{:04X}", x);        }
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
*/

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
