// lib.rs
//
// Copyright (c) 2025 Junpei Kawamoto
//
// This software is released under the MIT License.
//
// http://opensource.org/licenses/mit-license.php

//! This crate provides the source build of
//! [oneAPI Deep Neural Network Library (oneDNN)](https://!github.com/uxlfoundation/oneDNN) for Rust projects.
//!
//! ## Usage
//!
//! Add this crate to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! onednn-src = "0.1.2"
//! ```
//!
//! Then, add the extern declaration to your `main.rs` or `lib.rs` to ensure the libraries are properly linked:
//!
//! ```rust
//! extern crate onednn_src;
//! ```
//!
//! This crate exports the following environment variables for locating the oneDNN library in your build scripts:
//!
//! - `DEP_DNNL_ROOT`: Points to the root directory containing the compiled oneDNN library and all its files,
//! - `DEP_DNNL_INCLUDE_PATH`: Points to the directory containing oneDNN header files needed for compilation,
//! - `DEP_DNNL_LIBRARY_PATH`: Points to the directory containing the compiled oneDNN static libraries.
//!
//! ## Features
//!
//! - `graph`: Enables the graph component.
//! - `training`: Enables training functionality.
//! - `inference`: Enables inference functionality.
//! - `experimental`: Enables [experimental features](https://!uxlfoundation.github.io/oneDNN/dev_guide_experimental.html#doxid-dev-guide-experimental)
//! - `verbose`: Enables [verbose mode](https://!uxlfoundation.github.io/oneDNN/dev_guide_verbose.html#doxid-dev-guide-verbose)
//!
//! By default, the `training` feature is enabled.
//! The `training` and `inference` features are mutually exclusive.
//! If neither is specified, `training` will be used.

#![no_std]
