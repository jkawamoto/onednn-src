// build.rs
//
// Copyright (c) 2025 Junpei Kawamoto
//
// This software is released under the MIT License.
//
// http://opensource.org/licenses/mit-license.php

use cmake::Config;
use std::env;

macro_rules! feature_to_str {
    ($feature:literal) => {
        if cfg!(feature = $feature) {
            "ON"
        } else {
            "OFF"
        }
    };
    ($feature:literal, $on:expr, $off:expr) => {
        if cfg!(feature = $feature) { $on } else { $off }
    };
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-env-changed=DOCS_RS");
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let cmake = Config::new("oneDNN")
        .define("ONEDNN_LIBRARY_TYPE", "STATIC")
        .define("ONEDNN_BUILD_DOC", "OFF")
        .define("ONEDNN_BUILD_EXAMPLES", "OFF")
        .define("ONEDNN_BUILD_TESTS", "OFF")
        .define("ONEDNN_BUILD_GRAPH", feature_to_str!("graph"))
        .define(
            "ONEDNN_ENABLE_WORKLOAD",
            feature_to_str!("inference", "INFERENCE", "TRAINING"),
        )
        .define("ONEDNN_EXPERIMENTAL", feature_to_str!("experimental"))
        .define("ONEDNN_VERBOSE", feature_to_str!("verbose"))
        .define("ONEDNN_DEV_MODE", feature_to_str!("dev"))
        .define(
            "ONEDNN_BLAS_VENDOR",
            if cfg!(target_os = "macos") {
                "ACCELERATE"
            } else if cfg!(feature = "armpl") {
                "ARMPL"
            } else {
                "NONE"
            },
        )
        .build();
    println!("cargo::rustc-link-search={}/lib", cmake.display());
    println!("cargo::rustc-link-lib=static=dnnl");
    println!("cargo::metadata=ROOT={}", cmake.display());
    println!(
        "cargo::metadata=INCLUDE_PATH={}",
        cmake.join("include").display()
    );
    println!(
        "cargo::metadata=LIBRARY_PATH={}",
        cmake.join("lib").display()
    );
}
