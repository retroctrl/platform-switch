// Copyright (c) Jake Swensen
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Namespace facades for switching between platforms.
//!
//! # Motivation
//! Sometimes code needs to be shared between a contrainted platform
//! (such as an embedded system) and a target that has a more fully
//! featured environment. This crate provides a series of module wrappers
//! that can be used in combination with cargo manifest feature definitions
//! to switch between options at compile time.
//!
//! # Example
//! Using `thiserror` with `defmt` enabled on an embedded system:
//!
//! ```toml
//! [features]
//! default = ["std"]
//! std = ["platform-switch/std_error"]
//! mcu = ["platform-switch/core_error", "platform-switch/defmt"] # Requires nightly
//! ```
//!
//! ```rust
//! use platform_switch::log as log;
//! use platform_switch::thiserror as thiserror;
//!
//! #[derive(thiserror::Error, Debug)]
//! enum ExampleError {
//!     #[error("Error #1")]
//!     Error1,
//!     #[error("Error #2")]
//!     Error2,
//! }
//!
//! fn error_logger() {
//!     log::trace!("Trace");
//!     log::debug!("Debug");
//!     log::info!("Info");
//!     log::warn!("Warn");
//!     log::error!("Error");
//!
//!     log::error!("Error: {}", ExampleError::Error1);
//!     log::error!("Error: {}", ExampleError::Error2);
//! }
//! ```
//! 
//! ## Stable Toolchain
//! If a stable toolchain is required, `thiserror` can be disabled with the following features and attributes:
//! ```toml
//! [features]
//! default = ["std"]
//! std = ["thiserror", "platform-switch/std_error"]
//! mcu = ["platform-switch/defmt"]
//! thiserror = []
//! ```
//!
//! ```rust
//! use platform_switch::log as log;
//! use platform_switch::thiserror as thiserror;
//!
//! #[derive(Debug)]
//! #[cfg_attr(feature = "thiserror", derive(thiserror::Error))]
//! enum ExampleError {
//!     #[cfg_attr(feature = "thiserror", error("Error #1"))]
//!     Error1,
//!
//!     #[cfg_attr(feature = "thiserror", error("Error #2"))]
//!     Error2,
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(not(any(feature = "std", test)), feature = "core_error"), feature(error_in_core))]

/// A namespace facade around [`thiserror`].
///
/// Enables [`thiserror`] to be used in both [`std`] and `no_std` environments.
/// If configured for `no_std`, this module will use `thiserror-core` (which requires
/// a nightly toolchain).
///
/// ### Note
/// This module will be marked deprecated once `error_in_core` is
/// [stabilized](https://github.com/rust-lang/rust/issues/103765) and [`thiserror`]
/// fully supports using [`core::error`].
#[cfg(feature = "thiserror")]
pub mod thiserror {
    cfg_if::cfg_if! {
        if #[cfg(not(feature = "core_error"))] {
            pub use thiserror::*;
        } else {
            pub use thiserror_core::*;
        }
    }
}

/// A namespace facade around [`log`] and `defmt`.
///
/// Enabling the feature `defmt` will re-export the `defmt` macros for logging.
/// Otherwise, the `log` macros will be re-exported.
#[cfg(feature = "log")]
pub mod log {
    cfg_if::cfg_if! {
        if #[cfg(feature = "defmt")] {
            pub use defmt::debug as debug;
            pub use defmt::error as error;
            pub use defmt::info as info;
            pub use defmt::trace as trace;
            pub use defmt::warn as warn;
        } else {
            pub use log::debug as debug;
            pub use log::error as error;
            pub use log::info as info;
            pub use log::trace as trace;
            pub use log::warn as warn;
        }
    }
}

/// A namespace facade for formatting.
pub mod fmt {
    cfg_if::cfg_if! {
        if #[cfg(feature = "std")] {
            pub use std::fmt::Debug as Debug;
            pub use std::fmt::Display as Display;
            pub use std::fmt::Formatter as Formatter;
            pub use std::fmt::Result as Result;
        } else {
            pub use core::fmt::Debug as Debug;
            pub use core::fmt::Display as Display;
            pub use core::fmt::Formatter as Formatter;
            pub use core::fmt::Result as Result;
        }
    }
}
