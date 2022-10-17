use std::num::Wrapping;
use super::cpu::*;

fn get_rom(cpu: &mut CpuStruct) -> Wrapping<u8> {
    if cpu.swt {
        return cpu.rom[(cpu.dta.0 as usize)*256+cpu.adr.0 as usize];
    }
    return cpu.rom[cpu.prc.0 as usize];
}

// No instruction.
pub fn noi(cpu: &mut CpuStruct) {
    match cpu.ic.0 {
        0 => cpu.prc += 1,
        1 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f)}
        _ => {}
    }
}

// Jump instruction.
pub fn jum_num(cpu: &mut CpuStruct) {
    match cpu.ic.0 {
        0 => cpu.prc += 1,
        1 => cpu.aux = cpu.rom[cpu.prc.0 as usize],
        2 => cpu.prc += 1,
        3 => cpu.adr = cpu.rom[cpu.prc.0 as usize],
        4 => cpu.prc += 1,
        5 => if cpu.dta.0 != 0 {cpu.prc = Wrapping((cpu.aux.0 as u16)*256 + cpu.adr.0 as u16)},
        6 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f)},
        _ => {},
    }
}

// Load data registry from ROM at NEXT.
pub fn ldd_num(cpu: &mut CpuStruct) {
    match cpu.ic.0 {
        0 => cpu.prc += 1,
        1 => cpu.dta = cpu.rom[cpu.prc.0 as usize],
        2 => cpu.prc += 1,
        3 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f)},
        _ => {}
    }
}

// Load data registry from RAM.
pub fn ldd_ram(cpu: &mut CpuStruct) {
    match cpu.ic.0 {
        0 => cpu.prc += 1,
        1 => cpu.adr = cpu.rom[cpu.prc.0 as usize],
        2 => {cpu.dta = cpu.ram[cpu.adr.0 as usize]; cpu.prc += 1}
        3 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f)},
        _ => {}
    }
}

// Load data registry.
pub fn ldd_prr(cpu: &mut CpuStruct) {
    match cpu.ic.0 {
        0 => cpu.prc += 1,
        1 => cpu.adr = cpu.rom[cpu.prc.0 as usize],
        2 => cpu.adr = cpu.ram[cpu.adr.0 as usize],
        3 => {cpu.dta = cpu.ram[cpu.adr.0 as usize]; cpu.prc += 1},
        4 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f)},
        _ => {}
    }
}

pub fn ldd_inp(cpu: &mut CpuStruct) {
    match cpu.ic.0 {
        0 => cpu.prc += 1,
        1 => {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Error");
            let input: u8 = input.trim().parse().unwrap();
            cpu.dta = Wrapping(input);
        }
        2 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f) },
        _ => {}
    }
}

// Load data regstry from ROM address.
pub fn ldd_rom(cpu: &mut CpuStruct) {
    match cpu.ic.0 {
        0 => cpu.prc += 1,
        1 => cpu.dta = cpu.rom[cpu.prc.0 as usize],
        2 => cpu.prc += 1,
        3 => cpu.adr = cpu.rom[cpu.prc.0 as usize],
        4 => cpu.swt = true,
        5 => {
            cpu.dta = get_rom(cpu);
            cpu.prc += 1
        },
        6 => cpu.swt = false,
        7 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f) },
        _ => {}
    }
}

// Store data to RAM.
pub fn std_ram(cpu: &mut CpuStruct) {
    match cpu.ic.0 {
        0 => cpu.prc += 1,
        1 => cpu.adr = cpu.rom[cpu.prc.0 as usize],
        2 => {cpu.ram[cpu.adr.0 as usize] = cpu.dta; cpu.prc += 1}
        3 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f) },
        _ => {}
    }
}

// Store data to RAM.
pub fn std_prr(cpu: &mut CpuStruct) {
    match cpu.ic.0 {
        0 => cpu.prc += 1,
        1 => cpu.adr = cpu.rom[cpu.prc.0 as usize],
        2 => cpu.adr = cpu.ram[cpu.adr.0 as usize],
        3 => {cpu.ram[cpu.adr.0 as usize] = cpu.dta; cpu.prc += 1}
        4 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f) },
        _ => {}
    }
}

// Store data to the OUT registry.
pub fn std_out(cpu: &mut CpuStruct) {
    match cpu.ic.0 {
        0 => println!("{}", cpu.dta),
        1 => cpu.prc += 1,
        2 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f) },
        _ => {}
    }
}

fn alu_perform_operation(cpu: &mut CpuStruct, instruction: AluInstruction) {
    match instruction {
        AluInstruction::ADD => cpu.dta += cpu.aux,
        AluInstruction::SUB => cpu.dta -= cpu.aux,
        AluInstruction::NAN => cpu.dta = !(cpu.dta & cpu.aux),
        AluInstruction::SHL => cpu.dta <<= 1,
        AluInstruction::SHR => cpu.dta >>= 1,
        AluInstruction::GRE => cpu.dta = if cpu.dta > cpu.aux { Wrapping(0xff) } else { Wrapping(0x00) },
        AluInstruction::EQU => cpu.dta = if cpu.dta == cpu.aux { Wrapping(0xff) } else { Wrapping(0x00) },
    };
}

// Alu operation from a variable in rom.
pub fn alu_num(cpu: &mut CpuStruct, instruction: AluInstruction) {
    match cpu.ic.0 {
        0 => cpu.prc += 1,
        1 => cpu.aux = cpu.rom[cpu.prc.0 as usize],
        2 => {alu_perform_operation(cpu, instruction); cpu.prc += 1}
        3 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f)}
        _ => {}
    }
}

// Alu operation.
pub fn alu_ram(cpu: &mut CpuStruct, instruction: AluInstruction) {
    match cpu.ic.0 {
        0 => cpu.prc += 1,
        1 => cpu.adr = cpu.rom[cpu.prc.0 as usize],
        2 => cpu.aux = cpu.ram[cpu.adr.0 as usize],
        3 => {alu_perform_operation(cpu, instruction); cpu.prc += 1}
        4 => {cpu.ins = cpu.rom[cpu.prc.0 as usize]; cpu.ic = Wrapping(0x0f)}
        _ => {}
    }
}
