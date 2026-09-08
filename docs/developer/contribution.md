# Developer guide

## Prerequisites

- Rust 1.75+ (see `msrv.yml` for the tested matrix).
- wgpu runtime: Vulkan on Linux/Windows, Metal on macOS.
- For Android: Android NDK and `cargo-ndk`.

## Building

```sh
cargo build
cargo build --release
cargo build --all-features
```

## Cross-compiling for Android

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi \
    x86_64-linux-android i686-linux-android
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 build --release
```

The resulting `libstardew_view.so` files are placed in `android/libs/<abi>/`.

## Testing

```sh
cargo test                # unit tests
cargo test --test integration
cargo bench               # criterion benchmarks
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## CI

`.github/workflows` covers build, test, lint, docs, releases, security audits,
coverage, benchmarks, cross-compilation, and shader validation. See the
workflow files for exact commands.

## Docker

`scripts/docker/Dockerfile` provides a reproducible Vulkan build environment.
Android cross-builds run inside `scripts/docker/Dockerfile.android`.