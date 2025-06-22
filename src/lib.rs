#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(rustdoc::redundant_explicit_links)]
#![doc = include_str!("../README.md")]
#[allow(unused_imports)]
use crate as bevy_slow_text_outline;

mod plugin;
mod text_outline;
pub(crate) mod text_outline_rendering;

pub mod prelude
{
    pub use crate::plugin::*;
    pub use crate::text_outline::*;
}
