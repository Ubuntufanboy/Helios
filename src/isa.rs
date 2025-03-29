// src/isa.rs
use crate::cpu::CPU;
use crate::cpu::{FLAG_CARRY, FLAG_ZERO, FLAG_NEGATIVE, FLAG_OVERFLOW};

// OpCodes
const OP_LDA_IMM: u8 = 0xA9; // Load Accumulator (Immediate)
const OP_LDA_ZP: u8 = 0xA5;  // Load Accumulator (Zero Page)
const OP_LDA_ZPX: u8 = 0xB5; // Load Accumulator (Zero Page,X)
const OP_LDA_ABS: u8 = 0xAD; // Load Accumulator (Absolute)
const OP_LDX_IMM: u8 = 0xA2; // Load X Register (Immediate)
const OP_LDY_IMM: u8 = 0xA0; // Load Y Register (Immediate)
const OP_STA_ZP: u8 = 0x85;  // Store Accumulator (Zero Page)
const OP_STA_ZPX: u8 = 0x95; // Store Accumulator (Zero Page,X)
const OP_STA_ABS: u8 = 0x8D; // Store Accumulator (Absolute)
const OP_STX_ZP: u8 = 0x86;  // Store X Register (Zero Page)
const OP_STY_ZP: u8 = 0x84;  // Store Y Register (Zero Page)
const OP_TAX: u8 = 0xAA;     // Transfer Accumulator to X
const OP_TAY: u8 = 0xA8;     // Transfer Accumulator to Y
const OP_TXA: u8 = 0x8A;     // Transfer X to Accumulator
const OP_TYA: u8 = 0x98;     // Transfer Y to Accumulator
const OP_ADC_IMM: u8 = 0x69; // Add with Carry (Immediate)
const OP_SBC_IMM: u8 = 0xE9; // Subtract with Carry (Immediate)
const OP_AND_IMM: u8 = 0x29; // Logical AND (Immediate)
const OP_ORA_IMM: u8 = 0x09; // Logical OR (Immediate)
const OP_EOR_IMM: u8 = 0x49; // Logical Exclusive OR (Immediate)
const OP_INC_ZP: u8 = 0xE6;  // Increment Memory (Zero Page)
const OP_DEC_ZP: u8 = 0xC6;  // Decrement Memory (Zero Page)
const OP_INX: u8 = 0xE8;     // Increment X Register
const OP_INY: u8 = 0xC8;     // Increment Y Register
const OP_DEX: u8 = 0xCA;     // Decrement X Register
const OP_DEY: u8 = 0x88;     // Decrement Y Register
const OP_CMP_IMM: u8 = 0xC9; // Compare Accumulator (Immediate)
const OP_CPX_IMM: u8 = 0xE0; // Compare X Register (Immediate)
const OP_CPY_IMM: u8 = 0xC0; // Compare Y Register (Immediate)
const OP_JMP_ABS: u8 = 0x4C; // Jump (Absolute)
const OP_JSR_ABS: u8 = 0x20; // Jump to Subroutine
const OP_RTS: u8 = 0x60;     // Return from Subroutine
const OP_BEQ: u8 = 0xF0;     // Branch if Equal
const OP_BNE: u8 = 0xD0;     // Branch if Not Equal
const OP_BCS: u8 = 0xB0;     // Branch if Carry Set
const OP_BCC: u8 = 0x90;     // Branch if Carry Clear
const OP_BMI: u8 = 0x30;     // Branch if Minus
const OP_BPL: u8 = 0x10;     // Branch if Plus
const OP_NOP: u8 = 0xEA;     // No Operation
const OP_BRK: u8 = 0x00;     // Break / Force Interrupt
const OP_HLT: u8 = 0xFF;     // Halt (custom opcode for our emulator)
const OP_DBG: u8 = 0xDE;     // Debug stack address

// Audio opcodes
const OP_SND: u8 = 0x42;     // Custom sound opcode

