struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte) {
            self.execute(instruction);
        } else {
            panic!("Unknown instruction: {:#04X}", instruction_byte);
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction){
        match instruction{
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::C => {
                        let value = self.registers.c;
                        let new_value = self.add(value);
                        self.registers.a = new_value;
                        self.pc.wrapping_add(1)
                    }
                    _=> { /* more targets */ }
                }
            }
            Instruction::ADDHL(target) => {
                //just like add, the the target is added to the HL register
                let value = self.registers.c;
                let new_value = self.add(value);
                self.registershl = new_value;
                self.flags.zero = result == 0;
                self.flags.subtract = false;
                self.flags.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
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

    // ADDHL (add to HL) - just like ADD except that the target is added to the HL register
    fn addhl(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.hl.overflowing_add(value);
        self.registers.flags.zero = new_value == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.carry = did_overflow;
        // set if adding the lower nibbles and register A is greater than 0xF
        // if it is, then the addition caused a carry from the lower nibble to the upper nibble
        self.registers.flags.half_carry = (self.registers.hl & 0xF) + (value & 0xF) > 0xF;
        
        new_value
    }

    // ADC (add with carry) - just like ADD except that the value of the carry flag is also added to the number
    fn adc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value + self.registers.flags.carry as u8);
        self.registers.flags.zero = new_value == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.carry = did_overflow;
        // set if adding the lower nibbles and register A is greater than 0xF
        // if it is, then the addition caused a carry from the lower nibble to the upper nibble
        self.registers.flags.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        
        new_value
    }

    // SUB (subtract) - subtract the value stored in a specific register with the value in the A register
    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.flags.zero = new_value == 0;
        self.registers.flags.subtract = true;
        self.registers.flags.carry = did_overflow;
        // set if adding the lower nibbles and register A is greater than 0xF
        // if it is, then the addition caused a carry from the lower nibble to the upper nibble
        self.registers.flags.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        
        new_value
    }

    // SBC (subtract with carry) - just like ADD except that the value of the carry flag is also subtracted from the number
    fn sbc(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value - self.registers.flags.carry as u8);
        self.registers.flags.zero = new_value == 0;
        self.registers.flags.subtract = true;
        self.registers.flags.carry = did_overflow;
        // set if adding the lower nibbles and register A is greater than 0xF
        // if it is, then the addition caused a carry from the lower nibble to the upper nibble
        self.registers.flags.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        
        new_value
    }

    // AND (logical and) - do a bitwise and on the value in a specific register and the value in the A register
    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.flags.zero = new_value == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = true; 
        self.registers.flags.carry = false;
        
        new_value
    }

    // OR (logical or) - do a bitwise or on the value in a specific register and the value in the A register
    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.flags.zero = new_value == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = false;
        self.registers.flags.carry = false;
        
        new_value
    }

    // XOR (logical xor) - do a bitwise xor on the value in a specific register and the value in the A register
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.flags.zero = new_value == 0;
        self.registers.flags.subtract = false; 
        self.registers.flags.half_carry = false;
        self.registers.flags.carry = false;
        
        new_value
    }

    // CP (compare) - just like SUB except the result of the subtraction is not stored back into A
    fn cp(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a.overflowing_sub(value).0;
        self.registers.flags.zero = new_value == 0;
        self.registers.flags.subtract = true;
        self.registers.flags.carry = new_value > self.registers.a;
        self.registers.flags.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        
        new_value
    }

    // INC (increment) - increment the value in a specific register by 1
    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.flags.zero = new_value == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = (value & 0xF) + 1 > 0xF;
        
        new_value
    }

    // DEC (decrement) - decrement the value in a specific register by 1
    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.flags.zero = new_value == 0;
        self.registers.flags.subtract = true;
        self.registers.flags.half_carry = (value & 0xF) < 1;
        
        new_value
    }

    // CCF (complement carry flag) - toggle the value of the carry flag
    fn ccf(&mut self) {
        self.registers.flags.carry = !self.registers.flags.carry;
    }

    // SCF (set carry flag) - set the carry flag to true
    fn scf(&mut self) {
        self.registers.flags.carry = true;
    }

    // RRA (rotate right A register) - bit rotate A register right through the carry flag
    fn rra(&mut self) {
        let carry = self.registers.flags.carry;
        self.registers.flags.carry = self.registers.a & 1 != 0;
        self.registers.a = (self.registers.a >> 1) | (carry << 7);
    }

    // RLA (rotate left A register) - bit rotate A register left through the carry flag
    fn rla(&mut self) {
        let carry = self.registers.flags.carry;
        self.registers.flags.carry = self.registers.a & 0x80 != 0;
        self.registers.a = (self.registers.a << 1) | carry;
    }

    // RRCA (rotate right A register) - bit rotate A register right (not through the carry flag)
    fn rrca(&mut self) {
        self.registers.flags.carry = self.registers.a & 1 != 0;
        self.registers.a = (self.registers.a >> 1) | (self.registers.a << 7);
    }

    // RRLA (rotate left A register) - bit rotate A register left (not through the carry flag)
    fn rrla(&mut self) {
        self.registers.flags.carry = self.registers.a & 0x80 != 0;
        self.registers.a = (self.registers.a << 1) | (self.registers.a >> 7);
    }

    // CPL (complement) - toggle every bit of the A register
    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
    }

    // BIT (bit test) - test to see if a specific bit of a specific register is set
    fn bit(&mut self, value: u8) {
        self.registers.flags.zero = (self.registers.a & (1 << value)) == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = true; //TODO check
    }

    // RESET (bit reset) - set a specific bit of a specific register to 0
    fn reset(&mut self, value: u8) {
        self.registers.a &= !(1 << value);
    }

    // SET (bit set) - set a specific bit of a specific register to 1
    fn set(&mut self, value: u8) {
        self.registers.a |= 1 << value;
    }

    // SRL (shift right logical) - bit shift a specific register right by 1
    fn srl(&mut self, value: u8) {
        self.registers.flags.carry = value & 1 != 0;
        self.registers.a = value >> 1;
        self.registers.flags.zero = self.registers.a == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = false;
    }

    // RR (rotate right) - bit rotate a specific register right by 1 through the carry flag
    fn rr(&mut self, value: u8) {
        let carry = self.registers.flags.carry;
        self.registers.flags.carry = value & 1 != 0;
        self.registers.a = (value >> 1) | (carry << 7);
        self.registers.flags.zero = self.registers.a == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = false;
    }

    // RL (rotate left) - bit rotate a specific register left by 1 through the carry flag
    fn rl(&mut self, value: u8) {
        let carry = self.registers.flags.carry;
        self.registers.flags.carry = value & 0x80 != 0;
        self.registers.a = (value << 1) | carry;
        self.registers.flags.zero = self.registers.a == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = false;
    }
    
    // RRC (rotate right) - bit rotate a specific register right by 1 (not through the carry flag)
    fn rrc(&mut self, value: u8) {
        self.registers.flags.carry = value & 1 != 0;
        self.registers.a = (value >> 1) | (value << 7);
        self.registers.flags.zero = self.registers.a == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = false;
    }

    // RLC (rotate left) - bit rotate a specific register left by 1 (not through the carry flag)
    fn rlc(&mut self, value: u8) {
        self.registers.flags.carry = value & 0x80 != 0;
        self.registers.a = (value << 1) | (value >> 7);
        self.registers.flags.zero = self.registers.a == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = false;
    }
    
    // SRA (shift right arithmetic) - arithmetic shift a specific register right by 1
    fn sra(&mut self, value: u8) {
        self.registers.flags.carry = value & 1 != 0;
        self.registers.a = (value >> 1) | (value & 0x80);
        self.registers.flags.zero = self.registers.a == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = false;
    }

    // SLA (shift left arithmetic) - arithmetic shift a specific register left by 1
    fn sla(&mut self, value: u8) {
        self.registers.flags.carry = value & 0x80 != 0;
        self.registers.a = value << 1;
        self.registers.flags.zero = self.registers.a == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = false;
    }
    
    // SWAP (swap nibbles) - switch upper and lower nibble of a specific register
    fn swap(&mut self, value: u8) {
        self.registers.a = (value << 4) | (value >> 4);
        self.registers.flags.zero = self.registers.a == 0;
        self.registers.flags.subtract = false;
        self.registers.flags.half_carry = false;
        self.registers.flags.carry = false;
    }
}