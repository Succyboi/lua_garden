pub mod consts;
pub mod runtime;

use atomic_float::{ AtomicF32, AtomicF64 };
use mlem_base::{interface::{param_drag_value, param_toggle, utils::{fill_seperator_available, parameter_grid, parameter_label}}, metadata::PluginMetadata, parameters::PluginParameters};
use runtime::{ Runtime };
use mlem_base::{ interface::{ Interface }, PluginImplementation };
use nih_plug::prelude::*;
use std::{collections::VecDeque, ops::Deref, sync::{ Arc, atomic::{AtomicBool, AtomicUsize, Ordering} }, time::{SystemTime, UNIX_EPOCH}};
use nih_plug_egui::{EguiState, egui::{Align, Context, Layout, Ui}};
use consts::PLUGIN_METADATA;

const STRETCH_STRING_SOURCE: &str = "Stretch! ";
const MAX_STRETCH_STRING_LENGTH: usize = 24;
const MIN_BUFFER_SECONDS: f32 = 0.1;
const MAX_BUFFER_SECONDS: f32 = 60.0;

pub struct Stretch {
    runtime: Runtime,
    params: Arc<StretchParams>,
    implementation: Arc<StretchImplementation>,
}

#[derive(Params)]
pub struct StretchParams {
    #[persist = "editor-state"] editor_state: Arc<EguiState>,
    #[id = "stretch"]           stretch: BoolParam,
    #[id = "speed"]             speed: FloatParam,
    #[id = "variance"]          variance: FloatParam,
    #[id = "window"]            window: FloatParam,
    #[id = "pitch"]             pitch: FloatParam,
    #[id = "buffer"]            buffer: FloatParam,
    
    sample_rate: AtomicF32,
    buffer_size: AtomicUsize,
    channels: AtomicUsize,
    run_ms: AtomicF32,
}

pub struct StretchImplementation { 
    params: Arc<StretchParams>,

    stretch_time: AtomicF32
}

impl Default for Stretch {
    fn default() -> Self {
        let runtime = Runtime::new(None);
        let params = Arc::new(StretchParams::default());

        let stretch = Self {
            runtime: runtime,
            params: params.clone(),
            implementation: Arc::new(StretchImplementation::new(params.clone())),
        };

        return stretch;
    }
}

impl Default for StretchParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(PLUGIN_METADATA.window_width, PLUGIN_METADATA.window_height),
            stretch: BoolParam::new("Stretch", false),
            speed: FloatParam::new("Speed", 0.5, FloatRange::Linear { min: 0.0, max: 1.0 }),
            variance: FloatParam::new("Variance", 0.01, FloatRange::Linear { min: 0.0, max: 1.0 }),
            window: FloatParam::new("Window Size", 0.05, FloatRange::Linear { min: 0.01, max: 0.2 }),
            pitch: FloatParam::new("Pitch", 0.0, FloatRange::Linear { min: -12.0, max: 12.0 }).with_unit("st"),
            buffer: FloatParam::new("Buffer Size", 60.0, FloatRange::Linear { min: MIN_BUFFER_SECONDS, max: MAX_BUFFER_SECONDS }).with_unit("s"),

            sample_rate: AtomicF32::new(0.0),
            buffer_size: AtomicUsize::new(0),
            channels: AtomicUsize::new(0),
            run_ms: AtomicF32::new(0.0),
        }
    }
}

impl PluginParameters for StretchParams {
    fn sample_rate(&self) -> &AtomicF32 { &self.sample_rate }
    fn buffer_size(&self) -> &AtomicUsize { &self.buffer_size }
    fn channels(&self) -> &AtomicUsize { &self.channels }
    fn run_ms(&self) -> &AtomicF32 { &self.run_ms }
}

impl Stretch { }

