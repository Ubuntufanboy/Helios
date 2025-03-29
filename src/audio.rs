use std::sync::{Arc, Mutex};
use rodio::{OutputStream, Source, Sink};
use std::time::Duration;
use rand::prelude::*;

use crate::memory::{Memory, AUDIO_START, AUDIO_SIZE};

// Constants
const SAMPLE_RATE: u32 = 44100;
const NUM_CHANNELS: usize = 4;
const BUFFER_DURATION: Duration = Duration::from_millis(100);

// Channel types
#[derive(Clone, Copy)]
enum ChannelType {
    Sine,
    Square,
    Triangle,
    Noise,
}

#[derive(Clone)]
struct Channel {
    channel_type: ChannelType,
    frequency: f32,
    phase: f32,
    enabled: bool,
    volume: f32,
}

impl Channel {
    fn new(channel_type: ChannelType) -> Self {
        Self {
            channel_type,
            frequency: 440.0, // Default A4
            phase: 0.0,
            enabled: false,
            volume: 0.2,
        }
    }
    
    fn set_midi_note(&mut self, note: u8) {
        // Convert MIDI note to frequency
        // For our 8-bit console, we add 21 as specified to get the real MIDI note
        let real_note = note as f32 + 21.0;
        self.frequency = 440.0 * 2.0f32.powf((real_note - 69.0) / 12.0);
        self.enabled = true;
    }
}

struct MixedChannelSource {
    channels: Vec<Channel>,
    current_sample: usize,
    total_samples: usize,
}

impl MixedChannelSource {
    fn new(channels: Vec<Channel>) -> Self {
        let total_samples = (SAMPLE_RATE as usize * BUFFER_DURATION.as_secs_f32() as usize).max(1024);
        Self {
            channels,
            current_sample: 0,
            total_samples,
        }
    }
}

impl Iterator for MixedChannelSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample >= self.total_samples {
            return None;
        }

        let mut mixed_sample = 0.0;
        let mut rng = rand::thread_rng();

        for channel in &mut self.channels {
            if !channel.enabled {
                continue;
            }

            // Calculate phase increment
            let phase_inc = channel.frequency / SAMPLE_RATE as f32;
            let current_phase = (channel.phase + phase_inc * self.current_sample as f32) % 1.0;
            
            // Generate sample based on channel type
            let sample = match channel.channel_type {
                ChannelType::Sine => {
                    (current_phase * 2.0 * std::f32::consts::PI).sin() * channel.volume
                },
                ChannelType::Square => {
                    if (current_phase * 2.0 * std::f32::consts::PI).sin() >= 0.0 { 
                        channel.volume 
                    } else { 
                        -channel.volume 
                    }
                },
                ChannelType::Triangle => {
                    2.0 * (current_phase - (current_phase + 0.5).floor()).abs() * channel.volume - channel.volume / 2.0
                },
                ChannelType::Noise => {
                    rng.gen::<f32>() * 2.0 * channel.volume - channel.volume
                }
            };

            mixed_sample += sample;
        }

        self.current_sample += 1;
        
        // Clamp the mixed sample to prevent clipping
        Some(mixed_sample.max(-1.0).min(1.0))
    }
}

impl Source for MixedChannelSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.total_samples - self.current_sample)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(BUFFER_DURATION)
    }
}

pub struct Audio {
    memory: Arc<Mutex<Memory>>,
    _stream: OutputStream,
    sink: Sink,
    channels: Vec<Channel>,
    last_buffer: Vec<u8>,
}

impl Audio {
    pub fn new(memory: Arc<Mutex<Memory>>) -> Self {
        // Create output stream
        let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create audio output stream");

        // Create sink
        let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");

        // Create channels
        let channels = vec![
            Channel::new(ChannelType::Sine),
            Channel::new(ChannelType::Square),
            Channel::new(ChannelType::Triangle),
            Channel::new(ChannelType::Noise),
        ];

        // Create and start continuous audio
        let mixed_source = MixedChannelSource::new(channels.clone());
        sink.append(mixed_source);
        sink.set_volume(0.5);

        Self {
            memory,
            _stream,
            sink,
            channels,
            last_buffer: vec![0; AUDIO_SIZE],
        }
    }
    
    pub fn update(&mut self) {
        let memory = self.memory.lock().unwrap();
        let audio_buffer = memory.get_audio_buffer();
        
        let mut channels_updated = false;
        for i in 0..AUDIO_SIZE {
            if self.last_buffer[i] != audio_buffer[i] {
                self.last_buffer[i] = audio_buffer[i];
                
                // Format: CCNNNNNN
                // CC = Channel
                // NNNNNN = MIDI
                let data = audio_buffer[i];
                let channel = (data >> 6) & 0x03;
                let note = data & 0x3F;

                if channel < NUM_CHANNELS as u8 {
                    self.channels[channel as usize].set_midi_note(note);
                    channels_updated = true;
                }
            }
        }

        // Restart audio if channels have been updated
        if channels_updated {
            // Clear previous sounds
            self.sink.clear();
            
            // Create new mixed source with updated channels
            let mixed_source = MixedChannelSource::new(self.channels.clone());
            self.sink.append(mixed_source);
        }
    }
}
