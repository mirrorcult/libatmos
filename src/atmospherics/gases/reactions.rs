use std::collections::HashMap;

use crate::{
    atmospherics::gases::{
        GasType,
        GasMixture,
    },
};

// Base reaction struct

struct GasReaction {
    react: Box<dyn Fn(GasMixture)>,
    min_reqs: HashMap<&'static str, f64>, // between gas id or TEMP/ENER & mole count or temperature/therm energy
    priority: usize,
    name: &'static str,
    id: &'static str,
}

impl GasReaction {
    // TODO: Maybe remove this its not that useful
    pub fn new(name: &'static str, id: &'static str, react: Box<dyn Fn(GasMixture)>, min_reqs: HashMap<&'static str, f64>, priority: usize) -> GasReaction {
        GasReaction {
            react,
            min_reqs,
            priority,
            name,
            id,
        }
    }
}

// Actual reaction funcs.