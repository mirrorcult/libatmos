use std::fmt;

/// Struct used to identify and organize different gases by name.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct GasType {
    /// Short identifier
    pub id: &'static str,
    /// More formal, user-friendly name
    pub name: &'static str,
    /// Amount of energy required to heat a gas up by 1 deg. celsius.
    /// Higher => harder to cool/heat.
    /// Lower => easier to cool/heat
    pub specific_heat: usize,
    /// Multiplier for how much a gas accelerates a fusion reaction
    pub fusion_power: isize
}

impl fmt::Display for GasType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.id) // i.e. "o2 (Oxygen)"
    }
}