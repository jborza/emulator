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
            Instruction::ADDHL(target) => {
                //just like add, the the target is added to the HL register
                let value = self.registers.c;
                let new_value = self.add(value);
                self.registershl = new_value;
            }
            Instruction::ADC(target) => {
                //just like add, the the target is added to the A register, value of carry also added
                let value = self.registers.c;
                let new_value = self.add(value);
                self.registers.a = new_value + self.flags.carry;
            }
            Instruction::SUB(target) => {
                //just like add, the the target is subtracted from the A register
                let value = self.registers.c;
                let new_value = self.sub(value);
                self.registers.a = new_value;
            }
            Instruction::SBC(target) => {
                //just like add, the the target is subtracted from the A register, value of carry subtracted
                let value = self.registers.c;
                let new_value = self.sub(value);
                self.registers.a = new_value - self.flags.carry;
            }
            Instruction::AND(target){
                //do bitwise AND with the target and the A register
                let value = self.registers.c;
                self.registers.a = self.registers.a & value;
                self.flags.zero = result == 0;
                self.flags.subtract = false;
                self.flags.half_carry = true; 
                self.flags.carry = false;
            }
            Instruction::OR(target){
                //do bitwise OR with the target and the A register
                let value = self.registers.c;
                self.registers.a = self.registers.a | value;
                self.flags.zero = result == 0;
                self.flag.subtract = false;
                self.flags.half_carry = false;
                self.flags.carry = false;
            }
            Instruction::XOR(target){
                //do bitwise XOR with the target and the A register
                let value = self.registers.c;
                self.registers.a = self.registers.a ^ value;
                self.flags.zero = result == 0;
                self.flag.subtract = false; 
                self.flags.half_carry = false;
                self.flags.carry = false;
            }
            Instruction::CP(target){
                //just like sub, the the target is subtracted from the A register, but the result is not stored
                //back into A
                let value = self.registers.c;
                let new_value = self.sub(value);
            }
            Instruction::INC(target){
                //increment the target in a specific register by 1
                match target {
                    ArithmeticTarget::C => {
                        self.registers.c = self.registers.c.wrapping_add(1);
                    }
                    _=> { /* more targets */ }
                }
            }
            Instruction::DEC(target){
                //decrement the target in a specific register by 1
                match target {
                    ArithmeticTarget::C => {
                        self.registers.c = self.registers.c.wrapping_sub(1);
                    }
                    _=> { /* more targets */ }
                }
            }
            Instruction::CCF => {
                //complement the carry flag
                self.flags.carry = !self.flags.carry;
            }
            Instruction::SCF => {
                //set the carry flag
                self.flags.carry = true;
            }
            Instruction::RRA => {
                //rotate the A register right through the carry flag
                let carry = self.flags.carry;
                self.flags.carry = self.registers.a & 1 != 0;
                self.registers.a = (self.registers.a >> 1) | (carry << 7);
            }
            Instruction::RLA => {
                //rotate the A register left through the carry flag
                let carry = self.flags.carry;
                self.flags.carry = self.registers.a & 0x80 != 0;
                self.registers.a = (self.registers.a << 1) | carry;
            }
            Instruction::RRCA => {
                //rotate the A register right (not through the carry flag)
                self.flags.carry = self.registers.a & 1 != 0;
                self.registers.a = (self.registers.a >> 1) | (self.registers.a << 7);
            }
            Instruction::RLCA => {
                //rotate the A register left (not through the carry flag)
                self.flags.carry = self.registers.a & 0x80 != 0;
                self.registers.a = (self.registers.a << 1) | (self.registers.a >> 7);
            }
            Instruction::CPL => {
                //complement the A register
                self.registers.a = !self.registers.a;
            }
            Instruction::BIT => {
                //test the bit in the A register
                let value = self.registers.c;
                self.flags.zero = (self.registers.a & (1 << value)) == 0;
                self.flags.subtract = false;
                self.flags.half_carry = true; //TODO check
            }
            Instruction::RESET => {
                //reset the bit in the A register
                let value = self.registers.c;
                self.registers.a &= !(1 << value);
            }
            Instruction::SET => {
                //set the bit in the A register
                let value = self.registers.c;
                self.registers.a |= 1 << value;
            }
            Instruction::RESET => {
                //reset the bit in the register to 0
                let value = self.registers.c;
                //TODO implement
            }
            Instruction::SET => {
                //set the bit in the register to 1
                let value = self.registers.c;
                //TODO implement
            }
            Instruction::SRL => {
                //shift the register right
                let value = self.registers.c;
                self.flags.carry = value & 1 != 0;
                self.registers.c = value >> 1;
                self.flags.zero = self.registers.c == 0;
                self.flags.subtract = false;
                self.flags.half_carry = false;
            }
            Instruction::RR => {
                //rotate the register right by 1 through the carry flag
                let value = self.registers.c;
                let carry = self.flags.carry;
                self.flags.carry = value & 1 != 0;
                self.registers.c = (value >> 1) | (carry << 7);
                self.flags.zero = self.registers.c == 0;
                self.flags.subtract = false;
                self.flags.half_carry = false;
            }
            Instruction::RL => {
                //rotate the register left by 1 through the carry flag
                let value = self.registers.c;
                let carry = self.flags.carry;
                self.flags.carry = value & 0x80 != 0;
                self.registers.c = (value << 1) | carry;
                self.flags.zero = self.registers.c == 0;
                self.flags.subtract = false;
                self.flags.half_carry = false;
            }
            Instruction::RRC => {
                //rotate the register right (not through the carry flag)
                let value = self.registers.c;
                self.flags.carry = value & 1 != 0;
                self.registers.c = (value >> 1) | (value << 7);
                self.flags.zero = self.registers.c == 0;
                self.flags.subtract = false;
                self.flags.half_carry = false;
            }
            Instruction::RLC => {
                //rotate the register left (not through the carry flag)
                let value = self.registers.c;
                self.flags.carry = value & 0x80 != 0;
                self.registers.c = (value << 1) | (value >> 7);
                self.flags.zero = self.registers.c == 0;
                self.flags.subtract = false;
                self.flags.half_carry = false;
            }
            Instruction::SRA => {
                //aritmetic shift the register right by 1
                let value = self.registers.c;
                self.flags.carry = value & 1 != 0;
                self.registers.c = (value >> 1) | (value & 0x80);
                self.flags.zero = self.registers.c == 0;
                self.flags.subtract = false;
                self.flags.half_carry = false;
            }
            Instruction::SLA => {
                //aritmetic shift the register left by 1
                let value = self.registers.c;
                self.flags.carry = value & 0x80 != 0;
                self.registers.c = value << 1;
                self.flags.zero = self.registers.c == 0;
                self.flags.subtract = false;
                self.flags.half_carry = false;
            }
            Instruction::SWAP => {
                //swap the upper and lower nibbles of the register
                let value = self.registers.c;
                self.registers.c = (value << 4) | (value >> 4);
                self.flags.zero = self.registers.c == 0;
                self.flags.subtract = false;
                self.flags.half_carry = false;
                self.flags.carry = false;
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