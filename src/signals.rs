//! Process Discrete signals in time domain


use rand;
use rand::distributions::{Normal, IndependentSample};
use num_complex::{Complex, Complex64};
use vectors::{Vector, VectorImpl};

/// Discrete Time Signal
#[derive(Debug, PartialEq)]
pub struct Signal {
    data: Vector,
    sample_freq: usize
}


impl Signal {

    /// Create new signal from vector
    pub fn new(data: Vec<Complex64>) -> Signal {
        let n = data.len();
        Signal { data: data, sample_freq: n }
    }

    /// Create new signal from vector of real numbers
    pub fn from_reals(data: Vec<f64>) -> Signal {
        Signal { data: data.iter().map(|x| Complex::new(*x, 0.)).collect(),
                 sample_freq: data.len()}
    }

    /// Signal length()
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// This function will return 0 if index out of bound
    pub fn get(&self, i: isize) -> Complex64 {
        let s = self.data.len() as isize;
        if i < 0 || i >= s {
            Complex::new(0., 0.)
        } else {
        self.data[i as usize]
        }
    }

    /// Copy data into new vector
    pub fn to_vec(&self) -> Vec<Complex64> {
        self.data.clone()
    }

    /// Shift signal by given integer
    /// y[n] = x[n-k]
    /// This function will not change the signal length
    pub fn shift(&self, k: isize) -> Signal {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.data.len());
        let size: isize = self.data.len() as isize;
        for n in 0..size {
            v.push(self.get(n-k));
        }
        Signal::new(v)
    }

    /// Integrate signal
    /// y[n] = Sum x[k] For all k <= n
    pub fn integrate(&self) -> Signal {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.data.len());
        let mut acc = Complex::new(0., 0.);
        for n in 0..self.data.len() {
            acc = acc + self.data.at(n);
            v.push(acc);
        }
        Signal::new(v)
    }

    /// Differentiate the signal
    /// y[n] = x[n] - x[n-1]
    pub fn differentiate(&self) -> Signal {
        let mut v: Vec<Complex64> = Vec::with_capacity(self.data.len());
        let mut last = Complex::new(0., 0.);
        for n in 0..self.data.len() {
            v.push(self.data.at(n) - last);
            last = self.data.at(n);
        }
        Signal::new(v)
    }

    /// Calculate energy
    /// E = Sum x[n]^2 For all n
    pub fn energy(&self) -> f64 {
        self.data.iter().fold(0., |acc, &x| acc + (x*x.conj()).re)
    }

    /// Calculate power
    /// P = 1/N Sum x[n]^2 For all n
    pub fn power(&self) -> f64 {
        self.energy() / (self.data.len() as f64)
    }

    /// Add noise to the signal
    pub fn add_noise(&self, std: f64) -> Signal {
        let normal = Normal::new(0.0, std);
        let mut rng = rand::thread_rng();
        let data = self.data.iter().map(|x| x + normal.ind_sample(&mut rng)).collect();
        Signal { data: data, sample_freq: self.sample_freq }
    }

}




/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use num_complex::{Complex};
    use super::*;

    #[test]
    fn test_shift1() {
        let v = Signal::new(vec![Complex::new(1., 2.),
                                 Complex::new(2., 3.),
                                 Complex::new(3., 4.),
                                 Complex::new(4., 1.)]);
        let v1 = v.shift(1);
        assert!(v1 == Signal::new(vec![Complex::new(0., 0.),
                                       Complex::new(1., 2.),
                                       Complex::new(2., 3.),
                                       Complex::new(3., 4.)]));
    }

    #[test]
    fn test_shift2() {
        let v = Signal::from_reals(vec![1., 2., 3., 4.]);
        let v1 = v.shift(-1);
        assert!(v1 == Signal::new(vec![Complex::new(2., 0.),
                                       Complex::new(3., 0.),
                                       Complex::new(4., 0.),
                                       Complex::new(0., 0.)]));
    }

    #[test]
    fn test_integration() {
        let v = Signal::new(vec![Complex::new(1., 2.),
                                 Complex::new(2., -4.),
                                 Complex::new(3., -6.),
                                 Complex::new(4., 8.)]);
        let v2 = v.integrate();
        assert!(v2.len() == 4);
        assert!(v2 == Signal::new(vec![Complex::new(1., 2.),
                                       Complex::new(3., -2.),
                                       Complex::new(6., -8.),
                                       Complex::new(10., 0.)]));
    }

    #[test]
    fn test_differentiation() {
        let v = Signal::new(vec![Complex::new(1., 2.),
                                 Complex::new(2., -4.),
                                 Complex::new(3., -6.),
                                 Complex::new(4., 8.)]);
        let v2 = v.differentiate();
        assert!(v2.len() == 4);
        assert!(v2 == Signal::new(vec![Complex::new(1., 2.),
                                       Complex::new(1., -6.),
                                       Complex::new(1., -2.),
                                       Complex::new(1., 14.)]));
    }

    #[test]
    fn test_energy() {
        let v = Signal::new(vec![Complex::new(1., 1.),
                                 Complex::new(2., -1.),
                                 Complex::new(1., -1.),
                                 Complex::new(1., -2.)]);
        assert!(v.energy() == 14.0);
    }


    #[test]
    fn test_power() {
        let v = Signal::new(vec![Complex::new(1., 1.),
                                 Complex::new(2., -1.),
                                 Complex::new(1., -1.),
                                 Complex::new(1., -2.)]);
        assert!(v.power() == 14./4.);
    }

}