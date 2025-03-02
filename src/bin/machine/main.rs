use novavm::Machine;

fn main() -> Result<(), String> {
    let mut machine = Machine::new();

    machine.set_data(&[
        0x50, 16, 1,      // ADD 16 1 -> 17
        0x61, 1,          // POP A
        0x51, 0x53, 0x6,  // SUB 0x53, 0x6 -> 0x4D
        0x61, 2,          // POP B
        0x60, 3           // PUSH C
    ]);

    // machine.self.registers[r as usize];print_data();

    while !machine.halt {
        machine.print_state();
        machine.step()?;
    }

    Ok(())
}