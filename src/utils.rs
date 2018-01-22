//! Utility functions and data types.


#[derive(Debug, Clone, Copy)]
/// An n-tuple of 1-4 dimensions.
pub enum NTuple {
    X(u8),
    XY(u8, u8),
    XYZ(u8, u8, u8),
    TXYZ(u8, u8, u8, u8),
}


impl NTuple {
    /// Returns the dimentionality of this NTuple.
    pub fn get_dims(&self) -> u8 {
        match *self {
            NTuple::X(..) => 1,
            NTuple::XY(..) => 2,
            NTuple::XYZ(..) => 3,
            NTuple::TXYZ(..) => 4,
        }
    }

    /// Return the largest value within the NTuple.
    pub fn max(&self) -> u8 {
        match *self {
            NTuple::X(x) => x,
            NTuple::XY(x, y) => *vec![x, y].iter().max().unwrap(),
            NTuple::XYZ(x, y, z) => *vec![x, y, z].iter().max().unwrap(),
            NTuple::TXYZ(t, x, y, z) => *vec![t, x, y, z].iter().max().unwrap(),
        }
    }
}


// ----------------------------------------------------------------------------


/// A simple Gaussian source pulse with offset t0 and standard deviation sigma.
pub fn gaussian(t: f64, t0: f64, sigma: f64) -> f64 {
    (-(t - t0).powi(2) / (2.0 * sigma.powi(2))).exp() / 2.0
}
