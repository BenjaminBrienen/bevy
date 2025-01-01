#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "https://bevyengine.org/assets/icon.png",
    html_favicon_url = "https://bevyengine.org/assets/icon.png"
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "bevy-support")]
pub mod commands;
/// The basic components of the transform crate
pub mod components;

/// Transform related bundles
#[cfg(feature = "bevy-support")]
pub mod bundles;

/// Transform related traits
pub mod traits;

/// Transform related plugins
#[cfg(feature = "bevy-support")]
pub mod plugins;

/// [`GlobalTransform`]: components::GlobalTransform
/// Helpers related to computing global transforms
#[cfg(feature = "bevy-support")]
pub mod helper;
/// Systems responsible for transform propagation
#[cfg(feature = "bevy-support")]
pub mod systems;

/// The transform prelude.
///
/// This includes the most common types in this crate, re-exported for your convenience.
#[doc(hidden)]
#[expect(deprecated)]
pub mod prelude {
    #[doc(hidden)]
    pub use crate::components::*;

    #[cfg(feature = "bevy-support")]
    #[doc(hidden)]
    pub use crate::{
        bundles::TransformBundle,
        commands::BuildChildrenTransformExt,
        helper::TransformHelper,
        plugins::{TransformPlugin, TransformSystem},
        traits::TransformPoint,
    };
}

#[cfg(feature = "bevy-support")]
pub use prelude::{TransformPlugin, TransformPoint, TransformSystem};
