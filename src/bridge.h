// zimtohrli-sys bridge for cxx
//
// This file provides a C++ interface compatible with cxx for the zimtohrli library.

#pragma once

#include "zimtohrli.h"
#include "rust/cxx.h"
#include <memory>
#include <utility>

namespace zimtohrli_bridge {

// Opaque wrapper around zimtohrli::Spectrogram for Rust
struct SpectrogramWrapper {
    zimtohrli::Spectrogram inner;
    
    SpectrogramWrapper(size_t num_steps) : inner(num_steps) {}
    SpectrogramWrapper(zimtohrli::Spectrogram&& spec) : inner(std::move(spec)) {}
    
    size_t num_steps() const { return inner.num_steps; }
    size_t num_dims() const { return inner.num_dims; }
    size_t size() const { return inner.size(); }
    float max() const { return inner.max(); }
    void rescale(float f) { inner.rescale(f); }
    
    // Get raw pointer to values for Rust to read
    const float* values_ptr() const { return inner.values.get(); }
    float* values_ptr_mut() { return inner.values.get(); }
};

// Opaque wrapper around zimtohrli::Zimtohrli for Rust
struct ZimtohrliWrapper {
    zimtohrli::Zimtohrli inner;
    
    ZimtohrliWrapper() : inner() {}
    
    // Getters for configuration
    size_t nsim_step_window() const { return inner.nsim_step_window; }
    size_t nsim_channel_window() const { return inner.nsim_channel_window; }
    float perceptual_sample_rate() const { return inner.perceptual_sample_rate; }
    float full_scale_sine_db() const { return inner.full_scale_sine_db; }
    
    // Setters for configuration
    void set_nsim_step_window(size_t val) { inner.nsim_step_window = val; }
    void set_nsim_channel_window(size_t val) { inner.nsim_channel_window = val; }
    
    // Calculate spectrogram steps for a given number of samples
    size_t spectrogram_steps(size_t num_samples) const {
        return inner.SpectrogramSteps(num_samples);
    }
    
    // Analyze audio samples and return a spectrogram
    std::unique_ptr<SpectrogramWrapper> analyze(rust::Slice<const float> signal) const {
        zimtohrli::Span<const float> span(signal.data(), signal.size());
        auto spec = inner.Analyze(span);
        return std::make_unique<SpectrogramWrapper>(std::move(spec));
    }
    
    // Compute distance between two spectrograms
    // Note: This modifies the spectrograms (rescaling), so they are taken by non-const pointer
    float distance(SpectrogramWrapper& spec_a, SpectrogramWrapper& spec_b) const {
        return inner.Distance(spec_a.inner, spec_b.inner);
    }
};

// Factory functions for creating wrappers
inline std::unique_ptr<ZimtohrliWrapper> new_zimtohrli() {
    return std::make_unique<ZimtohrliWrapper>();
}

inline std::unique_ptr<SpectrogramWrapper> new_spectrogram(size_t num_steps) {
    return std::make_unique<SpectrogramWrapper>(num_steps);
}

// Get the expected sample rate (48000 Hz)
inline float sample_rate() {
    return 48000.0f;
}

// Get the number of frequency channels (128)
inline size_t num_channels() {
    return 128;
}

} // namespace zimtohrli_bridge
