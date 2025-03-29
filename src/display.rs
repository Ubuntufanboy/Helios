// src/display.rs
use std::sync::{Arc, Mutex};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use crate::memory::{Memory, DISPLAY_START, DISPLAY_SIZE};

// Constants
const DISPLAY_WIDTH: usize = 256;
const DISPLAY_HEIGHT: usize = 256;
const PIXEL_SCALE: usize = 2; // Scale up the pixels for better visibility

// Color palette (8 colors)
const COLORS: [Color; 8] = [
    Color::RGB(0, 0, 0),       // Black (0)
    Color::RGB(255, 0, 0),     // Red (1)
    Color::RGB(255, 255, 0),   // Yellow (2)
    Color::RGB(0, 255, 0),     // Green (3)
    Color::RGB(0, 0, 255),     // Blue (4)
    Color::RGB(0, 255, 255),   // Cyan (5)
    Color::RGB(192, 192, 192), // Grey (6)
    Color::RGB(255, 255, 255), // White (7)
];

pub struct Display {
    canvas: Canvas<Window>,
    memory: Arc<Mutex<Memory>>,
    event_pump: sdl2::EventPump,
    exit_requested: bool,
}

impl Display {
    pub fn new(memory: Arc<Mutex<Memory>>) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        
        let window = video_subsystem.window(
            "Helios 8-bit Console",
            (DISPLAY_WIDTH * PIXEL_SCALE) as u32,
            (DISPLAY_HEIGHT * PIXEL_SCALE) as u32,
        )
        .position_centered()
        .build()
        .unwrap();
        
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        
        Self {
            canvas,
            memory,
            event_pump,
            exit_requested: false,
        }
    }
    
    pub fn update(&mut self) {
        // Handle SDL events
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.exit_requested = true;
                },
                _ => {}
            }
        }
        
        // Clear the screen
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        
        // Draw the display buffer
        let memory = self.memory.lock().unwrap();
        let display_buffer = memory.get_display_buffer();
        
        for y in 0..DISPLAY_HEIGHT {
            for x in 0..DISPLAY_WIDTH {
                let pixel_index = y * DISPLAY_WIDTH + x;
                if pixel_index < DISPLAY_SIZE {
                    let pixel_value = display_buffer[pixel_index] & 0x07; // Get color index (0-7)
                    let color = COLORS[pixel_value as usize];
                    
                    self.canvas.set_draw_color(color);
                    self.canvas.fill_rect(Rect::new(
                        (x * PIXEL_SCALE) as i32,
                        (y * PIXEL_SCALE) as i32,
                        PIXEL_SCALE as u32,
                        PIXEL_SCALE as u32,
                    )).unwrap();
                }
            }
        }
        
        // Present the frame
        self.canvas.present();
    }
    
    pub fn should_exit(&self) -> bool {
        self.exit_requested
    }
}
