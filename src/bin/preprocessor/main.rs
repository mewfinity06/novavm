#![allow(unused)]
use pp::PreProcessor;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod macros;
mod pp;

/// TODO:
/// Implement a macrosystem
/// ```asm
/// !define_macro FOO 0x71 ; Set FOO as 0x71
/// !define_macro BAR 0x09 ; Set BAR as 0x09
///
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
