// src/compiler.rs
use std::collections::HashMap;

pub fn compile(source: &str) -> Result<Vec<u8>, String> {
    let mut binary = Vec::new();
    let mut labels = HashMap::new();
    let mut unresolved_jumps = Vec::new();

    // First pass: Collect all labels
    let mut current_address = 0;
    for (line_num, line) in source.lines().enumerate() {
        let line_num = line_num + 1; // 1-based line numbering
        let line = line.trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with(';') {
            continue;
        }

        // Check if line has a label
        if line.ends_with(':') {
            let label = line[..line.len() - 1].trim();
            labels.insert(label.to_string(), current_address);
        } else if !line.starts_with('.') { // Not a directive
            // Count the bytes for the instruction
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            match tokens[0].to_uppercase().as_str() {
                // Single byte instructions
                "NOP" | "TAX" | "TAY" | "TXA" | "TYA" | "INX" | "INY" | "DEX" | "DEY" | "RTS" | "BRK" | "HLT" => {
                    current_address += 1;
                },

                // Two or three byte instructions (opcode + operand)
                "LDA" | "LDX" | "LDY" | "STA" | "STX" | "STY" | "ADC" | "SBC" | "AND" | "ORA" | "EOR" |
                "INC" | "DEC" | "CMP" | "CPX" | "CPY" | "BEQ" | "BNE" | "BCS" | "BCC" | "BMI" | "BPL" | "DBG" | "SND" => {
                    if tokens.len() < 2 {
                        return Err(format!("Line {}: Missing operand for instruction: {}", line_num, line));
                    }

                    let operand = tokens[1];
                    current_address += get_instruction_size(tokens[0], operand)?;
                },

                // Three byte instructions (opcode + 2 byte operand)
                "JMP" | "JSR" => {
                    if tokens.len() < 2 {
                        return Err(format!("Line {}: Missing operand for instruction: {}", line_num, line));
                    }
                    current_address += 3;
                },

                _ => {
                    return Err(format!("Line {}: Unknown instruction: {}", line_num, tokens[0]));
                }
            }
        }
    }

    // Second pass: Generate binary code
    current_address = 0;
    for (line_num, line) in source.lines().enumerate() {
        let line_num = line_num + 1; // 1-based line numbering
        let line = line.trim();

        // Skip empty lines, comments, and labels
        if line.is_empty() || line.starts_with(';') || line.ends_with(':') || line.starts_with('.') {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }
        
        let instruction = tokens[0].to_uppercase();

        match instruction.as_str() {
            "NOP" => binary.push(0xEA),
            "BRK" => binary.push(0x00),
            "HLT" => binary.push(0xFF),
            "TAX" => binary.push(0xAA),
            "TAY" => binary.push(0xA8),
            "TXA" => binary.push(0x8A),
            "TYA" => binary.push(0x98),
            "INX" => binary.push(0xE8),
            "INY" => binary.push(0xC8),
            "DEX" => binary.push(0xCA),
            "DEY" => binary.push(0x88),
            "RTS" => binary.push(0x60),

            "LDA" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for LDA", line_num));
                }
                let operand = tokens[1];
                compile_lda(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "LDX" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for LDX", line_num));
                }
                let operand = tokens[1];
                compile_ldx(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "LDY" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for LDY", line_num));
                }
                let operand = tokens[1];
                compile_ldy(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },

            "STA" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for STA", line_num));
                }
                let operand = tokens;
                compile_sta(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "STX" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for STX", line_num));
                }
                let operand = tokens[1];
                compile_stx(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "STY" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for STY", line_num));
                }
                let operand = tokens[1];
                compile_sty(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },

            "ADC" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for ADC", line_num));
                }
                let operand = tokens[1];
                compile_adc(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "SBC" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for SBC", line_num));
                }
                let operand = tokens[1];
                compile_sbc(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "AND" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for AND", line_num));
                }
                let operand = tokens[1];
                compile_and(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "ORA" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for ORA", line_num));
                }
                let operand = tokens[1];
                compile_ora(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "EOR" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for EOR", line_num));
                }
                let operand = tokens[1];
                compile_eor(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "INC" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for INC", line_num));
                }
                let operand = tokens[1];
                compile_inc(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "DEC" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for DEC", line_num));
                }
                let operand = tokens[1];
                compile_dec(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "CMP" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for CMP", line_num));
                }
                let operand = tokens[1];
                compile_cmp(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "CPX" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for CPX", line_num));
                }
                let operand = tokens[1];
                compile_cpx(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "CPY" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for CPY", line_num));
                }
                let operand = tokens[1];
                compile_cpy(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "JMP" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for JMP", line_num));
                }
                let operand = tokens[1];
                binary.push(0x4C);

                if operand.starts_with('$') {
                    // Absolute address
                    parse_and_push_value(&mut binary, operand, 2, line_num)?;
                } else {
                    // Label
                    if let Some(&address) = labels.get(operand) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        // Unresolved label, add to list for second pass
                        unresolved_jumps.push((binary.len(), operand.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            },
            "JSR" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for JSR", line_num));
                }
                let operand = tokens[1];
                binary.push(0x20);

                if operand.starts_with('$') {
                    // Absolute address
                    parse_and_push_value(&mut binary, operand, 2, line_num)?;
                } else {
                    // Label
                    if let Some(&address) = labels.get(operand) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        // Unresolved label, add to list for second pass
                        unresolved_jumps.push((binary.len(), operand.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            },
            "BEQ" | "BNE" | "BCS" | "BCC" | "BMI" | "BPL" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for branch instruction", line_num));
                }
                let operand = tokens[1];
                let opcode = match instruction.as_str() {
                    "BEQ" => 0xF0,
                    "BNE" => 0xD0,
                    "BCS" => 0xB0,
                    "BCC" => 0x90,
                    "BMI" => 0x30,
                    "BPL" => 0x10,
                    _ => unreachable!(),
                };

                binary.push(opcode);

                if operand.starts_with('$') {
                    // Relative address (branch target is PC + offset)
                    let target = parse_value(operand, line_num)?;
                    let offset = (target as i32 - (current_address + 2) as i32) as i8;
                    binary.push(offset as u8);
                } else {
                    // Label
                    if let Some(&address) = labels.get(operand) {
                        let offset = (address as i32 - (current_address + 2) as i32) as i8;
                        binary.push(offset as u8);
                    } else {
                        // Unresolved label, add to list for second pass
                        unresolved_jumps.push((binary.len(), operand.to_string(), 1));
                        binary.push(0);
                    }
                }
            },
            "DBG" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for DBG", line_num));
                }
                let operand = tokens[1];
                compile_dbg(&mut binary, &mut unresolved_jumps, operand, current_address, &labels, line_num)?;
            },
            "SND" => {
                if tokens.len() < 2 {
                    return Err(format!("Line {}: Missing operand for SND", line_num));
                }
                let operand = tokens[1];
                binary.push(0x42); // Custom sound opcode
                parse_and_push_value(&mut binary, operand, 1, line_num)?;
            },
            _ => {
                return Err(format!("Line {}: Unknown instruction: {}", line_num, instruction));
            }
        }

        // Update current address
        current_address += binary.len() as u16;
    }

    // Resolve unresolved jumps
    for (position, label, size) in unresolved_jumps {
        if let Some(&address) = labels.get(&label) {
            if size == 1 {
                // Relative branch
                let target_address = position as u16 + 1;
                let offset = (address as i32 - target_address as i32) as i8;
                binary[position] = offset as u8;
            } else {
                // Absolute address (JMP/JSR)
                binary[position] = (address & 0xFF) as u8;
                binary[position + 1] = (address >> 8) as u8;
            }
        } else {
            return Err(format!("Undefined label: {}", label));
        }
    }

    Ok(binary)
}

