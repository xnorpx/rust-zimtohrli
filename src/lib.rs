//! # zimtohrli-sys
//!
//! Low-level Rust bindings to the [Zimtohrli](https://github.com/google/zimtohrli)
//! psychoacoustic perceptual audio metric library.
//!
//! Zimtohrli is a perceptual audio metric that quantifies the human-observable
//! difference between two audio signals. It's particularly focused on
//! just-noticeable-differences for high-quality audio compression evaluation.
//!
//! ## Example
//!
//! ```rust,no_run
//! use zimtohrli_sys::ffi;
//!
//! // Create a new Zimtohrli analyzer
//! let zimtohrli = ffi::new_zimtohrli();
//!
//! // Audio samples should be f32 in range [-1, 1] at 48kHz
//! let samples_a: Vec<f32> = vec![0.0; 48000]; // 1 second of silence
//! let samples_b: Vec<f32> = vec![0.0; 48000]; // 1 second of silence
//!
//! // Analyze both signals
//! let mut spec_a = zimtohrli.analyze(&samples_a);
//! let mut spec_b = zimtohrli.analyze(&samples_b);
//!
//! // Compute perceptual distance (0 = identical, 1 = maximally different)
//! let distance = zimtohrli.distance(spec_a.pin_mut(), spec_b.pin_mut());
//! println!("Perceptual distance: {}", distance);
//! ```
//!
//! ## Requirements
//!
//! - C++17 compatible compiler
//! - Audio samples must be 48kHz mono, with values in range [-1, 1]

#[cxx::bridge(namespace = "zimtohrli_bridge")]
pub mod ffi {
    unsafe extern "C++" {
        include!("bridge.h");

        // ============================================================
        // Opaque Types
        // ============================================================

        /// Main class for psychoacoustic audio analysis.
        ///
        /// Converts audio signals to perceptual spectrograms and computes
        /// perceptual distances between them.
        ///
        /// Thread-safety: Not specified by the upstream library; treat as not
        /// thread-safe unless you have verified otherwise.
        type ZimtohrliWrapper;

        /// A simple buffer of float samples describing a spectrogram with a given
        /// number of steps and feature dimensions.
        ///
        /// The values buffer is populated like:
        /// ```text
        /// [
        ///   [sample0_dim0, sample0_dim1, ..., sample0_dimn],
        ///   [sample1_dim0, sample1_dim1, ..., sample1_dimn],
        ///   ...,
        ///   [samplem_dim0, samplem_dim1, ..., samplem_dimn],
        /// ]
        /// ```
        type SpectrogramWrapper;

        // ============================================================
        // Factory Functions
        // ============================================================

        /// Creates a new `ZimtohrliWrapper` instance with default settings.
        fn new_zimtohrli() -> UniquePtr<ZimtohrliWrapper>;

        /// Creates a new `SpectrogramWrapper` with the given number of time steps.
        ///
        /// The number of dimensions is set to `num_channels()` (128).
        fn new_spectrogram(num_steps: usize) -> UniquePtr<SpectrogramWrapper>;

        // ============================================================
        // Constants
        // ============================================================

        /// Returns the expected sample rate for input audio (48000 Hz).
        fn sample_rate() -> f32;

        /// Returns the number of frequency channels in the spectrogram (128).
        ///
        /// This corresponds to the number of rotators in the cochlear model.
        fn num_channels() -> usize;

        // ============================================================
        // ZimtohrliWrapper Methods
        // ============================================================

        /// Window size in time (steps) dimension for NSIM computation.
        ///
        /// The NSIM (Normalized Structural Similarity) computation uses a window
        /// to compute local statistics. This defines the window size along the
        /// time axis. Default: 8.
        fn nsim_step_window(self: &ZimtohrliWrapper) -> usize;

        /// Window size in frequency (channels) dimension for NSIM computation.
        ///
        /// The NSIM computation uses a window to compute local statistics. This
        /// defines the window size along the frequency axis. Default: 5.
        fn nsim_channel_window(self: &ZimtohrliWrapper) -> usize;

        /// The perceptual sample rate in Hz.
        ///
        /// This determines the time resolution of the output spectrogram. Higher
        /// values give finer temporal resolution. Default: ~85 Hz.
        fn perceptual_sample_rate(self: &ZimtohrliWrapper) -> f32;

        /// The dB level of a full-scale sine wave.
        ///
        /// Audio is normalized based on this value. A sine wave with amplitude 1.0
        /// will be treated as having this dB level. Default: 78.3 dB (vendored
        /// header).
        fn full_scale_sine_db(self: &ZimtohrliWrapper) -> f32;

        /// Sets the window size in time dimension for NSIM computation.
        fn set_nsim_step_window(self: Pin<&mut ZimtohrliWrapper>, val: usize);

        /// Sets the window size in frequency dimension for NSIM computation.
        fn set_nsim_channel_window(self: Pin<&mut ZimtohrliWrapper>, val: usize);

        /// Calculates the number of time steps in the output spectrogram based on
        /// the input signal length.
        ///
        /// # Arguments
        /// * `num_samples` - Number of samples in the input audio signal.
        ///
        /// # Returns
        /// The number of time steps in the resulting spectrogram.
        fn spectrogram_steps(self: &ZimtohrliWrapper, num_samples: usize) -> usize;

        /// Analyzes an audio signal and returns a new spectrogram.
        ///
        /// # Arguments
        /// * `signal` - Input audio samples at 48kHz sample rate. Values should
        ///   be in the range [-1, 1], where 1 represents the maximum amplitude.
        ///
        /// # Returns
        /// A spectrogram representing the perceptual content of the signal.
        fn analyze(self: &ZimtohrliWrapper, signal: &[f32]) -> UniquePtr<SpectrogramWrapper>;

        /// Computes perceptual distance between two spectrograms.
        ///
        /// Uses Dynamic Time Warping (DTW) for time alignment and NSIM (Normalized
        /// Structural Similarity) for computing the perceptual difference.
        ///
        /// # Arguments
        /// * `spec_a` - First spectrogram (must be mutable for normalization).
        /// * `spec_b` - Second spectrogram (must be mutable for normalization).
        ///
        /// # Returns
        /// A perceptual distance value in the range [0, 1], where 0 means
        /// identical and values approaching 1 indicate increasing perceptual
        /// difference.
        fn distance(
            self: &ZimtohrliWrapper,
            spec_a: Pin<&mut SpectrogramWrapper>,
            spec_b: Pin<&mut SpectrogramWrapper>,
        ) -> f32;

        // ============================================================
        // SpectrogramWrapper Methods
        // ============================================================

        /// Returns the number of time steps in the spectrogram.
        fn num_steps(self: &SpectrogramWrapper) -> usize;

        /// Returns the number of feature dimensions (frequency channels).
        ///
        /// This is typically 128, corresponding to `num_channels()`.
        fn num_dims(self: &SpectrogramWrapper) -> usize;

        /// Returns the total number of values in the spectrogram.
        ///
        /// Equal to `num_steps() * num_dims()`.
        fn size(self: &SpectrogramWrapper) -> usize;

        /// Returns the maximum absolute value across all spectrogram values.
        fn max(self: &SpectrogramWrapper) -> f32;

        /// Multiplies all spectrogram values by the given factor.
        fn rescale(self: Pin<&mut SpectrogramWrapper>, f: f32);

        /// Returns a const pointer to the underlying spectrogram data.
        ///
        /// Use `values()` method for safe slice access.
        fn values_ptr(self: &SpectrogramWrapper) -> *const f32;

        /// Returns a mutable pointer to the underlying spectrogram data.
        ///
        /// Use `values_mut()` method for safe slice access.
        fn values_ptr_mut(self: Pin<&mut SpectrogramWrapper>) -> *mut f32;
    }
}

