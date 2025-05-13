pub mod consts;
pub mod runtime;
pub mod interface;
pub mod console;

use console::ConsoleReceiver;
use runtime::{ Runtime, runtime_data::RuntimeData, runtime_data::RuntimeState };
use interface::{ interface_data::InterfaceData, Interface };
use nih_plug::prelude::*;
use std::sync::{ Arc, RwLock };
use nih_plug_egui::EguiState;

pub struct LuaGarden {
    runtime: Runtime,
    params: Arc<LuaGardenParams>,
    runtime_data: Arc<RwLock<RuntimeData>>,
    interface_data: Arc<RwLock<InterfaceData>>
}

#[derive(Params)]
pub struct LuaGardenParams {
    #[persist = "editor-state"]
    editor_state: Arc<EguiState>,
}

impl Default for LuaGarden {
    fn default() -> Self {
        let runtime = Runtime::new(None);

        Self {
            runtime: runtime,
            params: Arc::new(LuaGardenParams::default()),
            runtime_data: Arc::from(RwLock::new(RuntimeData::new())),
            interface_data: Arc::from(RwLock::new(InterfaceData::new()))
        }
    }
}

impl Default for LuaGardenParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(consts::WINDOW_SIZE_WIDTH, consts::WINDOW_SIZE_HEIGHT)
        }
    }
}

impl LuaGarden {
    fn update_runtime_status(&self, runtime_data: &mut RuntimeData) {
        runtime_data.sample_rate = self.runtime.get_sample_rate();
        runtime_data.buffer_size = self.runtime.get_buffer_size();
        runtime_data.channels = self.runtime.get_channels();
        runtime_data.run_ms = self.runtime.get_run_ms();
    }

    fn refresh_runtime_module(&mut self, interface_data: &InterfaceData) {
        match interface_data.mode.clone() {
            interface::InterfaceMode::Draft => {
                let content = interface_data.draft_content.clone();
                self.runtime.load_new_module(content);
            },
            interface::InterfaceMode::Workspace => {
                match &interface_data.workspace {
                    Some(w) => {
                        let content = w.content.clone();
                        self.runtime.load_new_module(content);
                    },
                    None => ()
                }
            }
        }
    }

    fn clear_runtime_module(&mut self){
        self.runtime.load_module(None);
    }
}

impl Plugin for LuaGarden {
    const NAME: &'static str = "lua_garden";
    const VENDOR: &'static str = "Stupid++";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "stupidplusplus@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let editor_state = self.params.editor_state.clone();
        let params = self.params.clone();
        let runtime_status = self.runtime_data.clone();
        let interface_data = self.interface_data.clone();
        let interface = Interface::new();
        
        self.runtime.console = Some(interface.console.create_sender());
        let editor = interface.create_interface(editor_state, params, runtime_status, interface_data);

        return editor;
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        let _ = self.runtime.init(Some(_buffer_config.sample_rate));

        return true;
    }

    fn reset(&mut self) {
        let runtime_data_lock = self.runtime_data.clone();
        let mut runtime_data = runtime_data_lock.write().unwrap();
        
        if runtime_data.state == RuntimeState::Online {
            let runtime_success = self.runtime.reset();
        
            if !runtime_success {
                runtime_data.set_state(RuntimeState::Offline);
            }
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let runtime_data_lock = self.runtime_data.clone();
        let mut runtime_data = runtime_data_lock.write().unwrap();
        let interface_data = self.interface_data.read().unwrap().clone();

        runtime_data.update_from_interface(&interface_data);

        match runtime_data.state {
            RuntimeState::Refresh => {
                self.refresh_runtime_module(&interface_data);
                runtime_data.set_state(RuntimeState::Online);
    
                let runtime_success = self.runtime.init(None);            
                if !runtime_success {
                    runtime_data.set_state(RuntimeState::Offline);
                }
    
                let runtime_success = self.runtime.reset();
            
                if !runtime_success {
                    runtime_data.set_state(RuntimeState::Offline);
                }
            },
            RuntimeState::Clear => {
                self.clear_runtime_module();
                runtime_data.set_state(RuntimeState::Offline);
            },
            _ => ()
        }

        if runtime_data.state == RuntimeState::Online {
            self.runtime.set_clip(runtime_data.clip);
            self.runtime.set_input_noise(runtime_data.input_noise);
            let runtime_success = self.runtime.run(buffer);
    
            if !runtime_success {
                runtime_data.set_state(RuntimeState::Offline);
            }
        }
        
        runtime_data.update_from_runtime(&mut self.runtime, &interface_data);

        self.update_runtime_status(&mut runtime_data);

        return ProcessStatus::Normal;
    }
}

impl ClapPlugin for LuaGarden {
    const CLAP_ID: &'static str = "com.stupidplusplus.lua_garden";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A sonic programming playgound.");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    // Don't forget to change these features
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for LuaGarden {
    const VST3_CLASS_ID: [u8; 16] = *b"lua_gardenSTUPID";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(LuaGarden);
nih_export_vst3!(LuaGarden);
