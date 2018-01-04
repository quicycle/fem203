//! fem203 is intended to be an FEM system for computation based on the theory
//! of Absolute Relativity.


/// Numeric constants for EM
pub mod consts {
    /// Permittivity of free space
    pub const EPSILON_0: f64 = 8.854187817620389e-12_f64;

    /// Permeability of free space
    pub const MU_0: f64 = 1.2566370614359173e-06_f64;

    /// Speed of light
    pub const C: f64 = 299792458.0_f64;
}


/// A simple Gaussian source pulse with offset t0 and standard deviation sigma.
pub fn gaussian(t: f64, t0: f64, sigma: f64) -> f64 {
    (-(t - t0).powi(2) / (2.0 * sigma.powi(2))).exp() / 2.0
}
