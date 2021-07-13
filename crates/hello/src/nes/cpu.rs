use crate::nes::opcodes;
use std::collections::HashMap;

/*
 NES CPU can address 65536 memory cells. It takes
 2 bytes to store an address. NES CPU uses
 Little-Endian addressing rather than Big-Endian.
 That means that the 8 least significant bits of
 an address will be stored before the 8 most
 significant bits.

 For example:

     Real address: 0x8000
     address packed in big-endian:    80 00
     address packed in little-endian: 00 80

 ASM Example:

     LDA $8000   <=>    ad 00 80
*/

pub struct CPU {
  // accumulator
  pub register_a: u8,
  // index x
  pub register_x: u8,
  // index y
  pub register_y: u8,
  // processor status
  pub status: u8,
  pub program_counter: u16,
  // ram
  memory: [u8; 0xFFFF],
}
/*
  NES platform has a special mechanism to mark where
  the CPU should start the execution. Upon inserting
  a new cartridge, the CPU receives a special signal
  called "Reset interrupt" that instructs CPU to:

  - reset the state (registers and flags)
  - set program_counter to the 16-bit address that is stored at 0xFFFC
*/
const DEFAULT_PROGRAM_COUNTER: u16 = 0x8000;

trait Mem {
  fn mem_read(&self, addr: u16) -> u8;

  fn mem_write(&mut self, addr: u16, data: u8);

  fn mem_read_u16(&self, pos: u16) -> u16 {
    let lo = self.mem_read(pos) as u16;
    let hi = self.mem_read(pos + 1) as u16;
    (hi << 8) | (lo as u16)
  }

  fn mem_write_u16(&mut self, pos: u16, data: u16) {
    let hi = (data >> 8) as u8;
    let lo = (data & 0xff) as u8;
    self.mem_write(pos, lo);
    self.mem_write(pos + 1, hi);
  }
}

impl Mem for CPU {
  fn mem_read(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }

  fn mem_write(&mut self, addr: u16, data: u8) {
    self.memory[addr as usize] = data;
  }
}

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

impl CPU {
  pub fn new() -> Self {
    CPU {
      register_a: 0,
      register_x: 0,
      register_y: 0,
      status: 0,
      program_counter: 0,
      memory: [0; 0xFFFF],
    }
  }
  pub fn mem_read(&self, addr: u16) -> u8 {
    self.memory[addr as usize]
  }
  pub fn mem_write(&mut self, addr: u16, data: u8) {
    self.memory[addr as usize] = data;
  }
  fn mem_read_u16(&mut self, pos: u16) -> u16 {
    let lo = self.mem_read(pos) as u16;
    let hi = self.mem_read(pos + 1) as u16;
    (hi << 8) | (lo as u16)
  }

