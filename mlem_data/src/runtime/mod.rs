use core::fmt;
use std::fmt::format;
use std::fs::File;
use std::io::{self, Read, Seek};
use std::{ fmt::Error, sync::atomic::Ordering };
use mlem_base::console::ConsoleSender;
use nih_plug_egui::egui::load;
use u4::{U4, U4x2};
use crate::consts;
use crate::read_mode::DataReadMode;
use crate::{ MeterParams };
use nih_plug::{ prelude::* };
use mlem_base::runtime::utils::{ self, RMS, Timer };

const MAX_DATA_SIZE: usize = 1 * 1024 * 1024; // 1 Megabyte

// TODO Figure out how to synthesize from bits
pub struct Runtime {
    pub console: Option<ConsoleSender>,

    sample_rate: f32,
    buffer_size: usize,
    channels: usize,
    last_playing: bool,

    file_offset: u64,
    file_len: u64,
    data: [u8; MAX_DATA_SIZE],
    data_len: usize,
    data_pos: usize,
    bit_pos: usize,

    run_time: RMS,
}

impl Runtime {
    pub fn new(console: Option<ConsoleSender>) -> Runtime {
        let runtime = Self {
            console: console,

            sample_rate: 0.0,
            buffer_size: 0,
            channels: 0,
            last_playing: false,
            
            file_offset: 0,
            file_len: 0,
            data: [0; MAX_DATA_SIZE],
            data_len: 0,
            data_pos: 0,
            bit_pos: 0,

            run_time: RMS::new(1.0),
        };

        return runtime;
    }
    
    pub fn init(&mut self, sample_rate: f32) {        
        self.sample_rate = sample_rate;

        let execute_timer = Timer::new();
        let execute_time = execute_timer.elapsed_ms();

        let _ = self.update_data_from_array(consts::DEFAULT_DATA);
        
        self.log(format!("Init in {:.2}ms.", execute_time));
    }

    pub fn reset(&mut self) {
        let execute_timer = Timer::new();

        self.log(format!("Reset in {:.2}ms.", execute_timer.elapsed_ms()));
    }

    pub fn run(&mut self, buffer: &mut Buffer, params: &MeterParams, transport: &Transport) {
        self.buffer_size = buffer.samples();
        self.channels = buffer.channels();
        let execute_timer = Timer::new();

        self.last_playing = transport.playing;

        let refresh_path = params.path_refresh.load(Ordering::Relaxed);
        if refresh_path {
            let _ = self.update_data_from_file(params);
            params.path_refresh.store(false, Ordering::Relaxed);
        }

        let mut data_preview = params.data_preview.lock().unwrap();
        let len = usize::min(self.data_len - self.data_pos, data_preview.len());
        data_preview[0..len].clone_from_slice(&self.data[self.data_pos..(len + self.data_pos)]);

        for channel_samples in buffer.iter_samples() {          
            let mut value = if params.mono.value() { self.next_value(params) } else { 0.0 };

            for sample in channel_samples {
                if !params.mono.value() {
                    value = self.next_value(params);
                }

                if params.mute.value() {
                    *sample = 0.0;
                    continue;
                }

                *sample = utils::clip(value);
            }
        }

        self.run_time.process( execute_timer.elapsed_ms(), self.sample_rate);
        self.update_params(params);
    }

    pub fn update_params(&mut self, params: &MeterParams) {
        params.sample_rate.store(self.sample_rate, Ordering::Relaxed);
        params.buffer_size.store(self.buffer_size, Ordering::Relaxed);
        params.channels.store(self.channels, Ordering::Relaxed);
        params.run_ms.store(self.run_time.get(), Ordering::Relaxed);

        if self.file_len > 0 {
            params.data_progress.store((self.file_offset + self.data_pos as u64) as f32 / self.file_len as f32, Ordering::Relaxed);
        }
    }

