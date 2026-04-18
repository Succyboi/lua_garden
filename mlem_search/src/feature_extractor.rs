use rodio::{source::Source, Decoder};
use rubato::Resampler;
use rubato::{SincFixedIn, SincInterpolationParameters, SincInterpolationType};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc;
use threadpool::ThreadPool;
use walkdir::WalkDir;
use mfcc::mfcc::Transform;

use crate::{FEATURE_DIMENSIONS, FEATURE_THREADS, FEATURE_SAMPLE_RATE};
use crate::feature::Feature;

fn get_audio_files(root_dir: &str) -> Vec<String> {
    let path = PathBuf::from(root_dir);

    let supported_extensions = ["wav", "mp3"];
    WalkDir::new(path)
        .into_iter()
        .filter_map(|d| d.ok())
        .map(|d| d.path().to_owned())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| supported_extensions.contains(&ext.to_lowercase().as_str()))
                .unwrap_or(false)
        })
        .map(|path| path.into_os_string().into_string().unwrap())
        .collect()
}

pub enum RunMode {
    SingleThreaded,
    Parallel,
}

pub fn extract_features(run_mode: RunMode, asset_dir: &str, progress_callback: impl Fn(f32)) -> Result<Vec<Feature>, String> {
    let files = get_audio_files(asset_dir);
    let num_files = files.len();
    if num_files == 0 {
        return Err(format!("No files found in {asset_dir}"));
    }

    let mut features: Vec<Feature> = Vec::with_capacity(files.len());

    match run_mode {
        RunMode::SingleThreaded => {
            for file in files.iter() {
                if let Ok(mfcc) = decode_and_calculate_mfcc(file, FEATURE_SAMPLE_RATE) {
                    features.push(Feature::new(mfcc, file.to_string(), None));
                }
            }
        }
        RunMode::Parallel => {
            println!("Running with {FEATURE_THREADS} threads");
            let thread_pool = ThreadPool::new(FEATURE_THREADS);

            let (sender, receiver) = mpsc::channel::<Feature>();

            for file in files.iter() {
                let f = file.to_string();
                let sender = sender.clone();
                thread_pool.execute(move || {
                    if let Ok(mfcc) = decode_and_calculate_mfcc(&f, FEATURE_SAMPLE_RATE) {
                        sender.send(Feature::new(mfcc, f, None)).unwrap();
                    } else {
                        println!("Failed to extract features for {f}");
                    }
                });
            }

            let mut progress = 0.0;
            let progress_increment = 1.0 / files.len() as f32;
            while thread_pool.active_count() > 0 || thread_pool.queued_count() > 0 {
                if let Ok(feature) = receiver.try_recv() {
                    features.push(feature);
                    progress += progress_increment;
                    progress_callback(progress);
                }
            }
        }
    }
    Ok(features)
}

fn decode_and_calculate_mfcc(path: &str, output_sample_rate: usize) -> Result<Vec<f32>, String> {
    let mut decoded = decode_and_resample_file(path, output_sample_rate).unwrap();
    let mfcc = calculate_mfcc(&mut decoded, 22050);
    match mfcc {
        Ok(mfcc) => {
            return Ok(mfcc);
        }
        Err(e) => {
            println!("{}", e);
            return Err(e);
        }
    }
}

fn decode_and_resample_file(path: &str, output_sample_rate: usize) -> Result<Vec<f32>, String> {
    let file = BufReader::new(File::open(path).map_err(|e| e.to_string())?);
    let decoder = Decoder::new(file).map_err(|e| e.to_string())?;
    let num_channels = decoder.channels();
    let sample_rate = decoder.sample_rate();

    let mut samples: Vec<f32>;
    if num_channels == 1 {
        samples = decoder.convert_samples::<f32>().collect();
    } else if num_channels == 2 {
        // Sum to mono
        samples = decoder
            .convert_samples::<f32>()
            .array_chunks::<2>()
            .map(|frame: [f32; 2]| (frame[0] + frame[1]) * 0.5)
            .collect();
    } else {
        return Err("Unsupported channel count".to_string());
    }

    if sample_rate as usize != output_sample_rate {
        samples = resample_buffer(&samples, sample_rate as f64, output_sample_rate as f64);
    }

    Ok(samples)
}

fn resample_buffer(buffer: &Vec<f32>, source_sr: f64, dest_sr: f64) -> Vec<f32> {
    let max_resample_ratio_relative: f64 = 10.0;
    let chunk_size = 2048;
    let num_channels = 1;

    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 160,
        window: rubato::WindowFunction::BlackmanHarris2,
    };

    // Create the resampler
    let mut resampler = SincFixedIn::<f32>::new(
        dest_sr / source_sr,
        max_resample_ratio_relative,
        params,
        chunk_size,
        num_channels,
    )
    .unwrap();

    let mut input: Vec<&[f32]> = vec![buffer];
    let mut input_offset = 0;
    let mut resampled_buffer: Vec<f32> = Vec::with_capacity(buffer.len());
    let mut output_buffer: Vec<Vec<f32>> = vec![vec![0.0; 2048]];

    while let Ok((input_frames, output_frames)) =
        resampler.process_into_buffer(&input, &mut output_buffer, None)
    {
        let output = output_buffer.first().unwrap();
        resampled_buffer.extend_from_slice(&output[0..output_frames]);
        input_offset += input_frames;
        let next_input = &buffer[input_offset..];
        input[0] = next_input;
    }
    resampled_buffer
}

fn calculate_mfcc(buffer: &mut Vec<f32>, sample_rate: usize) -> Result<Vec<f32>, String> {
    let chunk_size = 2048;
    let num_coefficients = FEATURE_DIMENSIONS;

    // Pad with zeros if the buffer isn't large enough to hold a full fft block
    let num_blocks = (buffer.len() as f32 / chunk_size as f32).floor() as usize;
    if num_blocks == 0 {
        buffer.resize(chunk_size, 0.0);
    }

    let mut state = Transform::new(sample_rate, chunk_size);
    let mut mean_mfcc: Vec<f32> = vec![0.0; num_coefficients];

    for block_index in 0..num_blocks {
        let start = block_index * chunk_size;
        let buf = &buffer[start..];

        let mut i_buf = vec![0; buf.len()];
        for i in 0..buf.len() {
            i_buf[i] = f32::floor(buf[i] * i16::MAX as f32) as i16;
        }

        let mut output = vec![0.0; num_coefficients];
        state.transform(&i_buf, &mut output);

        for i in 0..output.len() {
            mean_mfcc[i] += (output[i] / 2.0) as f32;
        }
    }
    
    // Calculate mean by dividing by the number of blocks
    for e in &mut mean_mfcc {
        *e /= num_blocks as f32;
    }

    return Ok(mean_mfcc)
}