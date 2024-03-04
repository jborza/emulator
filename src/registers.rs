struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
}

impl Registers {
  fn get_bc(&self) -> u16 {
    (self.b as u16) << 8 | self.c as u16
  }

  fn set_bc(&mut self, value: u16) {
    self.b = (value >> 8) as u8;
    self.c = value as u8;
  }
}

struct FlagsRegister {
    //set if the result of operation is 0
    zero: bool,
    //set if the operation was a subtraction
    subtract: bool,
    //set if there was was an oveflow from the lower nibble to the upper nibble
    half_carry: bool,
    //
    carry: bool,
}

const ZERO_BYTE_POSITION: u8 = 7;
const SUBTRACT_BYTE_POSITION: u8 = 6;
const HALF_CARRY_BYTE_POSITION: u8 = 5;
const CARRY_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8{
    fn from(flag: FlagsRegister) -> u8 {
        if(flag.zero {1} else {0}) << ZERO_BYTE_POSITION |
        if(flag.subtract {1} else {0}) << SUBTRACT_BYTE_POSITION |
        if(flag.half_carry {1} else {0}) << HALF_CARRY_BYTE_POSITION |
        if(flag.carry {1} else {0}) << CARRY_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = (byte >> ZERO_BYTE_POSITION) & 1 != 0;
        let subtract = (byte >> SUBTRACT_BYTE_POSITION) & 1 != 0;
        let half_carry = (byte >> HALF_CARRY_BYTE_POSITION) & 1 != 0;
        let carry = (byte >> CARRY_BYTE_POSITION) & 1 != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}