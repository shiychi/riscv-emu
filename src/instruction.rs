use crate::cpu::{Cpu, DecodeError};
use anyhow::Result;
use std::convert::TryInto;

pub fn parse(raw: u32) -> Result<Instruction> {
    let opcode = raw & 0b1111111;

    match opcode {
        0x33 => {
            let inst = RTypeInstruction::new(raw);
            inst.parse()
        }
        0x13 => {
            let inst = ITypeInstruction::new(raw);
            inst.parse()
        }
        _ => Err(DecodeError::OpcodeError(opcode as u8).into()),
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add(RTypeInstruction),
    Sub(RTypeInstruction),
    Sll(RTypeInstruction),
    Addi(ITypeInstruction),
}

#[derive(Debug, PartialEq)]
pub struct RTypeInstruction {
    pub opcode: u8,
    pub funct3: u8,
    pub funct7: u8,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
}

#[derive(Debug, PartialEq)]
pub struct ITypeInstruction {
    pub opcode: u8,
    pub funct3: u8,
    pub rd: u8,
    pub rs1: u8,
    pub imm: u32,
}

#[derive(Debug, PartialEq)]
pub struct _STypeInstruction {
    pub opcode: u8,
    pub funct3: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: u32,
}

#[derive(Debug, PartialEq)]
pub struct _UTypeInstruction {
    pub opcode: u8,
    pub rd: u8,
    pub imm: u32,
}

impl RTypeInstruction {
    pub fn new(raw: u32) -> Self {
        let rd: u8 = ((raw >> 7) & 0b11111).try_into().unwrap();
        let funct3: u8 = ((raw >> 12) & 0b111).try_into().unwrap();
        let rs1: u8 = ((raw >> 15) & 0b11111).try_into().unwrap();
        let rs2: u8 = ((raw >> 20) & 0b11111).try_into().unwrap();
        let funct7: u8 = (raw >> 25).try_into().unwrap();

        Self {
            opcode: 0x33,
            funct3,
            funct7,
            rd,
            rs1,
            rs2,
        }
    }

    pub fn parse(self) -> Result<Instruction> {
        match self.funct3 {
            0 => match self.funct7 {
                0 => Ok(Instruction::Add(self)),
                32 => Ok(Instruction::Sub(self)),
                _ => Err(DecodeError::Funct7Error(self.funct7).into()),
            },
            1 => match self.funct7 {
                0 => Ok(Instruction::Sll(self)),
                _ => Err(DecodeError::Funct7Error(self.funct7).into()),
            },
            _ => Err(DecodeError::Funct3Error(self.funct3).into()),
        }
    }
}

impl ITypeInstruction {
    pub fn new(raw: u32) -> Self {
        let rd: u8 = ((raw >> 7) & 0b11111).try_into().unwrap();
        let funct3: u8 = ((raw >> 12) & 0b111).try_into().unwrap();
        let rs1: u8 = ((raw >> 15) & 0b11111).try_into().unwrap();
        let imm: u32 = raw >> 20;

        Self {
            opcode: 0x13,
            funct3,
            rd,
            rs1,
            imm,
        }
    }

    pub fn parse(self) -> Result<Instruction> {
        match self.funct3 {
            0 => Ok(Instruction::Addi(self)),
            _ => Err(DecodeError::Funct3Error(self.funct3).into()),
        }
    }
}

pub fn add(cpu: &mut Cpu, i: RTypeInstruction) {
    let rs1 = cpu.register[i.rs1 as usize];
    let rs2 = cpu.register[i.rs2 as usize];
    cpu.register[i.rd as usize] = rs1 + rs2;
}

pub fn sub(cpu: &mut Cpu, i: RTypeInstruction) {
    let rs1 = cpu.register[i.rs1 as usize];
    let rs2 = cpu.register[i.rs2 as usize];
    cpu.register[i.rd as usize] = rs1 - rs2;
}

pub fn sll(cpu: &mut Cpu, i: RTypeInstruction) {
    let rs1 = cpu.register[i.rs1 as usize];
    let rs2 = cpu.register[i.rs2 as usize];
    cpu.register[i.rd as usize] = rs1 << (rs2 & 0b11111);
}

pub fn addi(cpu: &mut Cpu, i: ITypeInstruction) {
    let rs1 = cpu.register[i.rs1 as usize];
    let imm = i.imm;
    cpu.register[i.rd as usize] = rs1 + imm;
}
