use std::{fs::File, io::{self, Read, Seek}, sync::{Arc, atomic::Ordering}};
use mlem_base::{base::mlem_runtime::MlemRuntime, console::ConsoleSender, runtime::{utils::{self}}};
use nih_plug::{buffer::Buffer, prelude::Transport};
use u4::{U4, U4x2};
use crate::{consts, params::DataParams, read_mode::DataReadMode};

const MAX_DATA_SIZE: usize = 1 * 1024 * 1024; // 1 Megabyte

pub struct DataRuntime { 
    params: Arc<DataParams>,
    console: ConsoleSender,

    file_offset: u64,
    file_len: u64,
    data: [u8; MAX_DATA_SIZE],
    data_len: usize,
    data_pos: usize,
    bit_pos: usize,
}

impl DataRuntime {
    pub fn new(params: Arc<DataParams>) -> Self {
        return Self { 
            params,
            console: ConsoleSender::from_singleton(),

            file_offset: 0,
            file_len: 0,
            data: [0; MAX_DATA_SIZE],
            data_len: 0,
            data_pos: 0,
            bit_pos: 0,
        };
    }
}

impl DataRuntime {
    fn next_value(&mut self) -> f32 {
        match self.params.read_mode.value() {
            DataReadMode::Bit1 => {
                let raw = self.next_bit();
                return if raw { 1.0 } else { -1.0 };
            },

            DataReadMode::Bit4 => {
                let raw = self.next_nibble();
                return raw as u8 as f32 / U4::MAX as u8 as f32 * 2.0 - 1.0;
            },

            DataReadMode::Bit8 => {
                let raw = self.next_byte();
                return raw as f32 / u8::MAX as f32 * 2.0 - 1.0;
            },

            DataReadMode::Bit16 => {
                let raw = u16::from_ne_bytes([
                    self.next_byte(),
                    self.next_byte()
                    ]);
                return raw as f32 / u16::MAX as f32 * 2.0;
            },

            DataReadMode::Bit32 => {
                let raw = u32::from_ne_bytes([
                    self.next_byte(),
                    self.next_byte(),
                    self.next_byte(),
                    self.next_byte()
                    ]);
                return raw as f32 / u32::MAX as f32 * 2.0;
            },

            DataReadMode::Bit64 => {
                let raw = u64::from_ne_bytes([
                    self.next_byte(),
                    self.next_byte(),
                    self.next_byte(),
                    self.next_byte(),
                    self.next_byte(),
                    self.next_byte(),
                    self.next_byte(),
                    self.next_byte()
                    ]);
                return raw as f32 / u64::MAX as f32 * 2.0;
            }
        }
    }
    
    fn next_nibble(&mut self) -> U4 {
        let byte = if self.bit_pos >= 1 {
            self.bit_pos = 0;
            self.next_byte()
        } else {
            self.curr_byte()
        };

        let pair = U4x2::from_byte(byte);
        let value = if self.bit_pos > 0 { pair.left() } else {pair.right() };
        self.bit_pos = self.bit_pos + 1;
        return value;
    }

    fn next_bit(&mut self) -> bool {
        let byte = if self.bit_pos >= 8 {
            self.bit_pos = 0;
            self.next_byte()
        } else {
            self.curr_byte()
        };

        let mask = 1 << self.bit_pos;
        self.bit_pos = self.bit_pos + 1;
        return (mask & byte) > 0;
    }

    fn next_byte(&mut self) -> u8 {
        let byte = self.curr_byte();
        self.data_pos = self.data_pos + 1;

        if self.data_pos >= self.data_len {
            let _ = self.update_data_from_file();
            self.data_pos = 0;
        }

        return byte;
    }

    fn curr_byte(&self) -> u8 {
        return self.data[self.data_pos];
    }

    fn update_data_from_file(&mut self) -> std::io::Result<()> {
        let mut path_current = self.params.path_current.load(Ordering::Acquire);
        let mut file = self.get_file(&mut path_current)?;
        
        self.file_offset = self.file_offset + self.data_len as u64;
        if self.file_offset >= self.file_len {
            path_current = path_current + 1;
            self.file_offset = 0;
            file = self.get_file(&mut path_current)?;
        }
        
        file.seek(io::SeekFrom::Start(self.file_offset))?;
        self.data_len = file.read(&mut self.data)?;
        self.data_pos = 0;
        self.params.path_current.store(path_current, Ordering::Release);

        self.console.log(format!("File read {bytes} bytes ({percent}%)", bytes = self.data_len, percent = f32::floor(self.file_offset as f32 / self.file_len as f32 * 100.0)));
        Ok(())
    }

    fn get_file(&mut self, index: &mut usize) -> std::io::Result<File> {
        let paths = self.params.paths.lock().unwrap();
                
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

        self.console.log(format!("Loading file \"{file_path}\" ({index})"));
        return Ok(file);
    }

    fn update_data_from_array(&mut self, array: &[u8]) -> std::io::Result<()> {
        let len = usize::min(array.len(), MAX_DATA_SIZE);

        self.data[0..len].clone_from_slice(&array[0..len]);
        self.data_len = len;
        self.data_pos = 0;

        Ok(())
    }
}

impl MlemRuntime<DataParams> for DataRuntime {
    fn params(&self) ->  Arc<DataParams> {
        return self.params.clone();
    }

    fn init(&mut self) {
        let _ = self.update_data_from_array(consts::DEFAULT_DATA);
    }
    
    fn reset(&mut self) { }
    
    fn run(&mut self, buffer: &mut Buffer, _transport: &Transport) {
        let refresh_path = self.params.path_refresh.load(Ordering::Relaxed);
        if refresh_path {
            let _ = self.update_data_from_file();
            self.params.path_refresh.store(false, Ordering::Relaxed);
        }

        for channel_samples in buffer.iter_samples() {          
            let mut value = if self.params.mono.value() { self.next_value() } else { 0.0 };

            for sample in channel_samples {
                if !self.params.mono.value() {
                    value = self.next_value();
                }

                if self.params.mute.value() {
                    *sample = 0.0;
                    continue;
                }

                *sample = utils::clip(value);
            }
        }

        let mut data_preview = self.params.data_preview.lock().unwrap();
        let len = usize::min(self.data_len - self.data_pos, data_preview.len());
        data_preview[0..len].clone_from_slice(&self.data[self.data_pos..(len + self.data_pos)]);
    }
}