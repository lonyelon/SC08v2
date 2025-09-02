use std::num::Wrapping;

pub struct CpuStruct {
    pub dta: Wrapping<u8>,
    pub aux: Wrapping<u8>,
    pub adr: Wrapping<u8>,
    pub ins: Wrapping<u8>,
    pub ic: Wrapping<u8>,
    pub prc: Wrapping<u16>,
    pub swt: bool,
    pub ram: [Wrapping<u8>; 256],
    pub rom: [Wrapping<u8>; 65536]
}

pub enum AluInstruction {
    ADD,
    SUB,
    NAN,
    SHL,
    SHR,
    EQU,
    GRE,
}
