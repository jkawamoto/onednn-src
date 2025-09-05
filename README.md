# oneDNN-src

[![GitHub License](https://img.shields.io/github/license/jkawamoto/onednn-src)](https://github.com/jkawamoto/onednn-src/blob/main/LICENSE)

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

This application is released under the MIT License. For details, see the [LICENSE](LICENSE) file.
