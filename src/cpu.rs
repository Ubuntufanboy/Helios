// src/cpu.rs
use std::sync::{Arc, Mutex};
use crate::memory::Memory;
use crate::isa;

// CPU Flags
pub const FLAG_ZERO: u8 = 0b00000001;
pub const FLAG_NEGATIVE: u8 = 0b00000010;
pub const FLAG_CARRY: u8 = 0b00000100;
pub const FLAG_OVERFLOW: u8 = 0b00001000;

pub struct CPU {
    // Registers
    pub a: u8,       // Accumulator
    pub x: u8,       // X index register
    pub y: u8,       // Y index register
    pub pc: u16,     // Program counter
    pub sp: u8,      // Stack pointer (0x00-0xFF, stack at 0x0100-0x01FF)
    pub status: u8,  // Status register (flags)
    
    // Memory
    pub memory: Arc<Mutex<Memory>>,
    
    // State
    pub cycles: u64,
    pub halted: bool,
}

impl CPU {
    pub fn new(memory: Arc<Mutex<Memory>>) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xFF, // Stack starts at the top and grows downward
            status: 0,
            memory,
            cycles: 0,
            halted: false,
        }
    }
    
    pub fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = 0; // Start execution at address 0
        self.sp = 0xFF;
        self.status = 0;
        self.cycles = 0;
        self.halted = false;
    }
    
    pub fn step(&mut self) -> bool {
        if self.halted {
            return false;
        }
        
        // Fetch opcode
        let opcode = self.fetch();
        
        // Execute instruction
        if opcode == 0xDE {
            println!("Saw the OPCODE: 0xDE");
        }
        isa::execute(self, opcode);
        
        // Increment cycle count
        self.cycles += 1;
        
        !self.halted
    }
    
    pub fn fetch(&mut self) -> u8 {
        let memory = self.memory.lock().unwrap();
        let opcode = memory.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        opcode
    }
    
    pub fn read(&self, address: u16) -> u8 {
        let memory = self.memory.lock().unwrap();
        memory.read(address)
    }
    
    pub fn write(&mut self, address: u16, value: u8) {
        let mut memory = self.memory.lock().unwrap();
        memory.write(address, value);
    }
    
    pub fn push(&mut self, value: u8) {
        let address = 0x0100 | (self.sp as u16);
        self.write(address, value);
        self.sp = self.sp.wrapping_sub(1);
    }
    
    pub fn pop(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let address = 0x0100 | (self.sp as u16);
        self.read(address)
    }
    
    pub fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.status |= flag;
        } else {
            self.status &= !flag;
        }
    }
    
    pub fn get_flag(&self, flag: u8) -> bool {
        (self.status & flag) != 0
    }
    
    pub fn update_zero_and_negative_flags(&mut self, value: u8) {
        self.set_flag(FLAG_ZERO, value == 0);
        self.set_flag(FLAG_NEGATIVE, (value & 0x80) != 0);
    }
    
    pub fn halt(&mut self) {
        self.halted = true;
    }
}
