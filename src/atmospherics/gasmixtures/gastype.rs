/// Struct used to identify and organize different gases by name.
pub struct GasType {
    /// Short identifier
    pub id: String,
    /// More formal, user-friendly name
    pub name: String,
    /// Amount of energy required to heat a gas up by 1 deg. celsius.
    /// Higher => harder to cool/heat.
    /// Lower => easier to cool/heat
    pub specific_heat: u32,
    /// Multiplier for how much a gas accelerates a fusion reaction
    pub fusion_power: i32
}