  fn mem_write_u16(&mut self, pos: u16, data: u16) {
    let hi = (data >> 8) as u8;
    let lo = (data & 0xff) as u8;
    self.mem_write(pos, lo);
    self.mem_write(pos + 1, hi);
  }
  pub fn load_and_run(&mut self, program: Vec<u8>) {
    self.load(program);
    self.reset();
    self.run()
  }
  pub fn load(&mut self, program: Vec<u8>) {
    self.memory
      [DEFAULT_PROGRAM_COUNTER as usize..(DEFAULT_PROGRAM_COUNTER as usize + program.len())]
      .copy_from_slice(&program[..]);
    // we dont have cartridge - :trollface:
    self.mem_write_u16(0xFFFC, DEFAULT_PROGRAM_COUNTER)
  }
  pub fn reset(&mut self) {
    self.register_a = 0;
    self.register_x = 0;
    self.register_y = 0;
    self.status = 0;

    self.program_counter = self.mem_read_u16(0xFFFC);
  }
  pub fn run(&mut self) {
    let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

    loop {
      let code = self.mem_read(self.program_counter);
      self.program_counter += 1;
      let program_counter_state = self.program_counter;

      let opcode = opcodes
        .get(&code)
        .expect(&format!("OpCode {:x} is not recognized", code));

      match code {
        // LDA - http://www.obelisk.me.uk/6502/reference.html#LDA
        0xa9 | 0xa5 | 0xb5 | 0xbd | 0xb9 | 0xa1 | 0xb1 | 0xad => {
          self.lda(&opcode.mode);
        }
        // STA - http://www.obelisk.me.uk/6502/reference.html#STA
        0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
          self.sta(&opcode.mode);
        }
        // TAX - http://www.obelisk.me.uk/6502/reference.html#TAX
        0xAA => self.tax(),
        // TAX - http://www.obelisk.me.uk/6502/reference.html#TAX
        0xE8 => self.inx(),
        // BRK - http://www.obelisk.me.uk/6502/reference.html#BRK
        0x00 => return,
        _ => todo!(),
      }

      if program_counter_state == self.program_counter {
        self.program_counter += (opcode.len - 1) as u16;
      }
    }
  }
  fn update_zero_and_negative_flags(&mut self, result: u8) {
    // set status - for zero flag
    if result == 0 {
      self.status = self.status | 0b0000_0010;
    } else {
      self.status = self.status & 0b1111_1101;
    }
    // set status - for negative flag
    if result & 0b1000_0000 != 0 {
      self.status = self.status | 0b1000_0000;
    } else {
      self.status = self.status & 0b0111_1111;
    }
  }
  fn lda(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    let value = self.mem_read(addr);

    self.register_a = value;
    self.update_zero_and_negative_flags(self.register_a);
  }
  fn sta(&mut self, mode: &AddressingMode) {
    let addr = self.get_operand_address(mode);
    self.mem_write(addr, self.register_a);
  }
  fn tax(&mut self) {
    println!("{} vs {}", self.register_a, self.register_x);
    self.register_x = self.register_a;
    println!("{} vs {}", self.register_a, self.register_x);
    self.update_zero_and_negative_flags(self.register_x);
  }

  fn inx(&mut self) {
    self.register_x = self.register_x.wrapping_add(1);
    self.update_zero_and_negative_flags(self.register_x);
  }

  fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
    match mode {
      AddressingMode::Immediate => self.program_counter,

      AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

      AddressingMode::Absolute => self.mem_read_u16(self.program_counter),

      AddressingMode::ZeroPage_X => {
        let pos = self.mem_read(self.program_counter);
        let addr = pos.wrapping_add(self.register_x) as u16;
        addr
      }
      AddressingMode::ZeroPage_Y => {
        let pos = self.mem_read(self.program_counter);
        let addr = pos.wrapping_add(self.register_y) as u16;
        addr
      }

      AddressingMode::Absolute_X => {
        let base = self.mem_read_u16(self.program_counter);
        let addr = base.wrapping_add(self.register_x as u16);
        addr
      }
      AddressingMode::Absolute_Y => {
        let base = self.mem_read_u16(self.program_counter);
        let addr = base.wrapping_add(self.register_y as u16);
        addr
      }

      AddressingMode::Indirect_X => {
        let base = self.mem_read(self.program_counter);

        let ptr: u8 = (base as u8).wrapping_add(self.register_x);
        let lo = self.mem_read(ptr as u16);
        let hi = self.mem_read(ptr.wrapping_add(1) as u16);
        (hi as u16) << 8 | (lo as u16)
      }
      AddressingMode::Indirect_Y => {
        let base = self.mem_read(self.program_counter);

        let lo = self.mem_read(base as u16);
        let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
        let deref_base = (hi as u16) << 8 | (lo as u16);
        let deref = deref_base.wrapping_add(self.register_y as u16);
        deref
      }

      AddressingMode::NoneAddressing => {
        panic!("mode {:?} is not supported", mode);
      }
    }
  }
}