fn get_instruction_size(instr: &str, operand: &str) -> Result<u16, String> {
    let instr = instr.to_uppercase();
    
    // Branch instructions are always 2 bytes
    if ["BEQ", "BNE", "BCS", "BCC", "BMI", "BPL"].contains(&instr.as_str()) {
        return Ok(2);
    }
    
    // JMP and JSR are always 3 bytes
    if ["JMP", "JSR"].contains(&instr.as_str()) {
        return Ok(3);
    }
    
    // Determine size by addressing mode
    if operand.starts_with('#') {
        // Immediate: always 2 bytes
        Ok(2)
    } else if operand.starts_with('(') && operand.ends_with("),Y") {
        // Indirect Indexed: always 2 bytes
        Ok(2)
    } else if operand.starts_with('(') && operand.ends_with(",X)") {
        // Indexed Indirect: always 2 bytes
        Ok(2)
    } else if operand.contains(',') {
        // Various indexed modes: typically 2 bytes for ZP, 3 for absolute
        let parts: Vec<&str> = operand.split(',').collect();
        let addr_part = parts[0].trim();
        
        if addr_part.starts_with('$') {
            let is_zp = addr_part.len() <= 3; // $XX (ZP) vs $XXXX (Absolute)
            Ok(if is_zp { 2 } else { 3 })
        } else {
            // Assume it's a label, which will be absolute (3 bytes)
            Ok(3)
        }
    } else if operand.starts_with('$') {
        // Direct addressing: depends on length of operand
        let is_zp = operand.len() <= 3; // $XX (ZP) vs $XXXX (Absolute)
        Ok(if is_zp { 2 } else { 3 })
    } else {
        // Assume it's a label, which will be absolute (3 bytes)
        Ok(3)
    }
}

