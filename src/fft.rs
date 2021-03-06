//! Analyze discrete signal in frequency domain

use rustfft::{FFT};
use signals::{Signal};
use spectrums::{Spectrum};


pub struct ForwardFFT {
    fft: FFT<f64>,
}

pub struct InverseFFT {
    fft: FFT<f64>,
}

impl ForwardFFT {
    /// Define new transformation
    /// ## Params:
    ///   * sample_rate - Samples per second (1/sample_frequency)
    ///   * sample_size - Size of the vector which will be converter. Should be power of 2 (or 3)
    pub fn new(sample_size: usize) -> ForwardFFT {
        let fft = FFT::new(sample_size, false);
        ForwardFFT{ fft }
    }

    /// Forward DFT (implemented as FFT)
    pub fn process(&mut self, v: &Signal) -> Spectrum {
        let raw_vec = v.to_vec();
        let mut out = raw_vec.clone();

        self.fft.process(&raw_vec, &mut out);
        Spectrum::new(out, v.sample_rate)
    }
}


impl InverseFFT {
    /// Define new transformation
    /// ## Params:
    ///   * sample_size - Size of the vector which will be converter. Should be power of 2 (or 3)
    pub fn new(sample_size: usize) -> InverseFFT {
        let fft = FFT::new(sample_size, true);
        InverseFFT{ fft }
    }

    /// Forward DFT (implemented as FFT)
    pub fn process(&mut self, v: &Spectrum) -> Signal {
        let raw_vec = v.to_vec();
        let mut out = raw_vec.clone();

        self.fft.process(&raw_vec, &mut out);
        Signal::new(out)
    }
}


/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use num_complex::{Complex};
    use signals::{Signal};
    use spectrums::{Spectrum};
    use super::*;

    #[test]
    fn test_fft() {
        let v = Signal::from_reals(vec![1., 0., 0., 0.], 4);
        let mut ft = ForwardFFT::new(4);
        let s = ft.process(&v);
        assert_eq!(s, Spectrum::new(vec![Complex::new(1., 0.),
                                              Complex::new(1., 0.),
                                              Complex::new(1., 0.),
                                              Complex::new(1., 0.)], 4));
    }

}