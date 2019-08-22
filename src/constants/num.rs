// Generic atmos 'defines'/pub constants
pub const R_IDEAL_GAS_EQUATION:               f64   = 8.31;
pub const ONE_ATMOSPHERE:                     f64   = 101.325;
pub const TCMB:                               f64   = 2.7;
pub const TCRYO:                              usize = 225;
pub const T0C:                                f64   = 273.15;
pub const T20C:                               f64   = 293.15;
// Tanks
pub const TANK_MELT_TEMPERATURE:              usize = 1_000_000;
pub const TANK_LEAK_PRESSURE:                 f64   = 30.0 * ONE_ATMOSPHERE;
pub const TANK_RUPTURE_PRESSURE:              f64   = 35.0 * ONE_ATMOSPHERE;
pub const TANK_FRAGMENT_PRESSURE:             f64   = 40.0 * ONE_ATMOSPHERE;
pub const TANK_FRAGMENT_SCALE:                f64   = 6.0  * ONE_ATMOSPHERE;
pub const TANK_MAX_RELEASE_PRESSURE:          f64   = 3.0  * ONE_ATMOSPHERE;

// 'Defines'/pub constants used in gas reactions
pub const MINIMUM_TEMP_DELTA_TO_CONSIDER:     f64   = 0.5;
pub const OXYGEN_BURN_RATE_BASE:              f64   = 1.4;
pub const PLASMA_BURN_RATE_DELTA:             usize = 9;
pub const PLASMA_MINIMUM_OXYGEN_NEEDED:       usize = 2;
pub const PLASMA_MINIMUM_OXYGEN_PLASMA_RATIO: usize = 30;
pub const FIRE_CARBON_ENERGY_RELEASED:        usize = 100_000;
pub const FIRE_HYDROGEN_ENERGY_RELEASED:      usize = 280_000;

pub const WATER_VAPOR_FREEZE:                 usize = 200;

pub const N2O_DECOMPOSITION_MIN_ENERGY:       usize = 1400;
pub const N2O_DECOMPOSITION_ENERGY_RELEASED:  usize = 200_000;

pub const NITRYL_FORMATION_ENERGY:            usize = 100_000;
pub const TRITIUM_BURN_OXY_FACTOR:            usize = 100;
pub const TRITIUM_BURN_TRIT_FACTOR:           usize = 10;
pub const TRITIUM_BURN_RADIOACTIVITY_FACTOR:  usize = 50_000;
pub const TRITIUM_MINIMUM_RADIATION_ENERGY:   f64   = 0.1;
pub const MINIMUM_TRIT_OXYBURN_ENERGY:        usize = 2_000_000;
pub const SUPER_SATURATION_THRESHOLD:         usize = 96;
pub const STIMULUM_HEAT_SCALE:                usize = 100_000;
pub const STIMULUM_FIRST_RISE:                f64   = 0.65;
pub const STIMULUM_FIRST_DROP:                f64   = 0.065;
pub const STIMULUM_SECOND_RISE:               f64   = 0.0009;
pub const STIMULUM_ABSOLUTE_DROP:             f64   = 0.00000335;
pub const REACTION_OPPRESSION_THRESHOLD:      usize = 5;
pub const NOBLIUM_FORMATION_ENERGY:           usize = 2_000_000_000;
pub const STIM_BALL_GAS_AMOUNT:               usize = 5;

pub const NOBLIUM_RESEARCH_AMOUNT:            usize = 1000;
pub const BZ_RESEARCH_SCALE:                  usize = 4;
pub const BZ_RESEARCH_MAX_AMOUNT:             usize = 400;
pub const MIASMA_RESEARCH_AMOUNT:             usize = 40;
pub const STIMULUM_RESEARCH_AMOUNT:           usize = 50;

pub const FUSION_ENERGY_THRESHOLD:            usize = 3_000_000_000;
pub const FUSION_MOLE_THRESHOLD:              usize = 250;
pub const FUSION_TRIT_CONVERSION_COEFFICIENT: f64   = 0.00000000001;
pub const INSTABILITY_GAS_POWER_FACTOR:       f64   = 0.003;
pub const FUSION_TRITIUM_MOLES_USED:          usize = 1;
pub const PLASMA_BINDING_ENERGY:              usize = 20_000_000;
pub const TOROID_VOLUME_BREAKEVEN:            usize = 1000;
pub const FUSION_TEMPERATURE_THRESHOLD:       usize = 10000;
pub const PARTICLE_CHANCE_CONSTANT:           isize = -20_000_000;
pub const FUSION_RAD_MAX:                     usize = 2000;
pub const FUSION_RAD_COEFFICIENT:             isize = -1000;
pub const FUSION_INSTABILITY_ENDOTHERMALITY:  usize = 2;

// TODO: make sure every pub const thats actually used in the code we're simulating
// is here