impl StretchImplementation {
    fn get_stretch_string(&self) -> String {
        if self.params.stretch.value() {
            self.stretch_time.store(self.stretch_time.load(Ordering::Relaxed) + self.params.speed.value() * 0.2, Ordering::Relaxed);
        }
        
        let mut string = String::new();
        for c in 0..MAX_STRETCH_STRING_LENGTH {
            let char_i = (f32::floor(c as f32 * self.params.speed.value() + self.stretch_time.load(Ordering::Relaxed)) as usize) % STRETCH_STRING_SOURCE.len();
            let char = STRETCH_STRING_SOURCE.chars().nth(char_i).expect("Out of bounds");

            string.push(char);
        }

        return string;
    }
}

impl PluginImplementation<StretchParams> for StretchImplementation {
    fn new(params: Arc<StretchParams>) -> StretchImplementation {
        return Self {
            params: params.clone(),

            stretch_time: AtomicF32::from(0.0),
        }
    }

    fn metadata(&self) -> PluginMetadata {
        return PLUGIN_METADATA;
    }

    fn params(&self) -> Arc<StretchParams> {
        return self.params.clone();
    }

    fn interface_build(&self, _ctx: &Context) { }

    fn interface_update_center(&self, ui: &mut Ui, _ctx: &Context, setter: &ParamSetter) {
        ui.horizontal(|ui| {
            ui.add(param_toggle::ParamToggle::for_param(&self.params.stretch, setter, "Stretch", "Stretch"));
            ui.add(param_drag_value::ParamDragValue::for_param(&self.params.speed, setter));
        });
        
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.add(param_drag_value::ParamDragValue::for_param(&self.params.variance, setter));
            ui.add(param_drag_value::ParamDragValue::for_param(&self.params.window, setter));
        });

        ui.horizontal(|ui| {
            ui.add(param_drag_value::ParamDragValue::for_param(&self.params.pitch, setter).with_decimals(0));
            ui.add_enabled_ui(!*&self.params.stretch.value(), |ui| {
                ui.add(param_drag_value::ParamDragValue::for_param(&self.params.buffer, setter).with_decimals(0));
            });
        });

        ui.separator();
        ui.horizontal(|ui| {
            ui.add_enabled_ui(false, |ui| {
                let stretch_string = &self.get_stretch_string();
                ui.monospace(stretch_string);
            });
        });
    }

    fn interface_update_bar(&self, ui: &mut Ui, _ctx: &Context, _setter: &ParamSetter) {
        
    }
}

impl Plugin for Stretch {
    const NAME: &'static str = PLUGIN_METADATA.name;
    const VENDOR: &'static str = PLUGIN_METADATA.vendor;
    const URL: &'static str = PLUGIN_METADATA.homepage_url;
    const EMAIL: &'static str = PLUGIN_METADATA.email;
    const VERSION: &'static str = PLUGIN_METADATA.version;

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = false;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let interface = Interface::new(consts::PLUGIN_METADATA, self.implementation.clone());
        
        let editor_state = self.params.editor_state.clone();
        self.runtime.console = Some(interface.console.create_sender());
        let editor = interface.create_interface(editor_state);

        return editor;
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        let _ = self.runtime.init(buffer_config.sample_rate);

        return true;
    }

    fn reset(&mut self) {
        self.runtime.reset();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let params = self.params.clone();

        self.runtime.run(buffer, &params, context.transport());

        return ProcessStatus::Normal;
    }
}

impl ClapPlugin for Stretch {
    const CLAP_ID: &'static str = PLUGIN_METADATA.identifier;
    const CLAP_DESCRIPTION: Option<&'static str> = Some(PLUGIN_METADATA.description);
    const CLAP_MANUAL_URL: Option<&'static str> = Some(PLUGIN_METADATA.homepage_url);
    const CLAP_SUPPORT_URL: Option<&'static str> = Some(PLUGIN_METADATA.support_url);

    const CLAP_FEATURES: &'static [ClapFeature] = PLUGIN_METADATA.clap_features;
}

impl Vst3Plugin for Stretch {
    const VST3_CLASS_ID: [u8; 16] = PLUGIN_METADATA.class_identifier;

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = PLUGIN_METADATA.vst3_subcategories;
}

nih_export_clap!(Stretch);
nih_export_vst3!(Stretch);
