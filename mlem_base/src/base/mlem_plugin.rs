use std::sync::Arc;
use crate::base::{mlem_metadata::MlemMetadata, mlem_params::MlemParams};

pub trait MlemPlugin<T: MlemParams>: 'static + Send + Sync {
    fn metadata(&self) -> MlemMetadata;
    fn params(&self) ->  Arc<T>;
}