    fn next_value(&mut self, params: &MeterParams) -> f32 {
        match params.read_mode.value() {
            DataReadMode::Bit1 => {
                let raw = self.next_bit(params);
                return if raw { 1.0 } else { 0.0 };
            },

            DataReadMode::Bit4 => {
                let raw = self.next_nibble(params);
                return raw as u8 as f32 / U4::MAX as u8 as f32 * 2.0 - 1.0;
            },

            DataReadMode::Bit8 => {
                let raw = self.next_byte(params);
                return raw as f32 / u8::MAX as f32 * 2.0 - 1.0;
            },

            DataReadMode::Bit16 => {
                let raw = u16::from_ne_bytes([
                    self.next_byte(params),
                    self.next_byte(params)
                    ]);
                return raw as f32 / u16::MAX as f32 * 2.0;
            },

            DataReadMode::Bit32 => {
                let raw = u32::from_ne_bytes([
                    self.next_byte(params),
                    self.next_byte(params),
                    self.next_byte(params),
                    self.next_byte(params)
                    ]);
                return raw as f32 / u32::MAX as f32 * 2.0;
            },

            DataReadMode::Bit64 => {
                let raw = u64::from_ne_bytes([
                    self.next_byte(params),
                    self.next_byte(params),
                    self.next_byte(params),
                    self.next_byte(params),
                    self.next_byte(params),
                    self.next_byte(params),
                    self.next_byte(params),
                    self.next_byte(params)
                    ]);
                return raw as f32 / u64::MAX as f32 * 2.0;
            }
        }
    }
    
    fn next_nibble(&mut self, params: &MeterParams) -> U4 {
        let byte = if self.bit_pos >= 1 {
            self.bit_pos = 0;
            self.next_byte(params)
        } else {
            self.curr_byte()
        };

        let pair = U4x2::from_byte(byte);
        let value = if self.bit_pos > 0 { pair.left() } else {pair.right() };
        self.bit_pos = self.bit_pos + 1;
        return value;
    }

    fn next_bit(&mut self, params: &MeterParams) -> bool {
        let byte = if self.bit_pos >= 8 {
            self.bit_pos = 0;
            self.next_byte(params)
        } else {
            self.curr_byte()
        };

        let mask = 1 << self.bit_pos;
        self.bit_pos = self.bit_pos + 1;
        return (mask & byte) > 0;
    }

    fn next_byte(&mut self, params: &MeterParams) -> u8 {
        let byte = self.curr_byte();
        self.data_pos = self.data_pos + 1;

        if self.data_pos >= self.data_len {
            let _ = self.update_data_from_file(params);
            self.data_pos = 0;
        }

        return byte;
    }

    fn curr_byte(&self) -> u8 {
        return self.data[self.data_pos];
    }

    fn update_data_from_file(&mut self, params: &MeterParams) -> std::io::Result<()> {
        let mut path_current = params.path_current.load(Ordering::Acquire);
        let mut file = self.get_file(params, &mut path_current)?;
        
        self.file_offset = self.file_offset + self.data_len as u64;
        if self.file_offset >= self.file_len {
            path_current = path_current + 1;
            self.file_offset = 0;
            file = self.get_file(params, &mut path_current)?;
        }
        
        file.seek(io::SeekFrom::Start(self.file_offset))?;
        self.data_len = file.read(&mut self.data)?;
        self.data_pos = 0;
        params.path_current.store(path_current, Ordering::Release);

        self.log(format!("File read {bytes} bytes ({percent}%)", bytes = self.data_len, percent = f32::floor(self.file_offset as f32 / self.file_len as f32 * 100.0)));
        Ok(())
    }

    fn get_file(&mut self, params: &MeterParams, index: &mut usize) -> std::io::Result<File> {
        let paths = params.paths.lock().unwrap();
                
        if paths.len() <= 0 {
            return Err(std::io::Error::new(io::ErrorKind::Other, "No paths available."));
        }
        
        *index = *index % paths.len();
        let file_path = &paths[*index];
        let file = File::open(file_path)?;
        self.file_len = file.metadata()?.len();

        if self.file_len == 0 {
            return Err(std::io::Error::new(io::ErrorKind::Other, "File length is 0."));
        }

        self.log(format!("Loading file \"{file_path}\" ({index})"));
        return Ok(file);
    }

    fn update_data_from_array(&mut self, array: &[u8]) -> std::io::Result<()> {
        let len = usize::min(array.len(), MAX_DATA_SIZE);

        self.data[0..len].clone_from_slice(&array[0..len]);
        self.data_len = len;
        self.data_pos = 0;

        Ok(())
    }

    fn log(&self, message: String) {
        match &self.console {
            Some(c) => {
                c.log(message);
            },
            None => {
                println!("No console exists for Runtime. Log not registered by receiver: {}", message)
            }
        }
    }
}