use mlem_base::{base::mlem_metadata::MlemMetadata};
use nih_plug::prelude::*;

pub const PLUGIN_METADATA: MlemMetadata = MlemMetadata::new(
    "\u{E564}",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_HOMEPAGE"),
    env!("CARGO_PKG_HOMEPAGE"),
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_AUTHORS"),
    env!("CARGO_PKG_DESCRIPTION"),
    "ANTI-CAPITALIST SOFTWARE LICENSE v 1.4",
    include_str!("../LICENSE"),
    concat!(include_str!("../credits.txt"), "\n\n", include_str!("../../mlem_base/credits.txt")),
    "Mlem Records",
    "support@mlemrecords.com", 
    "com.mlemrecords.mlem_data", 
    *b"dataMLEM        ", 
    
    &[ClapFeature::AudioEffect, ClapFeature::Stereo], 
    &[Vst3SubCategory::Fx], 

    384,
    384,
    mlem_egui_themes::MLEM_DARK
);
pub const DISCLAIMER: &str = include_str!("../disclaimer.txt");
pub const DEFAULT_DATA: &[u8; 1061034] = include_bytes!("../default_data");