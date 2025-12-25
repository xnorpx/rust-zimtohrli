# zimtohrli-sys

Low-level Rust FFI bindings to the [Zimtohrli](https://github.com/google/zimtohrli) psychoacoustic perceptual audio metric library.

## About Zimtohrli

Zimtohrli is a psychoacoustic perceptual metric that quantifies the human-observable difference between two audio signals. It's particularly focused on just-noticeable-differences for high-quality audio compression evaluation. For more details, see the [paper](https://arxiv.org/pdf/2509.26133).

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
zimtohrli-sys = "0.2"
```

### Example

```rust
use zimtohrli_sys::ffi;

fn main() {
    // Create a new Zimtohrli analyzer
    let zimtohrli = ffi::new_zimtohrli();
    
    // Audio samples must be:
    // - f32 values in range [-1.0, 1.0]
    // - 48kHz sample rate
    // - Mono
    let samples_a: Vec<f32> = load_audio_a(); // your audio loading code
    let samples_b: Vec<f32> = load_audio_b();
    
    // Analyze both signals to get spectrograms
    let mut spec_a = zimtohrli.analyze(&samples_a);
    let mut spec_b = zimtohrli.analyze(&samples_b);
    
    // Compute perceptual distance
    // Returns value in [0, 1] where:
    // - 0 = identical
    // - 1 = maximally different
    let distance = zimtohrli.distance(spec_a.pin_mut(), spec_b.pin_mut());
    
    println!("Perceptual distance: {}", distance);
}
```

## Requirements

- C++17 compatible compiler
- The vendored zimtohrli header is compiled automatically

### Fast-math

By default this crate enables compiler fast-math flags for the C++ bridge, which may change numerical results.

- GCC/Clang: `-fassociative-math -freciprocal-math -fno-signed-zeros -fno-math-errno`
- MSVC: `/fp:fast`

To disable it:

```toml
[dependencies]
zimtohrli-sys = { version = "0.1", default-features = false }
```

## Vendored Source

This crate vendors the `zimtohrli.h` header from the upstream repository. See `vendor/ZIMTOHRLI_VERSION` for the exact version information.

## License

This crate is licensed under the Apache License 2.0, matching the upstream Zimtohrli library.

## Links

- [Zimtohrli GitHub](https://github.com/google/zimtohrli)
- [Zimtohrli Paper (arXiv)](https://arxiv.org/pdf/2509.26133) - "Zimtohrli: A Perceptual Audio Quality Metric"
