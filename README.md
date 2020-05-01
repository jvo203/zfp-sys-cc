# zfp-sys-cc
Raw Rust bindings to ZFP (https://github.com/LLNL/zfp) with Windows 10 support.

A cross-platform "cc"-based crate with Windows 10 support, as opposed to the "cmake"-based "zfp-sys" crate that only worked in Linux and macOS.

The build supports an optional compilation with CUDA support, i.e. place this in your Cargo.toml:

zfp-sys-cc = {version = "*", features = ["cuda"]}

# Warning: the optional CUDA support in this crate has not been properly tested. Your mileage may vary.
