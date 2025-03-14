#![allow(dead_code)]


use macroquad::window::next_frame;

use crate::opcode::map_ops_code;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing,
}

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xFD;


pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    pub stack_pointer : u8,
    memory : [u8;0xFFFF]
 }

 impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
 }
  
 impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x : 0,
            register_y : 0,
            status: 0,
            program_counter: 0,
            stack_pointer : STACK_RESET,
            memory : [0x0;0xFFFF]
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {

        match mode {
            AddressingMode::Immediate => self.program_counter,
 
            AddressingMode::ZeroPage  => self.mem_read(self.program_counter) as u16,
           
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
         
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_x) as u16
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                pos.wrapping_add(self.register_y) as u16
            }
 
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_x as u16)
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                base.wrapping_add(self.register_y as u16)
            }
 
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
 
                let ptr: u8 = base.wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);
 
                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                deref_base.wrapping_add(self.register_y as u16)
            }
          
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }
    //########### STATUS REGISTER MANAGEMENT ##################
    fn get_carry_flag(&self) -> bool {
        self.status & 1 == 1
    }

    fn set_carry_flag(&mut self , value : bool) {
        if value {
            self.status |= 1;
        }
        else {
            self.status &= !1;
        }
    }

    fn get_zero_flag(&self) -> bool {
        (self.status >> 1) & 1 == 1 
    }

    fn set_zero_flag(&mut self , value : bool) {
        if value {
            self.status |= 1<<1;
        }
        else {
            self.status &= !(1<<1);
        }
    }

    fn get_interrupt_disable(&self) -> bool {
        (self.status >> 2) & 1 == 1
    }

    fn set_interrupt_disable(&mut self , value : bool) {
        if value {
            self.status |= 1 << 2;
        }
        else {
            self.status &= !(1 << 2);
        }
    }

    fn get_decimal_mode_flag(&self) -> bool {
        (self.status >> 3) & 1 == 1
    }

    fn set_decimal_mode_flag(&mut self , value : bool) {
        if value {
            self.status |= 1 << 3;
        }
        else {
            self.status &=  !(1 << 3);
        }
    }

    fn get_break_command(&self) -> bool {
        (self.status >> 4) & 1 == 1
    }

    fn set_break_command(&mut self , value : bool) {
        if value {
            self.status |= 1<<4;
        }
        else {
            self.status &= !(1<<4);
        }
    }

    fn get_overflow_flag(&self) -> bool {
        (self.status >> 6) & 1 == 1
    }

    fn set_overflow_flag(&mut self , value : bool) {
        if value {
            self.status |= 1<<6;
        }
        else {
            self.status &= !(1<<6);
        }
    }

    fn get_negative_flag(&self) -> bool {
        (self.status >> 7) & 1 == 1
    }

    fn set_negative_flag(&mut self , value : bool) {
        if value {
            self.status |= 1<<7;
        }
        else {
            self.status &= !(1<<7);
        }
    }
    //####################################################

    //########### MEMORY MANAGEMENT ##################
    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        (hi << 8) | lo
    }
 
    pub fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }

    pub fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    fn stack_push(&mut self , data : u8) {
        self.mem_write(STACK + self.stack_pointer as u16, data);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self , data : u16) {
        let hi = (data <<8) as u8;
        let lo = (data & 0xFF) as u8;

        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.mem_read(STACK + self.stack_pointer as u16)
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let lo = self.stack_pop() as u16;
        let hi = self.stack_pop() as u16;

        hi << 8 | lo
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.stack_pointer = STACK_RESET;
        self.status = 0;
 
        self.program_counter = self.mem_read_u16(0xFFFC);
    }
    //########### INIT ##################
    pub async fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.program_counter = self.mem_read_u16(0xFFFC);
        self.run().await;
    }
    
    

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x600 .. (0x600 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x600);
    }
    //####################################################

    //########### CPU INSTRUCTIONS ##################
    fn lda(&mut self, mode : &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ldx(&mut self , mode : &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn ldy(&mut self , mode : &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn lsr(&mut self , mode : &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

        self.set_carry_flag(value & 1 == 1);

        value >>= 1;
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
        value


    }

    fn cmp(&mut self, mode : &AddressingMode) {
        self.compare(mode, self.register_a);
    }

    fn cpx(&mut self , mode : &AddressingMode) {
        self.compare(mode, self.register_x);
    }

    fn cpy(&mut self , mode : &AddressingMode) {
        self.compare(mode, self.register_x);
    }

    fn adc(&mut self , mode : &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.add_to_register_a(value);
    }

    fn dec(&mut self , mode : &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

        value = value.wrapping_sub(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
        value
    }

    fn inc(&mut self , mode : &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

        value = value.wrapping_add(1);
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
        value
    }


    fn sbc(&mut self , mode : &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.add_to_register_a(((value as i8).wrapping_neg().wrapping_sub(1)) as u8);
    }

    fn and(&mut self , mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a &= value;
        self.update_zero_and_negative_flags(self.register_a); 
    }

    fn ora(&mut self , mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a |= value;
        self.update_zero_and_negative_flags(self.register_a); 
    }

    fn eor(&mut self , mode : &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a ^= value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn rol(&mut self , mode : &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

        let old_carry = self.get_carry_flag();

        self.set_carry_flag(value >> 7 == 1);

        value <<= 1;
        if old_carry {
            value |= 1;
        }

        self.mem_write(addr,value);
        self.update_zero_and_negative_flags(value);
        value

    }

    fn ror(&mut self , mode : &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

        let old_carry = self.get_carry_flag();

        self.set_carry_flag(value & 1 == 1);

        value >>= 1;
        if old_carry {
            value |= 1 << 7;
        }

        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
        value
    }

    fn asl(&mut self , mode : &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let mut value = self.mem_read(addr);

       
        self.set_carry_flag(value >> 7 == 1);
        

        value <<= 1;
        self.mem_write(addr, value);
        self.update_zero_and_negative_flags(value);
        value
    }
  
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn tsx(&mut self) {
        self.register_x = self.stack_pointer;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn txs(&mut self) {
        self.stack_pointer = self.register_x;
    }

    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn dex(&mut self) {
        self.register_x = self.register_x.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        self.register_y = self.register_y.wrapping_sub(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn pha(&mut self) {
        self.stack_push(self.register_a);
    }

    fn php(&mut self) {
        let mut flags = self.status;
        flags |= 1 << 4;
        flags |= 1 << 5;
        self.stack_push(flags);
    }

    fn pla(&mut self) {
        let value = self.stack_pop();

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn plp(&mut self) {
        let mut value = self.stack_pop();

        value &= !(1 << 4);
        value |= 1 << 5;

        self.status = value;
    }

    fn bcc(&mut self) {
        self.branch(!self.get_carry_flag());
    }
    fn bcs(&mut self) {
        self.branch(self.get_carry_flag());
    }

    fn beq(&mut self) {
        self.branch(self.get_zero_flag());
    }

    fn bmi(&mut self) {
        self.branch(self.get_negative_flag());
    }

    fn bne(&mut self) {
        self.branch(!self.get_zero_flag());
    }

    fn bpl(&mut self) {
        self.branch(!self.get_negative_flag());
    }

    fn bvc(&mut self) {
        self.branch(!self.get_overflow_flag());
    }
    fn bvs(&mut self) {
        self.branch(self.get_overflow_flag());
    }

    fn clc(&mut self) {
        self.set_carry_flag(false);
    }

    fn cld(&mut self) {
        self.set_decimal_mode_flag(false);
    }

    fn cli(&mut self) {
        self.set_interrupt_disable(false);
    }

    fn clv(&mut self) {
        self.set_overflow_flag(false);
    }

    fn sec(&mut self) {
        self.set_carry_flag(true);
    }

    fn sed(&mut self) {
        self.set_decimal_mode_flag(true);
    }

    fn sei(&mut self) {
        self.set_interrupt_disable(true);
    }

    fn jmp_abs(&mut self) {
        let addr = self.mem_read_u16(self.program_counter);
        self.program_counter = addr;
    }

    fn jmp_ind(&mut self) {
        let addr = self.mem_read_u16(self.program_counter);

        let indirect_ref = if addr & 0x00FF == 0x00FF {
            let lo = self.mem_read(addr);
            let hi = self.mem_read(addr & 0x00FF);
            (hi as u16) << 8 | (lo as u16)
        }

        else {
            self.mem_read_u16(addr)
        };

        self.program_counter = indirect_ref;
    }

    fn jsr(&mut self) {
        self.stack_push_u16(self.program_counter + 2 - 1);
        let target_addr = self.mem_read_u16(self.program_counter);
        self.program_counter = target_addr;
    }


    fn bit(&mut self , mode : &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let and = self.register_a & value;
       
        self.set_zero_flag(and == 0);
        
        self.set_overflow_flag(value & (1 << 6) > 0);
        self.set_negative_flag(value & (1 << 7) > 0);
    }

    fn rti(&mut self) {
        let mut flags = self.stack_pop();
        flags &= !(1 << 4);
        flags |= 1 << 5;

        self.status = flags;
        self.program_counter = self.stack_pop_u16();
    }

    fn rts(&mut self) {
        self.program_counter = self.stack_pop_u16() + 1;
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn stx(&mut self, mode : &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    fn sty(&mut self, mode : &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }


    //####################################################

    
    //########### HELPER FUNCTIONS ##################
     fn update_zero_and_negative_flags(&mut self, result: u8) {
        //  if result == 0 {
        //     self.status = self.status | 0b0000_0010;
            
        //  } else {
        //     self.status = self.status & 0b1111_1101;
            
        //  }

         self.set_zero_flag(result == 0);
 
        //  if result & 0b1000_0000 != 0 {
        //       self.status = self.status | 0b1000_0000;
        //     self.set_negative_flag(result & 0b1000_0000 != 0);
        //  } else {
        //        self.status = self.status & 0b0111_1111;
        //  }

         self.set_negative_flag(result & (1 << 7) != 0);
     }

     fn add_to_register_a(&mut self , value : u8) {
        let sum = self.register_a as u16 + value as u16 + 
        (if self.get_carry_flag() {
            1
        }
        else {
        0
        });

        let carry = sum > 0xFF;
        if carry {
            self.set_carry_flag(true);
        }
        else {
            self.set_carry_flag(false);
        }
        let result = sum as u8;
        if (value ^ result) & (result ^ self.register_a) & 0x80 != 0 {
            self.set_overflow_flag(true);
        }
        else {
            self.set_overflow_flag(false);
        }
     }

     fn branch(&mut self , cond : bool) {
        if cond {
            let jmp : i8 = self.mem_read(self.program_counter) as i8;
            let jmp_addr = self.program_counter.wrapping_add(1).wrapping_add(jmp as u16);
            self.program_counter = jmp_addr;
        }
     }

     fn compare(&mut self ,mode : &AddressingMode, compare_w : u8) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        let result = compare_w.wrapping_sub(value);
        self.update_zero_and_negative_flags(result);
        self.set_carry_flag(compare_w >= value);
     }
     //####################################################

    pub async fn run(&mut self) {
        self.run_with_callback(|_|{}).await;
    }
  
    pub async fn run_with_callback<F>(&mut self , mut callback : F)
    where 
    F : FnMut(&mut CPU), 
    {
        loop {
            callback(self);
            let code = self.mem_read(self.program_counter);
            let opscode = map_ops_code(code).unwrap();
            self.program_counter += 1;
            let program_state = self.program_counter;
    
            match opscode.code {

                0x0 => return,
                //adc
                0x69 | 0x65 |0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opscode.mode);
                    
                }
                //sbc
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    self.sbc(&opscode.mode);
                    
                }
                //and
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                    self.and(&opscode.mode);
                    
                }
                //ora
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                    self.and(&opscode.mode);
                    
                }
                //eor
                0x49 | 0x45 | 0x55 | 0x4D |0x5D | 0x59 | 0x41 | 0x51 => {
                    self.eor(&opscode.mode);
                }   
                //asl
                0x06 | 0x16 | 0x0E | 0x1E => {
                    self.asl(&opscode.mode);
                    
                }

                //cmp
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                    self.cmp(&opscode.mode);
                }

                //cpx
                0xE0 | 0xE4 | 0xEC => {
                    self.cpx(&opscode.mode);
                }

                //cpy
                0xC0 | 0xC4 | 0xCC => {
                    self.cpy(&opscode.mode);
                }

                //dec
                0xC6 | 0xD6 | 0xCE | 0xDE => {
                    self.dec(&opscode.mode);
                }

                //dec
                0xE6 | 0xF6 | 0xEE | 0xFE => {
                    self.inc(&opscode.mode);
                }

                //lda
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opscode.mode);
                    
                }
                //ldx
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                    self.ldx(&opscode.mode);
                    
                }

                //ldy
                0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                    self.ldy(&opscode.mode);
                    
                }

                //lsr 
                0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                    self.lsr(&opscode.mode);
                }
                //sta
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opscode.mode);
            
                }
                //stx
                0x86 | 0x96 | 0x8E => {
                    self.stx(&opscode.mode);
                }
                //sty
                0x84 | 0x94 | 0x8C => {
                    self.stx(&opscode.mode);
                }
                //bit
                0x24 | 0x2C => {
                    self.bit(&opscode.mode);
                    
                }
                //rol
                0x26 | 0x36 | 0x2E | 0x3E => {
                    self.rol(&opscode.mode);
                }
                //ror
                0x66 | 0x76 | 0x6E | 0x7E => {
                    self.rol(&opscode.mode);
                } 
                //pha
                0x48 => self.pha(),
                //php
                0x08 => self.php(),
                //pla
                0x68 => self.pla(),
                //plp
                0x28 => self.plp(),
                //tax
                0xAA => self.tax(),
                //tay
                0xA8 => self.tay(),
                //tsx
                0xBA => self.tsx(),
                //txa
                0x8A => self.txa(),
                //txs
                0x9A => self.txs(),
                //tya
                0x98 => self.tya(),
                //inx
                0xE8 => self.inx(),
                //iny
                0xC8 => self.iny(),
                //dex
                0xCA => self.dex(),
                //dey
                0x88 => self.dey(),
                //bcc
                0x90 => self.bcc(),
                //bcs
                0xB0 => self.bcs(),
                //beq
                0xF0 => self.beq(),
                //bmi
                0x30 => self.bmi(),
                //bne
                0xD0 => self.bne(),
                //bpl
                0x10 => self.bpl(),
                //bcv
                0x50 => self.bvc(),
                //bvs
                0x70 => self.bvs(),
                //clc
                0x18 => self.clc(),
                //cld
                0xD8 => self.cld(),
                //cli
                0x58 => self.cli(),
                //clv
                0xB8 => self.clv(),
                //sec
                0x38 => self.sec(),
                //sed
                0xF8 => self.sed(),
                //sei
                0x78 => self.sei(),
                //jmp
                0x4C => self.jmp_abs(),
                0x6C => self.jmp_ind(),

                //jsr 
                0x20 => self.jsr(),

                //rts
                0x60 => self.rts(),
                //rti
                0x40 => self.rti(),
                //nop
                0xEA => {}
                
                _ => todo!()
            }

            if program_state == self.program_counter {
                self.program_counter += opscode.len as u16 - 1;
            }
            next_frame().await;
        }
    }
 }