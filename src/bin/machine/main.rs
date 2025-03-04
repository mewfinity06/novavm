use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

use novavm::Machine;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage {:?} [file].proj", env::current_exe());
        std::process::exit(1);
    }

    let file_path = args.get(1).unwrap();

    let data = get_data(file_path)?;

    let mut machine = Machine::new();

    machine.set_data(&data);

    println!("| RUNNING THE MACHINE |");

    while !machine.halt {
        machine.print_state();
        machine.step()?;
    }

    Ok(())
}

fn get_data(file_path: &str) -> Result<Vec<u8>, String> {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return Err(format!("could not read {}", file_path)),
    };

    let reader = io::BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|_| "could not read line".to_string())?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("0x") {
            let value = u16::from_str_radix(&line[2..], 16)
                .map_err(|_| format!("could not parse hex number: {}", line))?;
            let high_byte = (value >> 8) as u8; // Extract the high 8 bits
            let low_byte = (value & 0xFF) as u8; // Extract the low 8 bits

            if high_byte != 0x00 {
                result.push(high_byte);
            }
            result.push(low_byte);
        } else {
            return Err(format!("invalid format: {}", line));
        }
    }

    Ok(result)
}