pub fn execute(cpu: &mut CPU, opcode: u8) {
    match opcode {
        OP_LDA_IMM => {
            let value = cpu.fetch();
            cpu.a = value;
            cpu.update_zero_and_negative_flags(cpu.a);
        },
        OP_LDA_ZP => {
            let address = cpu.fetch() as u16;
            cpu.a = cpu.read(address);
            cpu.update_zero_and_negative_flags(cpu.a);
        },
        OP_LDA_ZPX => {
            let zero_page_addr = cpu.fetch();
            let address = zero_page_addr.wrapping_add(cpu.x) as u16;
            cpu.a = cpu.read(address);
            cpu.update_zero_and_negative_flags(cpu.a);
        },
        OP_LDA_ABS => {
            let low = cpu.fetch() as u16;
            let high = cpu.fetch() as u16;
            let address = (high << 8) | low;
            cpu.a = cpu.read(address);
            cpu.update_zero_and_negative_flags(cpu.a);
        },
        OP_LDX_IMM => {
            let value = cpu.fetch();
            cpu.x = value;
            cpu.update_zero_and_negative_flags(cpu.x);
        },
        OP_LDY_IMM => {
            let value = cpu.fetch();
            cpu.y = value;
            cpu.update_zero_and_negative_flags(cpu.y);
        },
        OP_STA_ZP => {
            let address = cpu.fetch() as u16;
            cpu.write(address, cpu.a);
        },
        OP_STA_ZPX => {
            let zero_page_addr = cpu.fetch();
            let address = zero_page_addr.wrapping_add(cpu.x) as u16;
            cpu.write(address, cpu.a);
        },
        OP_STA_ABS => {
            let low = cpu.fetch() as u16;
            let high = cpu.fetch() as u16;
            let address = (high << 8) | low;
            cpu.write(address, cpu.a);
        },
        OP_STX_ZP => {
            let address = cpu.fetch() as u16;
            cpu.write(address, cpu.x);
        },
        OP_STY_ZP => {
            let address = cpu.fetch() as u16;
            cpu.write(address, cpu.y);
        },
        OP_TAX => {
            cpu.x = cpu.a;
            cpu.update_zero_and_negative_flags(cpu.x);
        },
        OP_TAY => {
            cpu.y = cpu.a;
            cpu.update_zero_and_negative_flags(cpu.y);
        },
        OP_TXA => {
            cpu.a = cpu.x;
            cpu.update_zero_and_negative_flags(cpu.a);
        },
        OP_TYA => {
            cpu.a = cpu.y;
            cpu.update_zero_and_negative_flags(cpu.a);
        },
        OP_ADC_IMM => {
            let value = cpu.fetch();
            let carry = if cpu.get_flag(FLAG_CARRY) { 1 } else { 0 };
            
            let result = cpu.a as u16 + value as u16 + carry as u16;
            let overflow = ((cpu.a ^ result as u8) & (value ^ result as u8) & 0x80) != 0;
            
            cpu.a = result as u8;
            cpu.set_flag(FLAG_CARRY, result > 0xFF);
            cpu.set_flag(FLAG_OVERFLOW, overflow);
            cpu.update_zero_and_negative_flags(cpu.a);
        },
        OP_SBC_IMM => {
            let value = cpu.fetch();
            let carry = if cpu.get_flag(FLAG_CARRY) { 0 } else { 1 };
            
            let result = cpu.a as i16 - value as i16 - carry as i16;
            let overflow = ((cpu.a ^ value) & (cpu.a ^ result as u8) & 0x80) != 0;
            
            cpu.a = result as u8;
            cpu.set_flag(FLAG_CARRY, result >= 0);
            cpu.set_flag(FLAG_OVERFLOW, overflow);
            cpu.update_zero_and_negative_flags(cpu.a);
        },
        OP_AND_IMM => {
            let value = cpu.fetch();
            cpu.a &= value;
            cpu.update_zero_and_negative_flags(cpu.a);
        },
        OP_ORA_IMM => {
            let value = cpu.fetch();
            cpu.a |= value;
            cpu.update_zero_and_negative_flags(cpu.a);
        },
        OP_EOR_IMM => {
            let value = cpu.fetch();
            cpu.a ^= value;
            cpu.update_zero_and_negative_flags(cpu.a);
        },
        OP_INC_ZP => {
            let address = cpu.fetch() as u16;
            let value = cpu.read(address).wrapping_add(1);
            cpu.write(address, value);
            cpu.update_zero_and_negative_flags(value);
        },
        OP_DEC_ZP => {
            let address = cpu.fetch() as u16;
            let value = cpu.read(address).wrapping_sub(1);
            cpu.write(address, value);
            cpu.update_zero_and_negative_flags(value);
        },
        OP_INX => {
            cpu.x = cpu.x.wrapping_add(1);
            cpu.update_zero_and_negative_flags(cpu.x);
        },
        OP_INY => {
            cpu.y = cpu.y.wrapping_add(1);
            cpu.update_zero_and_negative_flags(cpu.y);
        },
        OP_DEX => {
            cpu.x = cpu.x.wrapping_sub(1);
            cpu.update_zero_and_negative_flags(cpu.x);
        },
        OP_DEY => {
            cpu.y = cpu.y.wrapping_sub(1);
            cpu.update_zero_and_negative_flags(cpu.y);
        },
        OP_CMP_IMM => {
            let value = cpu.fetch();
            let result = cpu.a.wrapping_sub(value);
            cpu.set_flag(FLAG_CARRY, cpu.a >= value);
            cpu.update_zero_and_negative_flags(result);
        },
        OP_CPX_IMM => {
            let value = cpu.fetch();
            let result = cpu.x.wrapping_sub(value);
            cpu.set_flag(FLAG_CARRY, cpu.x >= value);
            cpu.update_zero_and_negative_flags(result);
        },
        OP_CPY_IMM => {
            let value = cpu.fetch();
            let result = cpu.y.wrapping_sub(value);
            cpu.set_flag(FLAG_CARRY, cpu.y >= value);
            cpu.update_zero_and_negative_flags(result);
        },
        OP_JMP_ABS => {
            let low = cpu.fetch() as u16;
            let high = cpu.fetch() as u16;
            cpu.pc = (high << 8) | low;
        },
        OP_JSR_ABS => {
            let low = cpu.fetch() as u16;
            let high = cpu.fetch() as u16;
            let return_address = cpu.pc - 1;
            
            cpu.push((return_address >> 8) as u8); // Push high byte
            cpu.push(return_address as u8);        // Push low byte
            
            cpu.pc = (high << 8) | low;
        },
        OP_RTS => {
            let low = cpu.pop() as u16;
            let high = cpu.pop() as u16;
            cpu.pc = ((high << 8) | low) + 1;
        },
        OP_BEQ => {
            let offset = cpu.fetch() as i8;
            if cpu.get_flag(FLAG_ZERO) {
                let old_pc = cpu.pc;
                cpu.pc = cpu.pc.wrapping_add(offset as u16);
            }
        },
        OP_BNE => {
            let offset = cpu.fetch() as i8;
            if !cpu.get_flag(FLAG_ZERO) {
                let old_pc = cpu.pc;
                cpu.pc = cpu.pc.wrapping_add(offset as u16);
            }
        },
        OP_BCS => {
            let offset = cpu.fetch() as i8;
            if cpu.get_flag(FLAG_CARRY) {
                let old_pc = cpu.pc;
                cpu.pc = cpu.pc.wrapping_add(offset as u16);
            }
        },
        OP_BCC => {
            let offset = cpu.fetch() as i8;
            if !cpu.get_flag(FLAG_CARRY) {
                let old_pc = cpu.pc;
                cpu.pc = cpu.pc.wrapping_add(offset as u16);
            }
        },
        OP_BMI => {
            let offset = cpu.fetch() as i8;
            if cpu.get_flag(FLAG_NEGATIVE) {
                let old_pc = cpu.pc;
                cpu.pc = cpu.pc.wrapping_add(offset as u16);
            }
        },
        OP_BPL => {
            let offset = cpu.fetch() as i8;
            if !cpu.get_flag(FLAG_NEGATIVE) {
                let old_pc = cpu.pc;
                cpu.pc = cpu.pc.wrapping_add(offset as u16);
            }
        },
        OP_NOP => {
            // No operation
        },
        OP_BRK => {
            // Break / Force Interrupt
            // In our simple emulator, we'll just set the program counter to the next instruction
            cpu.fetch(); // Skip the padding byte
        },
        OP_DBG => {
            println!("DEBUG INSTRUCTION CALLED");
            // Print out value
            let address = cpu.fetch() as u16;
            let value = cpu.read(address); 
            println!("HELIOS DEBUG: Value {} @ {}", value, address);
        },
        OP_SND => {
            // Custom sound opcode
            // Takes a single byte with format: CCNNNNNN where:
            // - CC is the channel number (0-3)
            // - NNNNNN is the MIDI note (0-63)
            let sound_data = cpu.fetch();
            let audio_address = 0xFC00 | (sound_data & 0xFF) as u16;
            cpu.write(audio_address, sound_data);
            println!("Got SND Instruction. Writing {} to {}", sound_data, audio_address);
        },
        OP_HLT => {
            // Halt the CPU
            cpu.halt();
        },
        _ => {
            // Unknown opcode
            println!("Unknown opcode: {:02X} at address {:04X}", opcode, cpu.pc - 1);
            cpu.halt();
        }
    }
}
