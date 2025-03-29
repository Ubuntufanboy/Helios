// src/memory.rs
pub const ROM_START: usize = 0x0000;
pub const ROM_SIZE: usize = 0x8000;   // 32KB ROM
pub const RAM_START: usize = 0x8000;
pub const RAM_SIZE: usize = 0x7000;   // 28KB RAM
pub const DISPLAY_START: usize = 0xF000;
pub const DISPLAY_SIZE: usize = 0x0C00; // 256x256 pixels, 1 byte per pixel (8 colors)
pub const AUDIO_START: usize = 0xFC00;
pub const AUDIO_SIZE: usize = 0x0100;  // 256 bytes audio buffer
pub const MEMORY_SIZE: usize = 0x10000; // 64KB total address space

pub struct Memory {
    data: [u8; MEMORY_SIZE],
    display_buffer: [u8; DISPLAY_SIZE], // Double buffer for display
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0; MEMORY_SIZE],
            display_buffer: [0; DISPLAY_SIZE],
        }
    }
    
    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }
    
    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
        // When writing to display memory, update the double buffer
        if (address as usize) >= DISPLAY_START && (address as usize) < DISPLAY_START + DISPLAY_SIZE {
            let display_offset = (address as usize) - DISPLAY_START;
            self.display_buffer[display_offset] = value;
        }
    }
    
    pub fn load_program(&mut self, program: &[u8]) {
        for (i, &byte) in program.iter().enumerate() {
            if i < ROM_SIZE {
                self.data[ROM_START + i] = byte;
            } else {
                break;
            }
        }
    }
    
    pub fn get_display_buffer(&self) -> &[u8] {
        &self.data[DISPLAY_START..DISPLAY_START + DISPLAY_SIZE]
    }
    
    pub fn get_audio_buffer(&self) -> &[u8] {
        &self.data[AUDIO_START..AUDIO_START + AUDIO_SIZE]
    }
    
    pub fn swap_display_buffer(&mut self) {
        for i in 0..DISPLAY_SIZE {
            self.data[DISPLAY_START + i] = self.display_buffer[i];
        }
    }
}
