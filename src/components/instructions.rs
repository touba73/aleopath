use core::num;

use crate::ByteCode;
use crate::util;
use super::registers::Register;
use super::types::Literal;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    Abs,
    AbsWrapped,
    Add,
    AddWrapped,
    And,
    AssertEq,
    AssertNeq,
    Call,
    Cast,
    CommitBHP256,
    CommitBHP512,
    CommitBHP768,
    CommitBHP1024,
    CommitPED64,
    CommitPED128,
    Div,
    DivWrapped,
    Double,
    GreaterThan,
    GreaterThanOrEqual,
    HashBHP256,
    HashBHP512,
    HashBHP768,
    HashBHP1024,
    HashPED64,
    HashPED128,
    HashPSD2,
    HashPSD4,
    HashPSD8,
    Inv,
    IsEq,
    IsNeq,
    LessThan,
    LessThanOrEqual,
    Mod,
    Mul,
    MulWrapped,
    Nand,
    Neg,
    Nor,
    Not,
    Or,
    Pow,
    PowWrapped,
    Rem,
    RemWrapped,
    Shl,
    ShlWrapped,
    Shr,
    ShrWrapped,
    Square,
    SquareRoot,
    Sub,
    SubWrapped,
    Ternary,
    Xor,
}

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Abs,
            1 => Self::AbsWrapped,
            2 => Self::Add,
            3 => Self::AddWrapped,
            4 => Self::And,
            5 => Self::AssertEq,
            6 => Self::AssertNeq,
            7 => Self::Call,
            8 => Self::Cast,
            9 => Self::CommitBHP256,
            10 => Self::CommitBHP512,
            11 => Self::CommitBHP768,
            12 => Self::CommitBHP1024,
            13 => Self::CommitPED64,
            14 => Self::CommitPED128,
            15 => Self::Div,
            16 => Self::DivWrapped,
            17 => Self::Double,
            18 => Self::GreaterThan,
            19 => Self::GreaterThanOrEqual,
            20 => Self::HashBHP256,
            21 => Self::HashBHP512,
            22 => Self::HashBHP768,
            23 => Self::HashBHP1024,
            24 => Self::HashPED64,
            25 => Self::HashPED128,
            26 => Self::HashPSD2,
            27 => Self::HashPSD4,
            28 => Self::HashPSD8,
            29 => Self::Inv,
            30 => Self::IsEq,
            31 => Self::IsNeq,
            32 => Self::LessThan,
            33 => Self::LessThanOrEqual,
            34 => Self::Mod,
            35 => Self::Mul,
            36 => Self::MulWrapped,
            37 => Self::Nand,
            38 => Self::Neg,
            39 => Self::Nor,
            40 => Self::Not,
            41 => Self::Or,
            42 => Self::Pow,
            43 => Self::PowWrapped,
            44 => Self::Rem,
            45 => Self::RemWrapped,
            46 => Self::Shl,
            47 => Self::ShlWrapped,
            48 => Self::Shr,
            49 => Self::ShrWrapped,
            50 => Self::Square,
            51 => Self::SquareRoot,
            52 => Self::Sub,
            53 => Self::SubWrapped,
            54 => Self::Ternary,
            55 => Self::Xor,
            _ => unreachable!(),
        }
    }
}

const UNARY: &[Opcode] = &[
    Opcode::Abs,
    Opcode::AbsWrapped,
    Opcode::Double,
    Opcode::Inv,
    Opcode::Neg,
    Opcode::Not,
    Opcode::Square,
    Opcode::SquareRoot,
    Opcode::HashBHP256,
    Opcode::HashBHP512,
    Opcode::HashBHP768,
    Opcode::HashBHP1024,
    Opcode::HashPED64,
    Opcode::HashPED128,
    Opcode::HashPSD2,
    Opcode::HashPSD4,
    Opcode::HashPSD8,
];

