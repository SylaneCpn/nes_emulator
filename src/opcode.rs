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


pub static  CPU_OPS_CODES : [OpCode ; 18] = [
    OpCode{code : 0x00, inst : "BRK", len : 1, cycles : 7,  page_crossed_cycles : 0,  mode :AddressingMode::NoneAddressing},
    OpCode{ code :0xaa, inst :"TAX", len : 1, cycles : 2,  page_crossed_cycles : 0 , mode : AddressingMode::NoneAddressing },
    OpCode{ code :0xe8,  inst : "INX",  len : 1,  cycles : 2, page_crossed_cycles : 0, mode : AddressingMode::NoneAddressing},

    OpCode {code : 0xa9, inst : "LDA", len : 2,cycles : 2,  page_crossed_cycles : 0 ,  mode :AddressingMode::Immediate},
    OpCode {code : 0xa5, inst :"LDA", len : 2, cycles :3,  page_crossed_cycles : 0 ,  mode :AddressingMode::ZeroPage},
    OpCode{code : 0xb5, inst :" LDA", len :2,  cycles :4,page_crossed_cycles : 0,  mode : AddressingMode::ZeroPage_X},
    OpCode{code : 0xad, inst : "LDA", len : 3,  cycles : 4,  page_crossed_cycles : 0 , mode :AddressingMode::Absolute},
    OpCode{code : 0xbd, inst : "LDA", len : 3, cycles : 4 , page_crossed_cycles : 1 , mode :AddressingMode::Absolute_X},
    OpCode{code : 0xb9, inst : "LDA", len : 3, cycles :4, page_crossed_cycles : 1, mode :AddressingMode::Absolute_Y},
    OpCode{code : 0xa1, inst :"LDA", len : 2, cycles : 6,  page_crossed_cycles : 0, mode : AddressingMode::Indirect_X},
    OpCode {code : 0xb1, inst : "LDA", len : 2, cycles : 5, page_crossed_cycles : 1, mode : AddressingMode::Indirect_Y},

    OpCode{code : 0x85, inst : "STA", len : 2, cycles : 3,  page_crossed_cycles : 0, mode :AddressingMode::ZeroPage},
    OpCode{code : 0x95, inst : "STA", len : 2, cycles : 4, page_crossed_cycles :0, mode :AddressingMode::ZeroPage_X},
    OpCode{code : 0x8d, inst : "STA", len : 3, cycles : 4, page_crossed_cycles : 0, mode :AddressingMode::Absolute},
    OpCode{code : 0x9d, inst : "STA", len : 3, cycles : 5, page_crossed_cycles : 0, mode :AddressingMode::Absolute_X},
    OpCode{code : 0x99, inst : "STA",  len : 3, cycles : 5, page_crossed_cycles : 0 , mode : AddressingMode::Absolute_Y},
    OpCode{code : 0x81, inst : "STA", len : 2, cycles : 6, page_crossed_cycles : 0,  mode : AddressingMode::Indirect_X},
    OpCode{code : 0x91, inst : "STA", len : 2, cycles : 6, page_crossed_cycles : 0, mode : AddressingMode::Indirect_Y},
];

pub fn map_ops_code(code : u8) -> Option<&'static OpCode> {
    for opscode in CPU_OPS_CODES.iter() {
        if opscode.code == code {
            return Some(opscode);
        }
    }
    None
}