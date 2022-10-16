use std::env;
mod cpu;
mod instruction;

use cpu::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("No ROM file given.");
    }

    let mut cpu = CpuStruct {
        dta: 0,
        aux: 0,
        ins: 0,
        prc: 0,
        adr: 0,
        ic: 0,
        swt: false,
        ram: [0; 256],
        rom: [0; 65536]
    };

    // Read program from ROM memory.
    let bytes = std::fs::read(&args[1]).expect("Error while reading ROM file");
    let mut i = 0;
    for b in bytes {
        cpu.rom[i] = b;
        i += 1;
    }

    println!("steps:");
    let mut step = 0;
    loop {
        println!("- instruction: {}", match cpu.ins {
            0b00000_000 => "NOI.noa",
            0b00001_001 => "JUM.num",
            0b01000_001 => "LDD.num",
            0b01000_010 => "LDD.ram",
            0b01000_011 => "LDD.rom",
            0b01001_010 => "STD.ram",
            0b01001_111 => "STD.out",
            0b10000_001 => "ADD.num",
            0b10001_001 => "SUB.num",
            0b10010_001 => "NAN.num",
            0b10100_001 => "SHL.num",
            0b10101_001 => "SHR.num",
            0b10110_001 => "EQU.num",
            0b10111_001 => "GRE.num",
            0b10000_010 => "ADD.ram",
            0b10001_010 => "SUB.ram",
            0b10010_010 => "NAN.ram",
            0b10100_010 => "SHL.ram",
            0b10101_010 => "SHR.ram",
            0b10110_010 => "EQU.ram",
            0b10111_010 => "GRE.ram",
            _ => "ERR",
        });
        println!("  state:");
        println!("    ic:  {:#04x}", cpu.ic);
        println!("    prc: {:#04x}", cpu.prc);
        println!("    ins: {:#04x}", cpu.ins);
        println!("    dta: {:#04x}", cpu.dta);
        println!("    aux: {:#04x}", cpu.aux);
        println!("    adr: {:#04x}", cpu.adr);
        println!("  step: {}", step);

        if step == 2000 {
            break;
        }

        match cpu.ins {
            // 0b0_<INS>_<SUB>
            0b00001_001 => instruction::jum_num(&mut cpu),
            0b01000_001 => instruction::ldd_num(&mut cpu),
            0b01000_010 => instruction::ldd_ram(&mut cpu),
            0b01000_011 => instruction::ldd_rom(&mut cpu),
            0b01001_010 => instruction::std_ram(&mut cpu),
            0b01001_111 => instruction::std_out(&mut cpu),
            0b10000_001 => instruction::alu_num(&mut cpu, AluInstruction::ADD),
            0b10001_001 => instruction::alu_num(&mut cpu, AluInstruction::SUB),
            0b10010_001 => instruction::alu_num(&mut cpu, AluInstruction::NAN),
            0b10100_001 => instruction::alu_num(&mut cpu, AluInstruction::SHL),
            0b10101_001 => instruction::alu_num(&mut cpu, AluInstruction::SHR),
            0b10110_001 => instruction::alu_num(&mut cpu, AluInstruction::EQU),
            0b10111_001 => instruction::alu_num(&mut cpu, AluInstruction::GRE),
            0b10000_010 => instruction::alu_ram(&mut cpu, AluInstruction::ADD),
            0b10001_010 => instruction::alu_ram(&mut cpu, AluInstruction::SUB),
            0b10010_010 => instruction::alu_ram(&mut cpu, AluInstruction::NAN),
            0b10100_010 => instruction::alu_ram(&mut cpu, AluInstruction::SHL),
            0b10101_010 => instruction::alu_ram(&mut cpu, AluInstruction::SHR),
            0b10110_010 => instruction::alu_ram(&mut cpu, AluInstruction::EQU),
            0b10111_010 => instruction::alu_ram(&mut cpu, AluInstruction::GRE),
            _ => instruction::noi(&mut cpu),
        }

        cpu.ic += 0x1;
        if cpu.ic == 0x10 {
            cpu.ic = 0x00;
        }

        step += 1;
    }
}