const BINARY: &[Opcode] = &[
    Opcode::Add,
    Opcode::AddWrapped,
    Opcode::Sub,
    Opcode::SubWrapped,
    Opcode::Mul,
    Opcode::MulWrapped,
    Opcode::Div,
    Opcode::DivWrapped,
    Opcode::Rem,
    Opcode::RemWrapped,
    Opcode::Pow,
    Opcode::PowWrapped,
    Opcode::Shl,
    Opcode::ShlWrapped,
    Opcode::Shr,
    Opcode::ShrWrapped,
    Opcode::And,
    Opcode::Xor,
    Opcode::Or,
    Opcode::Nand,
    Opcode::Nor,
    Opcode::GreaterThan,
    Opcode::GreaterThanOrEqual,
    Opcode::LessThan,
    Opcode::LessThanOrEqual,
    Opcode::IsEq,
    Opcode::IsNeq,
    Opcode::CommitBHP256,
    Opcode::CommitBHP512,
    Opcode::CommitBHP768,
    Opcode::CommitBHP1024,
    Opcode::CommitPED64,
    Opcode::CommitPED128,
    Opcode::Mod,
];

const ASSERT: &[Opcode] = &[Opcode::AssertEq, Opcode::AssertNeq];
const IS_CHECK: &[Opcode] = &[Opcode::IsEq, Opcode::IsNeq];

#[derive(Debug)]
pub enum Locator {
    Internal(String),
    External((String, String, String))
}

#[derive(Debug)]
pub enum Operand {
    Literal(Literal),
    Register(Register),
    ProgramId(Locator),
    Caller,
}

impl Operand {
    fn read(bytes: &mut ByteCode) -> Self {
        match bytes.read_u8() {
            0 => Self::Literal(Literal::read(bytes)),
            1 => Self::Register(Register::read(bytes)),
            2 => Self::ProgramId(Locator::External(util::read_locator(bytes))),
            3 => Self::Caller,
            _ => unreachable!(),    
        }
    }
}

#[derive(Debug)]
pub enum Output {
    Single(Register),
    Multiple(Vec<Register>),
    None,
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    operands: Option<Vec<Operand>>,
    output: Output,
}

impl Instruction {
    fn read_operands(bytes: &mut ByteCode, n: u8) -> Option<Vec<Operand>> {
        if n == 0 {
            None
        } else {
            Some((0..n).map(|_| Operand::read(bytes)).collect())
        }   
    }

    fn read_call_instruction(bytes: &mut ByteCode) -> Self {
        let callee = match bytes.read_u8() {
            1 => Locator::Internal(util::read_identifier(bytes)),
            _ => Locator::External(util::read_locator(bytes)),
        };

        let num_inputs = bytes.read_u8();
        let operands = Self::read_operands(bytes, num_inputs);
        let num_outputs = bytes.read_u8();
        let output = Output::Multiple((0..num_outputs).map(|_| Register::read(bytes)).collect());
        
        Self {
            opcode: Opcode::Call,
            operands,
            output,
        }
    }

    fn read_assert_instruction(bytes: &mut ByteCode, opcode: Opcode) -> Self {
        Self {
            opcode,
            operands: Self::read_operands(bytes, 2),
            output: Output::None,
        }
    }

    fn read_ternary_instruction(bytes: &mut ByteCode, opcode: Opcode) -> Self {
        Self {
            opcode,
            operands: Self::read_operands(bytes, 3),
            output: Output::Single(Register::read(bytes)),
        }
    }

    fn read_unary_instruction(bytes: &mut ByteCode, opcode: Opcode) -> Self {
        Self {
            opcode,
            operands: Self::read_operands(bytes, 1),
            output: Output::Single(Register::read(bytes)),
        }
    }

    fn read_binary_instruction(bytes: &mut ByteCode, opcode: Opcode) -> Self {
        Self {
            opcode,
            operands: Self::read_operands(bytes, 2),
            output: Output::Single(Register::read(bytes)),
        }
    }

    pub fn read_instructions(bytes: &mut ByteCode) -> (u32, Vec<Self>) {
        let num = bytes.read_u32();
        let instructions = (0..num).map(|_| Self::read(bytes)).collect();
        (num, instructions)
    }

    pub fn read(bytes: &mut ByteCode) -> Self {
        let opcode = Opcode::from(bytes.read_u16());
        match opcode {
            Opcode::Call => Self::read_call_instruction(bytes),
            Opcode::Ternary => Self::read_ternary_instruction(bytes, opcode),
            o if ASSERT.contains(&o) => Self::read_assert_instruction(bytes, opcode),
            o if UNARY.contains(&o) => Self::read_unary_instruction(bytes, opcode),
            o if BINARY.contains(&o) => Self::read_binary_instruction(bytes, opcode),
            _ => unreachable!(),
        }
    }
}