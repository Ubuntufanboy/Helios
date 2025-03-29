// src/main.rs
mod cpu;
mod isa;
mod display;
mod audio;
mod compiler;
mod memory;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Helios")
        .version("0.1.0")
        .about("An 8-bit console emulator")
        .arg(Arg::with_name("rom")
                .short("r")
                .long("rom")
                .value_name("FILE")
                .help("ROM file to load")
                .takes_value(true))
        .arg(Arg::with_name("assembly")
                .short("a")
                .long("asm")
                .value_name("FILE")
                .help("Assembly file to compile and run")
                .takes_value(true))
        .get_matches();

    // Initialize shared memory
    let memory = Arc::new(Mutex::new(memory::Memory::new()));
    
    // Initialize CPU
    let cpu = Arc::new(Mutex::new(cpu::CPU::new(Arc::clone(&memory))));
    
    // Load ROM or compile assembly
    if let Some(rom_path) = matches.value_of("rom") {
        let mut file = File::open(rom_path).expect("Failed to open ROM file");
        let mut rom_data = Vec::new();
        file.read_to_end(&mut rom_data).expect("Failed to read ROM file");
        
        memory.lock().unwrap().load_program(&rom_data);
    } else if let Some(asm_path) = matches.value_of("assembly") {
        let mut file = File::open(asm_path).expect("Failed to open assembly file");
        let mut asm_content = String::new();
        file.read_to_string(&mut asm_content).expect("Failed to read assembly file");
        
        match compiler::compile(&asm_content) {
            Ok(binary) => memory.lock().unwrap().load_program(&binary),
            Err(err) => {
                eprintln!("Compilation failed: {}", err);
                return;
            }
        }
    } else {
        println!("No ROM or assembly file specified. Use --rom or --asm options.");
        return;
    }
    
    // Start display thread
    /*
    let display_memory = Arc::clone(&memory);
    let display_handle = thread::spawn(move || {
        let mut display = display::Display::new(Arc::clone(&display_memory));
        
        let frame_duration = Duration::from_millis(33); // ~30 FPS
        let mut last_frame = Instant::now();
        
        loop {
            let now = Instant::now();
            let elapsed = now.duration_since(last_frame);
            
            if elapsed >= frame_duration {
                display.update();
                last_frame = now;
            }
            
            if display.should_exit() {
                break;
            }
            
            thread::sleep(Duration::from_millis(1));
        }
    });
    // Start audio thread
    let audio_memory = Arc::clone(&memory);
    let audio_handle = thread::spawn(move || {
        let mut audio = audio::Audio::new(Arc::clone(&audio_memory));
        
        loop {
            audio.update();
            thread::sleep(Duration::from_millis(1));
        }
    });
    */
    // Run CPU at 1 MHz (each instruction takes varying cycles)
    let cpu_memory = Arc::clone(&memory);
    let cpu_handle = thread::spawn(move || {
        let cycle_time = Duration::from_nanos(1_000); // 1 MHz = 1000ns per cycle
        
        loop {
            let start = Instant::now();
            
            {
                let mut cpu = cpu.lock().unwrap();
                if !cpu.step() {
                    break; // Stop if CPU is halted
                }
            }
            
            let elapsed = start.elapsed();
            if elapsed < cycle_time {
                thread::sleep(cycle_time - elapsed);
            }
        }
    });
    
    // Wait for threads to finish
    cpu_handle.join().unwrap();
    // display_handle.join().unwrap();
    // audio_handle.join().unwrap();
}
