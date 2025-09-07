// build.rs
//
// Copyright (c) 2025 Junpei Kawamoto
//
// This software is released under the MIT License.
//
// http://opensource.org/licenses/mit-license.php

use cmake::Config;
use std::env;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let cmake = Config::new("oneDNN")
        .define("ONEDNN_LIBRARY_TYPE", "STATIC")
        .define("ONEDNN_BUILD_DOC", "OFF")
        .define("ONEDNN_BUILD_EXAMPLES", "OFF")
        .define("ONEDNN_BUILD_TESTS", "OFF")
        .define(
            "ONEDNN_BUILD_GRAPH",
            if cfg!(feature = "graph") { "ON" } else { "OFF" },
        )
        .define(
            "ONEDNN_ENABLE_WORKLOAD",
            if cfg!(feature = "inference") {
                "INFERENCE"
            } else {
                "TRAINING"
            },
        )
        .define(
            "ONEDNN_EXPERIMENTAL",
            if cfg!(feature = "experimental") {
                "ON"
            } else {
                "OFF"
            },
        )
        .define(
            "ONEDNN_VERBOSE",
            if cfg!(feature = "verbose") {
                "ON"
            } else {
                "OFF"
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
