pub struct CpuStruct {
    pub dta: u8,
    pub aux: u8,
    pub adr: u8,
    pub ins: u8,
    pub ic: u8,
    pub prc: u16,
    pub swt: bool,
    pub ram: [u8; 256],
    pub rom: [u8; 65536]
}

pub enum AluInstruction {
    NOO,
    ADD,
    SUB,
    EQU,
    GRE,
    SHL,
    SHR
}
