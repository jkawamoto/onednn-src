# oneDNN-src

[![GitHub License](https://img.shields.io/github/license/jkawamoto/onednn-src)](https://github.com/jkawamoto/onednn-src/blob/main/LICENSE)
[![Build](https://github.com/jkawamoto/onednn-src/actions/workflows/build.yaml/badge.svg)](https://github.com/jkawamoto/onednn-src/actions/workflows/build.yaml)

This crate provides the source build of
[oneAPI Deep Neural Network Library (oneDNN)](https://github.com/uxlfoundation/oneDNN) for Rust projects.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
onednn-src = "0.1"
```

Then, add the extern declaration to your `main.rs` or `lib.rs` to ensure the libraries are properly linked:

```rust
extern crate onednn_src;
```

This crate also exports the `DEP_DNNL_ROOT` environment variable,
which points to the root directory of the compiled oneDNN library.
You can use this in your `build.rs` to locate the library.

## Features

- `graph`: Enables the graph component.
- `training`: Enables training functionality.
- `inference`: Enables inference functionality.
- `experimental`: Enables [experimental features](https://uxlfoundation.github.io/oneDNN/dev_guide_experimental.html#doxid-dev-guide-experimental)
- `verbose`: Enables [verbose mode](https://uxlfoundation.github.io/oneDNN/dev_guide_verbose.html#doxid-dev-guide-verbose)

By default, the `training` feature is enabled.
The `training` and `inference` features are mutually exclusive.
If neither is specified, `training` will be used.

## Rust Compatibility

- Edition: Rust 2021
- MSRV: 1.77 (Minimum Supported Rust Version)

## License

This crate itself is licensed under the MIT License. See the [LICENSE](LICENSE) file.

oneDNN (which this crate builds and links to) is licensed under the Apache License, Version 2.0. If you redistribute binaries or source that include oneDNN, you must comply with the Apache-2.0 terms, including preserving license and notice files. For details, see oneDNN’s upstream LICENSE/NOTICE.

The published crate may build and/or redistribute artifacts originating from the following third-party projects. Each component remains under its own license, which applies in addition to this crate’s MIT license. When redistributing your software, ensure that you include the required attributions and license texts for any components that were used by your build configuration and target platform. The exact set of components used can depend on enabled features, target architecture, and toolchain.

The published crate also includes the following libraries:

3-clause BSD license:

- [Xbyak](https://github.com/herumi/xbyak)
- [gtest](https://github.com/google/googletest)
- [Instrumentation and Tracing Technology API
  (ITT API)](https://github.com/intel/ittapi)
- [CMake](https://github.com/Kitware/CMake)

2-clause BSD license:

- [Sphinx](https://www.sphinx-doc.org/)

Apache License Version 2.0:

- [Xbyak_aarch64](https://github.com/fujitsu/xbyak_aarch64)

Boost Software License, Version 1.0:

- [Boost C++ Libraries](https://www.boost.org/)

MIT License:

- [Intel Graphics Compute Runtime for oneAPI Level Zero
  and OpenCL Driver](https://github.com/intel/compute-runtime)
- [Intel Graphics Compiler](https://github.com/intel/intel-graphics-compiler)
- [oneAPI Level Zero](https://github.com/oneapi-src/level-zero)
- [Doxyrest](https://github.com/vovkos/doxyrest)
- [Intel Metrics Discovery Application Programming
  Interface](https://github.com/intel/metrics-discovery)
- [spdlog](https://github.com/gabime/spdlog)
- [sphinx-copybutton](https://github.com/executablebooks/sphinx-copybutton)

Notes:

- Redistribution guidance: If you distribute binaries produced with this crate, include the applicable third-party licenses and notices (e.g., Apache-2.0 NOTICE for oneDNN and attribution for BSD/Boost/MIT components).
- Features you enable (e.g., graph/training/inference/experimental/verbose) may influence which third-party components are compiled and thus which licenses apply to your distribution.
- This summary is provided for convenience and does not constitute legal advice. Please review the full license texts of each component.
