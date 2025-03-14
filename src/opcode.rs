use crate::cpu::AddressingMode;

pub struct OpCode {
    pub code : u8,
    pub inst : &'static str,
    pub len : u8,
    pub cycles : u8,
    pub page_crossed_cycles : u8,
    pub mode : AddressingMode
}

// impl OpCode {
//     fn new(code : u8 , inst : &'static str , len : u8 , cycles : u8 , page_crossed_cycles : u8, mode : AddressingMode) -> Self {
//         Self {code , inst , len , cycles , page_crossed_cycles , mode}
//     }
// }


pub static  CPU_OPS_CODES : [OpCode ; 151] = [
    OpCode{code : 0x00, inst : "BRK", len : 1, cycles : 7,  page_crossed_cycles : 0,  mode :AddressingMode::NoneAddressing},
    OpCode{code : 0xEA, inst : "NOP", len : 1, cycles : 2,  page_crossed_cycles : 0,  mode :AddressingMode::NoneAddressing},
    OpCode{ code :0xAA, inst :"TAX", len : 1, cycles : 2,  page_crossed_cycles : 0 , mode : AddressingMode::NoneAddressing },
    OpCode{ code :0xA8, inst :"TAY", len : 1, cycles : 2,  page_crossed_cycles : 0 , mode : AddressingMode::NoneAddressing },
    OpCode{ code :0xBA, inst :"TSX", len : 1, cycles : 2,  page_crossed_cycles : 0 , mode : AddressingMode::NoneAddressing },
    OpCode{ code :0x8A, inst :"TXA", len : 1, cycles : 2,  page_crossed_cycles : 0 , mode : AddressingMode::NoneAddressing },
    OpCode{ code :0x9A, inst :"TXS", len : 1, cycles : 2,  page_crossed_cycles : 0 , mode : AddressingMode::NoneAddressing },
    OpCode{ code :0x98, inst :"TYA", len : 1, cycles : 2,  page_crossed_cycles : 0 , mode : AddressingMode::NoneAddressing },
    OpCode{ code :0x48,  inst : "PHA",  len : 1,  cycles : 3, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x08,  inst : "PHP",  len : 1,  cycles : 3, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x68,  inst : "PLA",  len : 1,  cycles : 4, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x28,  inst : "PLP",  len : 1,  cycles : 4, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0xE8,  inst : "INX",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0xC8,  inst : "INY",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0xCA,  inst : "DEX",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x88,  inst : "DEY",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x90,  inst : "BCC",  len : 2,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0xB0,  inst : "BCS",  len : 2,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0xF0,  inst : "BEQ",  len : 2,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x30,  inst : "BMI",  len : 2,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0xD0,  inst : "BNE",  len : 2,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x10,  inst : "BPL",  len : 2,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x50,  inst : "BVC",  len : 2,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x70,  inst : "BVS",  len : 2,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x18,  inst : "CLC",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0xD8,  inst : "CLD",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x58,  inst : "CLI",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0xB8,  inst : "CLV",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x38,  inst : "SEC",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0xF8,  inst : "SED",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x78,  inst : "SEI",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x40,  inst : "RTI",  len : 1,  cycles : 6, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},
    OpCode{ code :0x60,  inst : "RTS",  len : 1,  cycles : 6, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},


    OpCode {code : 0x4C, inst :"JMP", len : 3, cycles :3,  page_crossed_cycles : 0 ,  mode :AddressingMode::Absolute},
    OpCode {code : 0x6C, inst :"JMP", len : 3, cycles :5,  page_crossed_cycles : 0 ,  mode :AddressingMode::NoneAddressing},

    OpCode {code : 0x20, inst :"JSR", len : 3, cycles :6,  page_crossed_cycles : 0 ,  mode :AddressingMode::Absolute},

    OpCode {code : 0x24, inst :"BIT", len : 2, cycles :3,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode {code : 0x2C, inst :"BIT", len : 3, cycles :4,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},

    OpCode {code : 0xA9, inst : "LDA", len : 2,cycles : 2,  page_crossed_cycles : 0 ,  mode :AddressingMode::Immediate},
    OpCode {code : 0xA5, inst :"LDA", len : 2, cycles :3,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode{code : 0xB5, inst :" LDA", len :2,  cycles :4,page_crossed_cycles : 0,  mode : AddressingMode::ZeroPage_X},
    OpCode{code : 0xAD, inst : "LDA", len : 3,  cycles : 4,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},
    OpCode{code : 0xBD, inst : "LDA", len : 3, cycles : 4 , page_crossed_cycles : 1 , mode :AddressingMode::Absolute_X},
    OpCode{code : 0xB9, inst : "LDA", len : 3, cycles :4, page_crossed_cycles : 1, mode :AddressingMode::Absolute_Y},
    OpCode{code : 0xA1, inst :"LDA", len : 2, cycles : 6,  page_crossed_cycles : 0, mode : AddressingMode::Indirect_X},
    OpCode {code : 0xB1, inst : "LDA", len : 2, cycles : 5, page_crossed_cycles : 1, mode : AddressingMode::Indirect_Y},

    OpCode {code : 0xA2, inst : "LDX", len : 2,cycles : 2,  page_crossed_cycles : 0 ,  mode :AddressingMode::Immediate},
    OpCode {code : 0xA6, inst :"LDX", len : 2, cycles :3,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode {code : 0xB6, inst :"LDX", len : 2, cycles :4,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage_Y},
    OpCode{code : 0xAE, inst : "LDX", len : 3,  cycles : 4,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},
    OpCode{code : 0xBE, inst : "LDX", len : 3, cycles :4, page_crossed_cycles : 1, mode :AddressingMode::Absolute_Y},

    OpCode {code : 0xA0, inst : "LDY", len : 2,cycles : 2,  page_crossed_cycles : 0 ,  mode :AddressingMode::Immediate},
    OpCode {code : 0xA4, inst :"LDY", len : 2, cycles :3,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode {code : 0xB4, inst :"LDY", len : 2, cycles :4,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0xAC, inst : "LDY", len : 3,  cycles : 4,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},
    OpCode{code : 0xBC, inst : "LDY", len : 3, cycles :4, page_crossed_cycles : 1, mode :AddressingMode::Absolute_X},

    OpCode {code : 0x2A, inst : "ROL", len : 1,cycles : 2,  page_crossed_cycles : 0 ,  mode :AddressingMode::NoneAddressing},
    OpCode {code : 0x26, inst :"ROL", len : 2, cycles :5,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode {code : 0x36, inst :"ROL", len : 2, cycles :6,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0x2E, inst : "ROL", len : 3,  cycles : 6,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},
    OpCode{code : 0x3E, inst : "ROL", len : 3, cycles :7, page_crossed_cycles : 0, mode :AddressingMode::Absolute_X},

    OpCode {code : 0x6A, inst : "ROR", len : 1,cycles : 2,  page_crossed_cycles : 0 ,  mode :AddressingMode::NoneAddressing},
    OpCode {code : 0x66, inst :"ROR", len : 2, cycles :5,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode {code : 0x76, inst :"ROR", len : 2, cycles :6,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0x6E, inst : "ROR", len : 3,  cycles : 6,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},
    OpCode{code : 0x6E, inst : "ROR", len : 3, cycles :7, page_crossed_cycles : 0, mode :AddressingMode::Absolute_X},

    OpCode {code : 0x4A, inst : "LSR", len : 1,cycles : 2,  page_crossed_cycles : 0 ,  mode :AddressingMode::NoneAddressing},
    OpCode {code : 0x46, inst :"LSR", len : 2, cycles :5,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode {code : 0x56, inst :"LSR", len : 2, cycles :6,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0x4E, inst : "LSR", len : 3,  cycles : 6,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},
    OpCode{code : 0x5E, inst : "LSR", len : 3, cycles :7, page_crossed_cycles : 0, mode :AddressingMode::Absolute_X},

    OpCode{code : 0x85, inst : "STA", len : 2, cycles : 3,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage},
    OpCode{code : 0x95, inst : "STA", len : 2, cycles : 4, page_crossed_cycles :0, mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0x8D, inst : "STA", len : 3, cycles : 4, page_crossed_cycles : 0, mode :AddressingMode::Absolute},
    OpCode{code : 0x9D, inst : "STA", len : 3, cycles : 5, page_crossed_cycles : 0, mode :AddressingMode::Absolute_X},
    OpCode{code : 0x99, inst : "STA",  len : 3, cycles : 5, page_crossed_cycles : 0 , mode : AddressingMode::Absolute_Y},
    OpCode{code : 0x81, inst : "STA", len : 2, cycles : 6, page_crossed_cycles : 0,  mode : AddressingMode::Indirect_X},
    OpCode{code : 0x91, inst : "STA", len : 2, cycles : 6, page_crossed_cycles : 0, mode : AddressingMode::Indirect_Y},

    OpCode{code : 0x86, inst : "STX", len : 2, cycles : 3,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage},
    OpCode{code : 0x96, inst : "STX", len : 2, cycles : 4, page_crossed_cycles :0, mode :AddressingMode::ZeroPage_Y},
    OpCode{code : 0x8E, inst : "STX", len : 3, cycles : 4, page_crossed_cycles : 0, mode :AddressingMode::Absolute},

    OpCode{code : 0x84, inst : "STY", len : 2, cycles : 3,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage},
    OpCode{code : 0x94, inst : "STY", len : 2, cycles : 4, page_crossed_cycles :0, mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0x8C, inst : "STY", len : 3, cycles : 4, page_crossed_cycles : 0, mode :AddressingMode::Absolute},

    OpCode{code : 0x69, inst : "ADC", len : 2, cycles : 2,  page_crossed_cycles : 0, mode :AddressingMode::Immediate},
    OpCode{code : 0x65, inst : "ADC", len : 2, cycles : 3,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage},
    OpCode{code : 0x75, inst : "ADC", len : 2, cycles : 4,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0x6D, inst : "ADC", len : 3, cycles : 4,  page_crossed_cycles : 0, mode :AddressingMode::Absolute},
    OpCode{code : 0x7D, inst : "ADC", len : 3, cycles : 4,  page_crossed_cycles : 1, mode :AddressingMode::Absolute_X},
    OpCode{code : 0x79, inst : "ADC", len : 3, cycles : 4,  page_crossed_cycles : 1, mode :AddressingMode::Absolute_Y},
    OpCode{code : 0x61, inst : "ADC", len : 2, cycles : 6,  page_crossed_cycles : 0, mode :AddressingMode::Indirect_X},
    OpCode{code : 0x71, inst : "ADC", len : 2, cycles : 5,  page_crossed_cycles : 1, mode :AddressingMode::Indirect_Y},

    OpCode{code : 0xE9, inst : "SBC", len : 2, cycles : 2,  page_crossed_cycles : 0, mode :AddressingMode::Immediate},
    OpCode{code : 0xE5, inst : "SBC", len : 2, cycles : 3,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage},
    OpCode{code : 0xF5, inst : "SBC", len : 2, cycles : 4,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0xED, inst : "SBC", len : 3, cycles : 4,  page_crossed_cycles : 0, mode :AddressingMode::Absolute},
    OpCode{code : 0xFD, inst : "SBC", len : 3, cycles : 4,  page_crossed_cycles : 1, mode :AddressingMode::Absolute_X},
    OpCode{code : 0xF9, inst : "SBC", len : 3, cycles : 4,  page_crossed_cycles : 1, mode :AddressingMode::Absolute_Y},
    OpCode{code : 0xE1, inst : "SBC", len : 2, cycles : 6,  page_crossed_cycles : 0, mode :AddressingMode::Indirect_X},
    OpCode{code : 0xF1, inst : "SBC", len : 2, cycles : 5,  page_crossed_cycles : 1, mode :AddressingMode::Indirect_Y},

    OpCode{code : 0x29, inst : "AND", len : 2, cycles : 2,  page_crossed_cycles : 0, mode :AddressingMode::Immediate},
    OpCode{code : 0x25, inst : "AND", len : 2, cycles : 3,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage},
    OpCode{code : 0x35, inst : "AND", len : 2, cycles : 4,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0x2D, inst : "AND", len : 3, cycles : 4,  page_crossed_cycles : 0, mode :AddressingMode::Absolute},
    OpCode{code : 0x3D, inst : "AND", len : 3, cycles : 4,  page_crossed_cycles : 1, mode :AddressingMode::Absolute_X},
    OpCode{code : 0x39, inst : "AND", len : 3, cycles : 4,  page_crossed_cycles : 1, mode :AddressingMode::Absolute_Y},
    OpCode{code : 0x21, inst : "AND", len : 2, cycles : 6,  page_crossed_cycles : 0, mode :AddressingMode::Indirect_X},
    OpCode{code : 0x31, inst : "AND", len : 2, cycles : 5,  page_crossed_cycles : 1, mode :AddressingMode::Indirect_Y},

    OpCode{code : 0x09, inst : "ORA", len : 2, cycles : 2,  page_crossed_cycles : 0, mode :AddressingMode::Immediate},
    OpCode{code : 0x05, inst : "ORA", len : 2, cycles : 3,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage},
    OpCode{code : 0x15, inst : "ORA", len : 2, cycles : 4,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0x0D, inst : "ORA", len : 3, cycles : 4,  page_crossed_cycles : 0, mode :AddressingMode::Absolute},
    OpCode{code : 0x1D, inst : "ORA", len : 3, cycles : 4,  page_crossed_cycles : 1, mode :AddressingMode::Absolute_X},
    OpCode{code : 0x19, inst : "ORA", len : 3, cycles : 4,  page_crossed_cycles : 1, mode :AddressingMode::Absolute_Y},
    OpCode{code : 0x01, inst : "ORA", len : 2, cycles : 6,  page_crossed_cycles : 0, mode :AddressingMode::Indirect_X},
    OpCode{code : 0x01, inst : "ORA", len : 2, cycles : 5,  page_crossed_cycles : 1, mode :AddressingMode::Indirect_Y},

    OpCode{code : 0x49, inst : "EOR", len : 2, cycles : 2,  page_crossed_cycles : 0, mode :AddressingMode::Immediate},
    OpCode{code : 0x45, inst : "EOR", len : 2, cycles : 3,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage},
    OpCode{code : 0x55, inst : "EOR", len : 2, cycles : 4,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0x4D, inst : "EOR", len : 3, cycles : 4,  page_crossed_cycles : 0, mode :AddressingMode::Absolute},
    OpCode{code : 0x5D, inst : "EOR", len : 3, cycles : 4,  page_crossed_cycles : 1, mode :AddressingMode::Absolute_X},
    OpCode{code : 0x59, inst : "EOR", len : 3, cycles : 4,  page_crossed_cycles : 1, mode :AddressingMode::Absolute_Y},
    OpCode{code : 0x41, inst : "EOR", len : 2, cycles : 6,  page_crossed_cycles : 0, mode :AddressingMode::Indirect_X},
    OpCode{code : 0x51, inst : "EOR", len : 2, cycles : 5,  page_crossed_cycles : 1, mode :AddressingMode::Indirect_Y},

    OpCode{code : 0x0A, inst : "ASL", len : 1, cycles : 2,  page_crossed_cycles : 0, mode :AddressingMode::NoneAddressing},
    OpCode{code : 0x06, inst : "ASL", len : 2, cycles : 5,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage},
    OpCode{code : 0x16, inst : "ASL", len : 2, cycles : 6,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0x0E, inst : "ASL", len : 3, cycles : 6,  page_crossed_cycles : 0, mode :AddressingMode::Absolute},
    OpCode{code : 0x1E, inst : "ASl", len : 3, cycles : 7,  page_crossed_cycles : 0, mode :AddressingMode::Absolute_X},

    OpCode {code : 0xC9, inst : "CMP", len : 2,cycles : 2,  page_crossed_cycles : 0 ,  mode :AddressingMode::Immediate},
    OpCode {code : 0xC5, inst :"CMP", len : 2, cycles :3,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode{code : 0xD5, inst :"CMP", len :2,  cycles :4,page_crossed_cycles : 0,  mode : AddressingMode::ZeroPage_X},
    OpCode{code : 0xCD, inst : "CMP", len : 3,  cycles : 4,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},
    OpCode{code : 0xDD, inst : "CMP", len : 3, cycles : 4 , page_crossed_cycles : 1 , mode :AddressingMode::Absolute_X},
    OpCode{code : 0xD9, inst : "CMP", len : 3, cycles :4, page_crossed_cycles : 1, mode :AddressingMode::Absolute_Y},
    OpCode{code : 0xC1, inst :"CMP", len : 2, cycles : 6,  page_crossed_cycles : 0, mode : AddressingMode::Indirect_X},
    OpCode {code : 0xD1, inst : "CMP", len : 2, cycles : 5, page_crossed_cycles : 1, mode : AddressingMode::Indirect_Y},

    OpCode {code : 0xE0, inst : "CPX", len : 2,cycles : 2,  page_crossed_cycles : 0 ,  mode :AddressingMode::Immediate},
    OpCode {code : 0xE4, inst :"CPX", len : 2, cycles :3,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode{code : 0xEC, inst : "CPX", len : 3,  cycles : 4,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},

    OpCode {code : 0xC0, inst : "CPY", len : 2,cycles : 2,  page_crossed_cycles : 0 ,  mode :AddressingMode::Immediate},
    OpCode {code : 0xC4, inst :"CPY", len : 2, cycles :3,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode{code : 0xCC, inst : "CPY", len : 3,  cycles : 4,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},

    OpCode {code : 0xC6, inst :"DEC", len : 2, cycles :5,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode{code : 0xD6, inst :"DEC", len :2,  cycles :6,page_crossed_cycles : 0,  mode : AddressingMode::ZeroPage_X},
    OpCode{code : 0xCE, inst : "DEC", len : 3,  cycles : 6,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},
    OpCode{code : 0xDE, inst : "DEC", len : 3, cycles : 7 , page_crossed_cycles : 0 , mode :AddressingMode::Absolute_X},

    OpCode {code : 0xE6, inst :"INC", len : 2, cycles :5,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode{code : 0xF6, inst :"INC", len :2,  cycles :6,page_crossed_cycles : 0,  mode : AddressingMode::ZeroPage_X},
    OpCode{code : 0xEE, inst : "INC", len : 3,  cycles : 6,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},
    OpCode{code : 0xFE, inst : "INC", len : 3, cycles : 7 , page_crossed_cycles : 0 , mode :AddressingMode::Absolute_X},
];

pub fn map_ops_code(code : u8) -> Option<&'static OpCode> {
    CPU_OPS_CODES.iter().find(|&opscode| opscode.code == code)
}