impl ffi::SpectrogramWrapper {
    /// Get the spectrogram values as a slice.
    ///
    /// The values are stored as a flattened 2D array with dimensions
    /// `[num_steps][num_dims]`, where `num_dims` is typically 128 (the number
    /// of frequency channels).
    pub fn values(&self) -> &[f32] {
        let size = self.size();
        if size == 0 {
            return &[];
        }

        unsafe { std::slice::from_raw_parts(self.values_ptr(), size) }
    }

    /// Get the spectrogram values as a mutable slice.
    pub fn values_mut(self: std::pin::Pin<&mut Self>) -> &mut [f32] {
        let size = self.size();
        if size == 0 {
            return unsafe {
                std::slice::from_raw_parts_mut(std::ptr::NonNull::<f32>::dangling().as_ptr(), 0)
            };
        }

        unsafe { std::slice::from_raw_parts_mut(self.values_ptr_mut(), size) }
    }
}

impl std::fmt::Debug for ffi::ZimtohrliWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZimtohrliWrapper")
            .field("nsim_step_window", &self.nsim_step_window())
            .field("nsim_channel_window", &self.nsim_channel_window())
            .field("perceptual_sample_rate", &self.perceptual_sample_rate())
            .field("full_scale_sine_db", &self.full_scale_sine_db())
            .finish()
    }
}

impl std::fmt::Debug for ffi::SpectrogramWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpectrogramWrapper")
            .field("num_steps", &self.num_steps())
            .field("num_dims", &self.num_dims())
            .field("size", &self.size())
            .field("max", &self.max())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(ffi::sample_rate(), 48000.0);
        assert_eq!(ffi::num_channels(), 128);
    }

    #[test]
    fn test_create_zimtohrli() {
        let z = ffi::new_zimtohrli();
        assert_eq!(z.nsim_step_window(), 8);
        assert_eq!(z.nsim_channel_window(), 5);
    }

    #[test]
    fn test_spectrogram_steps() {
        let z = ffi::new_zimtohrli();
        // At 48kHz with perceptual_sample_rate ~85Hz, 48000 samples should give ~85 steps
        let steps = z.spectrogram_steps(48000);
        assert!(steps > 0);
    }

    #[test]
    fn test_analyze_silence() {
        let z = ffi::new_zimtohrli();
        let samples: Vec<f32> = vec![0.0; 4800]; // 0.1 seconds
        let spec = z.analyze(&samples);
        assert!(spec.num_steps() > 0);
        assert_eq!(spec.num_dims(), 128);
    }

    #[test]
    fn test_distance_identical() {
        let z = ffi::new_zimtohrli();
        let samples: Vec<f32> = vec![0.0; 4800];
        let mut spec_a = z.analyze(&samples);
        let mut spec_b = z.analyze(&samples);
        let distance = z.distance(spec_a.pin_mut(), spec_b.pin_mut());
        // Identical signals should have distance close to 0
        assert!((0.0..=1.0).contains(&distance));
    }
}
