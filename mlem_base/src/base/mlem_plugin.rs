use std::sync::Arc;
use nih_plug::{editor::Editor, params::Params, plugin::Plugin, prelude::{AsyncExecutor, ClapPlugin, Vst3Plugin}};

use crate::base::{mlem_interface::MlemInterface, mlem_metadata::MlemMetadata, mlem_params::MlemParams, mlem_runtime::MlemRuntime};

pub trait MlemPlugin<T: MlemParams>: 'static + Send + Sync {
    fn metadata(&self) -> MlemMetadata;
    fn params(&self) ->  Arc<T>;
}