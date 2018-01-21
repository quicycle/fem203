//! Data types for specifying simulation paramaters.

use super::consts::{EPSILON_0, MU_0};

#[derive(Debug)]
/// Markers for setting boundary conditions for the simulation.
/// Passes to the FEMBase `get_boundary()` method to fetch values at
/// simulation boundaries.
pub enum BoundaryCondition {
    /// Fix boundaries at 0.
    Drichlet,
    /// Take boundary values from opposide end of the simulation.
    Periodic,
    /// Perfectly reflect at the boundary.
    Perfect,
}


// ----------------------------------------------------------------------------


#[derive(Debug)]
/// Used to define the permeability and permittivity of the simulation.
pub enum Medium {
    /// εr and μr are both 1.
    Air,
    /// εr and μr are equal to ε0 and μ0 respectively.
    FreeSpace,
}


impl Medium {
    /// Get the values for εr and μr for the given simulation medium.
    pub fn get_epsilon_mu(&self) -> (f64, f64) {
        match *self {
            Medium::Air => (0.0, 0.0),
            Medium::FreeSpace => (EPSILON_0, MU_0),
        }
    }
}
