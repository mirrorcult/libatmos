extern crate snafu;
extern crate fern;
#[macro_use]
extern crate log;
extern crate chrono;

#[allow(unused_imports)] // not actually unused, but it says it is..
use log::{info};

/// Module containing all actual atmos code
pub mod atmospherics {
    /// Environmental atmos-related stuff, LINDA, hotspots, etc
    pub mod environmental {}
    /// Gas code--reactions, gases, etc
    pub mod gases;
    /// Machinery code--air pumps, pipes, canisters, etc
    pub mod machinery;
}
/// Module containing all constant or static values used in the program
pub mod constants;

/// Testing module.
#[cfg(test)]
mod tests {
    /// ENVIRONMENT!
    mod environmental {}
    /// Testing for gas-related stuff.
    mod gases {
        /// Gas-mixture methods
        mod gasmixture;
        /// Reaction methods
        mod reactions;
    }
    /// MACHINES!
    mod machinery {}
}

/// Error module.
mod errors;

/// Logging module using fern.
mod logging;
