impl CPU {
    fn execute(&mut self, instruction: Instruction){
        match instruction{
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                    }
                    _=> { /* more targets */ }
                }
            }
            _=> { /* more instructions */ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.flags.zero = new_value == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.carry = did_overflow;
        // set if adding the lower nibbles and register A is greater than 0xF
        // if it is, then the addition caused a carry from the lower nibble to the upper nibble
        self.registers.flags.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        
        new_value
    }
}