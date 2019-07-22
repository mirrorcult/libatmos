/// Module containing all actual atmos code
pub mod atmospherics {
    /// Environmental atmos-related stuff, LINDA, hotspots, etc
    pub mod environmental { }
    /// Gas code--reactions, gases, etc
    pub mod gasmixtures {
        /// Gas type struct 
        pub mod gastype;
        /// Gas mixture struct and functions
        pub mod gasmixture;
        /// Reaction functions and react loop code
        pub mod reactions;
    }
    /// Machinery code--air pumps, pipes, canisters, etc
    pub mod machinery {
        /// Air pump code, of course!
        pub mod airpump;
        /// Canister code, of course!
        pub mod canister;
    }
}
/// Module containing all constant or static values used in the program
pub mod constants {
    /// Static gas structs - oxygen, nitrogen, etc.
    pub mod gases;
    /// References to all react() functions
    pub mod reactions;
    /// All constant number values
    pub mod num;
}