fn parse_value(value_str: &str, line_num: usize) -> Result<u16, String> {
    if value_str.starts_with('$') {
        // Hexadecimal
        u16::from_str_radix(&value_str[1..], 16)
            .map_err(|_| format!("Line {}: Invalid hexadecimal value: {}", line_num, value_str))
    } else if value_str.starts_with('%') {
        // Binary
        u16::from_str_radix(&value_str[1..], 2)
            .map_err(|_| format!("Line {}: Invalid binary value: {}", line_num, value_str))
    } else {
        // Decimal
        value_str.parse::<u16>()
            .map_err(|_| format!("Line {}: Invalid decimal value: {}", line_num, value_str))
    }
}

fn parse_and_push_value(binary: &mut Vec<u8>, value_str: &str, num_bytes: usize, line_num: usize) -> Result<(), String> {
    let value = parse_value(value_str, line_num)?;

    if num_bytes == 1 {
        if value > 0xFF {
            return Err(format!("Line {}: Value {} is too large for a single byte", line_num, value));
        }
        binary.push((value & 0xFF) as u8);
    } else {
        binary.push((value & 0xFF) as u8);
        binary.push((value >> 8) as u8);
    }
    
    Ok(())
}

// Compile individual instructions with all their addressing modes

fn compile_lda(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        // Immediate
        binary.push(0xA9);
        parse_and_push_value(binary, &operand[1..], 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with("),Y") {
        // Indirect Indexed
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0xB1);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with(",X)") {
        // Indexed Indirect
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0xA1);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.contains(',') {
        // Zero Page,X or Absolute,X or Absolute,Y
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        
        let addr_value = if addr_part.starts_with('$') {
            // Parse address value
            Some(parse_value(addr_part, line_num)?)
        } else {
            None // Label
        };
        
        if index_part == "X" {
            match addr_value {
                Some(addr) if addr <= 0xFF => {
                    // Zero Page,X
                    binary.push(0xB5);
                    binary.push(addr as u8);
                },
                Some(_) => {
                    // Absolute,X
                    binary.push(0xBD);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    // Label,X (assume absolute)
                    binary.push(0xBD);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else if index_part == "Y" {
            match addr_value {
                Some(addr) if addr <= 0xFF => {
                    // No Zero Page,Y for LDA, use Absolute,Y
                    binary.push(0xB9);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                Some(_) => {
                    // Absolute,Y
                    binary.push(0xB9);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    // Label,Y (assume absolute)
                    binary.push(0xB9);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else {
            return Err(format!("Line {}: Invalid index register: {}", line_num, index_part));
        }
    } else if operand.starts_with('$') {
        // Zero Page or Absolute
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            // Zero Page
            binary.push(0xA5);
            binary.push(value as u8);
        } else {
            // Absolute
            binary.push(0xAD);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        // Assume it's a label (Absolute)
        binary.push(0xAD);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    
    Ok(())
}

fn compile_ldx(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        // Immediate
        binary.push(0xA2);
        parse_and_push_value(binary, &operand[1..], 1, line_num)?;
    } else if operand.contains(',') {
        // Must be Zero Page,Y or Absolute,Y for LDX
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        
        if index_part != "Y" {
            return Err(format!("Line {}: LDX only supports Y-indexed addressing, got: {}", line_num, index_part));
        }
        
        let addr_value = if addr_part.starts_with('$') {
            // Parse address value
            Some(parse_value(addr_part, line_num)?)
        } else {
            None // Label
        };
        
        match addr_value {
            Some(addr) if addr <= 0xFF => {
                // Zero Page,Y
                binary.push(0xB6);
                binary.push(addr as u8);
            },
            Some(_) => {
                // Absolute,Y
                binary.push(0xBE);
                parse_and_push_value(binary, addr_part, 2, line_num)?;
            },
            None => {
                // Label,Y (assume absolute)
                binary.push(0xBE);
                if let Some(&address) = labels.get(addr_part) {
                    binary.push((address & 0xFF) as u8);
                    binary.push((address >> 8) as u8);
                } else {
                    unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                    binary.push(0);
                    binary.push(0);
                }
            }
        }
    } else if operand.starts_with('$') {
        // Zero Page or Absolute
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            // Zero Page
            binary.push(0xA6);
            binary.push(value as u8);
        } else {
            // Absolute
            binary.push(0xAE);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        // Assume it's a label (Absolute)
        binary.push(0xAE);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    
    Ok(())
}

fn compile_ldy(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        // Immediate
        binary.push(0xA0);
        parse_and_push_value(binary, &operand[1..], 1, line_num)?;
    } else if operand.contains(',') {
        // Must be Zero Page,X or Absolute,X for LDY
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        
        if index_part != "X" {
            return Err(format!("Line {}: LDY only supports X-indexed addressing, got: {}", line_num, index_part));
        }
        
        let addr_value = if addr_part.starts_with('$') {
            // Parse address value
            Some(parse_value(addr_part, line_num)?)
        } else {
            None // Label
        };
        
        match addr_value {
            Some(addr) if addr <= 0xFF => {
                // Zero Page,X
                binary.push(0xB4);
                binary.push(addr as u8);
            },
            Some(_) => {
                // Absolute,X
                binary.push(0xBC);
                parse_and_push_value(binary, addr_part, 2, line_num)?;
            },
            None => {
                // Label,X (assume absolute)
                binary.push(0xBC);
                if let Some(&address) = labels.get(addr_part) {
                    binary.push((address & 0xFF) as u8);
                    binary.push((address >> 8) as u8);
                } else {
                    unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                    binary.push(0);
                    binary.push(0);
                }
            }
        }
    } else if operand.starts_with('$') {
        // Zero Page or Absolute
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            // Zero Page
            binary.push(0xA4);
            binary.push(value as u8);
        } else {
            // Absolute
            binary.push(0xAC);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        // Assume it's a label (Absolute)
        binary.push(0xAC);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    
    Ok(())
}

fn compile_sta(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operands: Vec<&str>,  // Changed from single operand to slice of operands
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    // Assume the first operand is the address/operand
    let operand = operands[0].split(';').next().unwrap().trim();

    if operand.starts_with('#') {
        return Err(format!("Line {}: STA does not support immediate addressing", line_num));
    } else if operand.starts_with('(') && operand.ends_with(",X)") {
        // (Indirect,X)
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0x81);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with("),Y") {
        // (Indirect),Y
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0x91);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.contains(',') {
        // Indexed addressing (Zero Page,X or Absolute,X / Absolute,Y)
        let parts: Vec<&str> = operand.split(&[',', ' '])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
                
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        let addr_value = if addr_part.starts_with('$') {
            Some(parse_value(addr_part, line_num)?)
        } else {
            None
        };
        if index_part == "X" {
            match addr_value {
                Some(addr) if addr <= 0xFF => {
                    // Zero Page,X
                    binary.push(0x95);
                    binary.push(addr as u8);
                },
                Some(_) => {
                    // Absolute,X
                    binary.push(0x9D);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0x9D);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else if index_part == "Y" {
            // STA does not support a zero page,Y mode; only absolute,Y is allowed.
            binary.push(0x99);
            match addr_value {
                Some(addr) => {
                    binary.push((addr & 0xFF) as u8);
                    binary.push((addr >> 8) as u8);
                }
                None => {
                    if let Some(&address) = labels.get(addr_part){
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    }
                    else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else {
            return Err(format!("Line {}: Invalid index register: {}", line_num, index_part));
        }
    } else if operand.starts_with('$') {
        // Zero Page or Absolute
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0x85);
            binary.push(value as u8);
        } else {
            binary.push(0x8D);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        // Label (Absolute)
        binary.push(0x8D);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_stx(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        return Err(format!("Line {}: STX does not support immediate addressing", line_num));
    } else if operand.contains(',') {
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        if index_part != "Y" {
            return Err(format!("Line {}: STX only supports Y-indexed addressing", line_num));
        }
        let addr_value = if addr_part.starts_with('$') {
            Some(parse_value(addr_part, line_num)?)
        } else {
            None
        };
        match addr_value {
            Some(addr) if addr <= 0xFF => {
                binary.push(0x96);
                binary.push(addr as u8);
            },
            Some(_) => {
                // Absolute,Y is not defined for STX.
                return Err(format!("Line {}: STX does not support absolute,Y addressing", line_num));
            },
            None => {
                binary.push(0x96);
                if let Some(&address) = labels.get(addr_part) {
                    binary.push((address & 0xFF) as u8);
                } else {
                    unresolved_jumps.push((binary.len(), addr_part.to_string(), 1));
                    binary.push(0);
                }
            }
        }
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0x86);
            binary.push(value as u8);
        } else {
            binary.push(0x8E);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0x8E);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_sty(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        return Err(format!("Line {}: STY does not support immediate addressing", line_num));
    } else if operand.contains(',') {
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        if index_part != "X" {
            return Err(format!("Line {}: STY only supports X-indexed addressing", line_num));
        }
        let addr_value = if addr_part.starts_with('$') {
            Some(parse_value(addr_part, line_num)?)
        } else {
            None
        };
        match addr_value {
            Some(addr) if addr <= 0xFF => {
                binary.push(0x94);
                binary.push(addr as u8);
            },
            Some(_) => {
                // Absolute,X is not supported for STY.
                return Err(format!("Line {}: STY does not support absolute,X addressing", line_num));
            },
            None => {
                binary.push(0x94);
                if let Some(&address) = labels.get(addr_part) {
                    binary.push((address & 0xFF) as u8);
                } else {
                    unresolved_jumps.push((binary.len(), addr_part.to_string(), 1));
                    binary.push(0);
                }
            }
        }
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0x84);
            binary.push(value as u8);
        } else {
            binary.push(0x8C);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0x8C);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_adc(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        binary.push(0x69);
        parse_and_push_value(binary, &operand[1..], 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with(",X)") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0x61);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with("),Y") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0x71);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.contains(',') {
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        let addr_value = if addr_part.starts_with('$') {
            Some(parse_value(addr_part, line_num)?)
        } else {
            None
        };
        if index_part == "X" {
            match addr_value {
                Some(addr) if addr <= 0xFF => {
                    binary.push(0x75);
                    binary.push(addr as u8);
                },
                Some(_) => {
                    binary.push(0x7D);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0x7D);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else if index_part == "Y" {
            match addr_value {
                Some(_) => {
                    binary.push(0x79);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0x79);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else {
            return Err(format!("Line {}: Invalid index register: {}", line_num, index_part));
        }
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0x65);
            binary.push(value as u8);
        } else {
            binary.push(0x6D);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0x6D);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_sbc(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        binary.push(0xE9);
        parse_and_push_value(binary, &operand[1..], 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with(",X)") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0xE1);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with("),Y") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0xF1);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.contains(',') {
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        let addr_value = if addr_part.starts_with('$') {
            Some(parse_value(addr_part, line_num)?)
        } else {
            None
        };
        if index_part == "X" {
            match addr_value {
                Some(addr) if addr <= 0xFF => {
                    binary.push(0xF5);
                    binary.push(addr as u8);
                },
                Some(_) => {
                    binary.push(0xFD);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0xFD);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else if index_part == "Y" {
            match addr_value {
                Some(_) => {
                    binary.push(0xF9);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0xF9);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else {
            return Err(format!("Line {}: Invalid index register: {}", line_num, index_part));
        }
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0xE5);
            binary.push(value as u8);
        } else {
            binary.push(0xED);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0xED);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_and(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        binary.push(0x29);
        parse_and_push_value(binary, &operand[1..], 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with(",X)") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0x21);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with("),Y") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0x31);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.contains(',') {
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        let addr_value = if addr_part.starts_with('$') {
            Some(parse_value(addr_part, line_num)?)
        } else {
            None
        };
        if index_part == "X" {
            match addr_value {
                Some(addr) if addr <= 0xFF => {
                    binary.push(0x35);
                    binary.push(addr as u8);
                },
                Some(_) => {
                    binary.push(0x3D);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0x3D);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else if index_part == "Y" {
            match addr_value {
                Some(_) => {
                    binary.push(0x39);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0x39);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else {
            return Err(format!("Line {}: Invalid index register: {}", line_num, index_part));
        }
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0x25);
            binary.push(value as u8);
        } else {
            binary.push(0x2D);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0x2D);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_ora(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        binary.push(0x09);
        parse_and_push_value(binary, &operand[1..], 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with(",X)") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0x01);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with("),Y") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0x11);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.contains(',') {
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        let addr_value = if addr_part.starts_with('$') {
            Some(parse_value(addr_part, line_num)?)
        } else {
            None
        };
        if index_part == "X" {
            match addr_value {
                Some(addr) if addr <= 0xFF => {
                    binary.push(0x15);
                    binary.push(addr as u8);
                },
                Some(_) => {
                    binary.push(0x1D);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0x1D);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else if index_part == "Y" {
            match addr_value {
                Some(_) => {
                    binary.push(0x19);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0x19);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else {
            return Err(format!("Line {}: Invalid index register: {}", line_num, index_part));
        }
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0x05);
            binary.push(value as u8);
        } else {
            binary.push(0x0D);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0x0D);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_eor(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        binary.push(0x49);
        parse_and_push_value(binary, &operand[1..], 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with(",X)") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0x41);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with("),Y") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0x51);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.contains(',') {
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        let addr_value = if addr_part.starts_with('$') {
            Some(parse_value(addr_part, line_num)?)
        } else {
            None
        };
        if index_part == "X" {
            match addr_value {
                Some(addr) if addr <= 0xFF => {
                    binary.push(0x55);
                    binary.push(addr as u8);
                },
                Some(_) => {
                    binary.push(0x5D);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0x5D);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else if index_part == "Y" {
            match addr_value {
                Some(_) => {
                    binary.push(0x59);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0x59);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else {
            return Err(format!("Line {}: Invalid index register: {}", line_num, index_part));
        }
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0x45);
            binary.push(value as u8);
        } else {
            binary.push(0x4D);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0x4D);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_inc(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        return Err(format!("Line {}: INC does not support immediate addressing", line_num));
    } else if operand.contains(',') {
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        if index_part != "X" {
            return Err(format!("Line {}: INC only supports X-indexed addressing", line_num));
        }
        let addr_value = if addr_part.starts_with('$') {
            Some(parse_value(addr_part, line_num)?)
        } else {
            None
        };
        match addr_value {
            Some(addr) if addr <= 0xFF => {
                binary.push(0xF6);
                binary.push(addr as u8);
            },
            Some(_) => {
                binary.push(0xFE);
                parse_and_push_value(binary, addr_part, 2, line_num)?;
            },
            None => {
                binary.push(0xFE);
                if let Some(&address) = labels.get(addr_part) {
                    binary.push((address & 0xFF) as u8);
                    binary.push((address >> 8) as u8);
                } else {
                    unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                    binary.push(0);
                    binary.push(0);
                }
            }
        }
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0xE6);
            binary.push(value as u8);
        } else {
            binary.push(0xEE);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0xEE);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_dec(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        return Err(format!("Line {}: DEC does not support immediate addressing", line_num));
    } else if operand.contains(',') {
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        if index_part != "X" {
            return Err(format!("Line {}: DEC only supports X-indexed addressing", line_num));
        }
        let addr_value = if addr_part.starts_with('$') {
            Some(parse_value(addr_part, line_num)?)
        } else {
            None
        };
        match addr_value {
            Some(addr) if addr <= 0xFF => {
                binary.push(0xD6);
                binary.push(addr as u8);
            },
            Some(_) => {
                binary.push(0xDE);
                parse_and_push_value(binary, addr_part, 2, line_num)?;
            },
            None => {
                binary.push(0xDE);
                if let Some(&address) = labels.get(addr_part) {
                    binary.push((address & 0xFF) as u8);
                    binary.push((address >> 8) as u8);
                } else {
                    unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                    binary.push(0);
                    binary.push(0);
                }
            }
        }
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0xC6);
            binary.push(value as u8);
        } else {
            binary.push(0xCE);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0xCE);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_cmp(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        binary.push(0xC9);
        parse_and_push_value(binary, &operand[1..], 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with(",X)") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0xC1);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.starts_with('(') && operand.ends_with("),Y") {
        let addr_part = &operand[1..operand.len()-3];
        binary.push(0xD1);
        parse_and_push_value(binary, addr_part, 1, line_num)?;
    } else if operand.contains(',') {
        let parts: Vec<&str> = operand.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Line {}: Invalid indexed addressing format: {}", line_num, operand));
        }
        let addr_part = parts[0].trim();
        let index_part = parts[1].trim().to_uppercase();
        let addr_value = if addr_part.starts_with('$') {
            Some(parse_value(addr_part, line_num)?)
        } else {
            None
        };
        if index_part == "X" {
            match addr_value {
                Some(addr) if addr <= 0xFF => {
                    binary.push(0xD5);
                    binary.push(addr as u8);
                },
                Some(_) => {
                    binary.push(0xDD);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0xDD);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else if index_part == "Y" {
            match addr_value {
                Some(_) => {
                    binary.push(0xD9);
                    parse_and_push_value(binary, addr_part, 2, line_num)?;
                },
                None => {
                    binary.push(0xD9);
                    if let Some(&address) = labels.get(addr_part) {
                        binary.push((address & 0xFF) as u8);
                        binary.push((address >> 8) as u8);
                    } else {
                        unresolved_jumps.push((binary.len(), addr_part.to_string(), 2));
                        binary.push(0);
                        binary.push(0);
                    }
                }
            }
        } else {
            return Err(format!("Line {}: Invalid index register: {}", line_num, index_part));
        }
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0xC5);
            binary.push(value as u8);
        } else {
            binary.push(0xCD);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0xCD);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_cpx(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        binary.push(0xE0);
        parse_and_push_value(binary, &operand[1..], 1, line_num)?;
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0xE4);
            binary.push(value as u8);
        } else {
            binary.push(0xEC);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0xEC);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}

fn compile_cpy(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    _current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    if operand.starts_with('#') {
        binary.push(0xC0);
        parse_and_push_value(binary, &operand[1..], 1, line_num)?;
    } else if operand.starts_with('$') {
        let value = parse_value(operand, line_num)?;
        if value <= 0xFF {
            binary.push(0xC4);
            binary.push(value as u8);
        } else {
            binary.push(0xCC);
            binary.push((value & 0xFF) as u8);
            binary.push((value >> 8) as u8);
        }
    } else {
        binary.push(0xCC);
        if let Some(&address) = labels.get(operand) {
            binary.push((address & 0xFF) as u8);
            binary.push((address >> 8) as u8);
        } else {
            unresolved_jumps.push((binary.len(), operand.to_string(), 2));
            binary.push(0);
            binary.push(0);
        }
    }
    Ok(())
}
fn compile_dbg(
    binary: &mut Vec<u8>,
    unresolved_jumps: &mut Vec<(usize, String, usize)>,
    operand: &str,
    current_address: u16,
    labels: &HashMap<String, u16>,
    line_num: usize
) -> Result<(), String> {
    println!("Compiling DBG Instruction");
    let value = parse_value(operand, line_num)?;
    if value <= 0xFF {
        // Zero Page
        binary.push(value as u8);
    } else {
        // Absolute
        binary.push(0xAC);
        binary.push((value & 0xFF) as u8);
        binary.push((value >> 8) as u8);
    }
    Ok(())
}
