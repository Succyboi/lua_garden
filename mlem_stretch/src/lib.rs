pub mod consts;
pub mod params;
pub mod interface;
pub mod runtime;

use mlem_base::{base::{mlem_interface::MlemInterface, mlem_metadata::MlemMetadata, mlem_params::MlemParams, mlem_plugin::MlemPlugin, mlem_runtime::MlemRuntime}, console::ConsoleSender, runtime::Runtime};
use atomic_float::{ AtomicF32 };
use mlem_base::{interface::{param_drag_value, param_toggle }};
use mlem_base::{ interface::{ Interface } };
use nih_plug::prelude::*;
use std::{collections::VecDeque, default, ops::Deref, sync::{ Arc, atomic::{AtomicBool, AtomicUsize, Ordering} }, time::{SystemTime, UNIX_EPOCH}};
use nih_plug_egui::{EguiState, egui::{Align, Context, Layout, Ui}};
use consts::PLUGIN_METADATA;
use crate::{interface::StretchInterface, params::StretchParams, runtime::StretchRuntime};

pub struct Stretch {
    params: Arc<StretchParams>,
    runtime: Runtime<StretchParams, StretchRuntime>
}

impl Default for Stretch {
    fn default() -> Self {
        let params = Arc::new(StretchParams::default());
        let runtime = Runtime::new(params.clone(), StretchRuntime::new(params.clone()));

        let stretch = Self {
            params,
            runtime
        };

        return stretch;
    }
}

impl Stretch { }

impl MlemPlugin<StretchParams> for Stretch {
    fn metadata(&self) -> MlemMetadata {
        return consts::PLUGIN_METADATA;
    }
    
    fn params(&self) ->  Arc<StretchParams> {
        return self.params.clone();
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
        let interface = StretchInterface::new(self.params.clone());
        let interface = Interface::new(self.params.clone(), interface, self.metadata());
        
        let editor_state = self.params.editor_state.clone();
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
        self.runtime.run(buffer, context.transport());

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
