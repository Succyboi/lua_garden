pub mod consts;
pub mod params;
pub mod interface;
pub mod runtime;

use mlem_base::{base::{mlem_metadata::MlemMetadata, mlem_plugin::MlemPlugin}, runtime::Runtime};
use mlem_base::{ interface::{ Interface } };
use nih_plug::prelude::*;
use std::{sync::{ Arc }};
use consts::PLUGIN_METADATA;
use crate::{interface::TidbitInterface, params::MP3Params, runtime::MP3Runtime};

pub struct MP3 {
    params: Arc<MP3Params>,
    runtime: Runtime<MP3Params, MP3Runtime>
}

impl Default for MP3 {
    fn default() -> Self {
        let params = Arc::new(MP3Params::default());
        let runtime = Runtime::new(params.clone(), MP3Runtime::new(params.clone()));

        let new = Self {
            params,
            runtime
        };

        return new;
    }
}

impl MP3 { }

impl MlemPlugin<MP3Params> for MP3 {
    fn metadata(&self) -> MlemMetadata {
        return consts::PLUGIN_METADATA;
    }
    
    fn params(&self) ->  Arc<MP3Params> {
        return self.params.clone();
    }
}

impl Plugin for MP3 {
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
        let interface = TidbitInterface::new(self.params.clone());
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

impl ClapPlugin for MP3 {
    const CLAP_ID: &'static str = PLUGIN_METADATA.identifier;
    const CLAP_DESCRIPTION: Option<&'static str> = Some(PLUGIN_METADATA.description);
    const CLAP_MANUAL_URL: Option<&'static str> = Some(PLUGIN_METADATA.homepage_url);
    const CLAP_SUPPORT_URL: Option<&'static str> = Some(PLUGIN_METADATA.support_url);

    const CLAP_FEATURES: &'static [ClapFeature] = PLUGIN_METADATA.clap_features;
}

impl Vst3Plugin for MP3 {
    const VST3_CLASS_ID: [u8; 16] = PLUGIN_METADATA.class_identifier;

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = PLUGIN_METADATA.vst3_subcategories;
}

nih_export_clap!(MP3);
nih_export_vst3!(MP3);
