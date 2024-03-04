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
}