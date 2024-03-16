enum Instruction { 
    ADD(ArithmeticTarget)
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L,
}

impl Instruction{
    fn from_byte(byte: u8) -> Instruction {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            _ => /* TODO Add mapping for the rest of instructions */ None
        }
    }
}