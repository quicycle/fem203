//! Base data types for FEM simulation

use super::consts::C0;
use super::paramaters::{BoundaryCondition, Medium};
use super::utils::{NTuple, gaussian};


#[derive(Debug)]
/// A source pulse for E/B
pub struct Source {
    position: NTuple,
    pulse: Vec<f64>, // The values of the source at that position over time
}


impl Source {
    fn new(
        position: NTuple,
        time_intervals: &Vec<f64>,
        source_time_offset: f64,
        source_duration: f64,
    ) -> Source {
        let pulse = time_intervals
            .iter()
            .map(|s| gaussian(*s, source_time_offset, source_duration))
            .collect();

        Source { position, pulse }
    }
}


// ----------------------------------------------------------------------------


#[derive(Debug)]
/// Control struct for running FEM simulations.
pub struct FEMBase {
    n_dimentions: u8,
    axis_lengths: NTuple,
    boundary: BoundaryCondition,
    epsilon_r: f64,
    mu_r: f64,
    source_f_max: f64,
    lambda_r_max: f64,
    lambda_r_min: f64,
    dt: f64,
    dx: f64,
    dy: f64,
    dz: f64,
    e_sources: Vec<Source>,
    b_sources: Vec<Source>,
    source_travel_time: f64,
    n_steps: u8,
    initialised: bool,
}


impl FEMBase {
    /// Create a new simulation that requires initialisation to run
    pub fn new(
        axis_lengths: NTuple,
        medium: Medium,
        boundary: BoundaryCondition,
        source_f_max: f64,
        lambda_r_max: f64,
    ) -> FEMBase {
        let dt = 0.0;
        let dx = 0.0;
        let dy = 0.0;
        let dz = 0.0;
        let n_steps = 0;
        let n_dimentions = axis_lengths.get_dims();
        let (epsilon_r, mu_r) = medium.get_epsilon_mu();
        let lambda_r_min = C0 / lambda_r_max;
        let e_sources = vec![];
        let b_sources = vec![];
        let source_travel_time = 0.0;
        let initialised = false;

        FEMBase {
            n_dimentions,
            axis_lengths,
            boundary,
            epsilon_r,
            mu_r,
            source_f_max,
            lambda_r_max,
            lambda_r_min,
            dt,
            dx,
            dy,
            dz,
            e_sources,
            b_sources,
            source_travel_time,
            n_steps,
            initialised,
        }
    }


    /// Initialise the simulation
    pub fn init(&mut self, e_source_positions: Vec<NTuple>, b_source_positions: Vec<NTuple>) {
        // TODO :: Handle Materials in the simulation. This involves looking
        //         at the max εr and μr on the grid.
        let n_max = (self.epsilon_r * self.mu_r).sqrt();

        // Step size for spatial dimensions
        let mut dk = self.lambda_r_min / n_max / self.lambda_r_max;
        if dk > 10.0 {
            dk = 10.0;
        };
        self.dx = dk;
        self.dy = dk;
        self.dz = dk;

        let l_max = self.axis_lengths.max() as f64;
        self.source_travel_time = n_max * l_max * dk / C0;
        // Ensure that the wave travels one grid cell in 2dz time intervals
        // NOTE:: Devices should not be allowed at boundaries!
        self.dt = n_max * dk / 2.0 / C0;

        let (n_steps, source_duration, source_time_offset, time_intervals) =
            self.compute_source_paramaters();

        self.n_steps = n_steps;
        // Need to prevent moving time_intervals into the first closure as
        // that stops us being able to use it again in the second.
        let intervals = &time_intervals;

        // Set sources
        self.e_sources = e_source_positions
            .iter()
            .map(move |p| {
                Source::new(*p, intervals, source_time_offset, source_duration)
            })
            .collect();

        self.b_sources = b_source_positions
            .iter()
            .map(move |p| {
                Source::new(*p, intervals, source_time_offset, source_duration)
            })
            .collect();

        self.initialised = true;
    }


    fn compute_source_paramaters(&self) -> (u8, f64, f64, Vec<f64>) {
        let source_duration = 0.5 / self.source_f_max;
        let source_time_offset = 5.0 * source_duration;

        // // NOTE:: This is an arbitrary choice of the source pulse duration
        // //        and two passes over the grid.
        let simulation_time = 2.0 * source_time_offset + 2.0 * self.source_travel_time;

        let n_steps = (simulation_time / self.dt).ceil() as u8;
        let time_intervals = (0..n_steps)
            .map(|s| s as f64 * self.dt)
            .collect();

        return (n_steps, source_duration, source_time_offset, time_intervals);
    }
}
