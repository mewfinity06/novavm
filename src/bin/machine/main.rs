use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

use novavm::Machine;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        return Err(format!("Usage {:?} [file].proj", env::current_exe()));
    }

    let flags = if args.len() > 2 {
        &args[1..args.len()]
    } else {
        &[]
    };

    let file_path = args.last().unwrap();

    let (memory, data_section) = get_data(file_path)?;

    let mut machine = Machine::new();

    machine.set_memory(&memory);
    machine.set_data(&data_section);

    flags.iter().for_each(|f| match f.as_str() {
        "--debug" | "-d" => machine.enable_debug(),
        _ => {}
    });

    println!("| RUNNING THE MACHINE |");

    while !machine.halt {
        machine.step()?;
    }

    Ok(())
}

fn get_data(file_path: &str) -> Result<(Vec<u8>, Vec<u8>), String> {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return Err(format!("could not read {}", file_path)),
    };

    let reader = io::BufReader::new(file);

    let mut memory: Vec<u8> = vec![];
    let mut data: Vec<u8> = vec![];
    let mut in_data_section: bool = false;

    for line in reader.lines() {
        let line = line.map_err(|_| "could not read line".to_string())?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line == "[[DATA]]" {
            in_data_section = true;
            continue;
        }

        let chunks: Vec<_> = line.split("0x").collect();

        dbg!(&chunks);

        for chunk in chunks {
            if chunk.is_empty() {
                continue;
            }

            let value = u16::from_str_radix(chunk, 16)
                .map_err(|_| format!("could not parse hex number from `{}`", chunk))?;

            let low = (value & 0xff) as u8;

            if !in_data_section {
                memory.push(low);
            } else {
                data.push(low);
            }
        }
    }

    Ok((memory, data))
}

fn _get_dataet_data(file_path: &str) -> Result<(Vec<u8>, Vec<u8>), String> {
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return Err(format!("could not read {}", file_path)),
    };

    let reader = io::BufReader::new(file);
    let mut memory = Vec::new();
    let mut data = Vec::new();
    let mut in_data_section = false;

    for line in reader.lines() {
        let line = line.map_err(|_| "could not read line".to_string())?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line == "[[DATA]]" {
            in_data_section = true;
            continue;
        }

        if line.starts_with("0x") {
            let value = u16::from_str_radix(&line[2..], 16)
                .map_err(|_| format!("could not parse hex number: {}", line))?;
            let high_byte = (value >> 8) as u8; // Extract the high 8 bits
            let low_byte = (value & 0xFF) as u8; // Extract the low 8 bits

            if in_data_section {
                if high_byte != 0x00 {
                    data.push(high_byte);
                }
                data.push(low_byte);
            } else {
                if high_byte != 0x00 {
                    memory.push(high_byte);
                }
                memory.push(low_byte);
            }
        } else {
            return Err(format!("invalid format: {}", line));
        }
    }

    Ok((memory, data))
}
