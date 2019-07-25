// Generic atmos 'defines'/constants
const R_IDEAL_GAS_EQUATION:               f64   = 8.31;
const ONE_ATMOSPHERE:                     f64   = 101.325;
const TCMB:                               f64   = 2.7;
const TCRYO:                              usize = 225;
const T0C:                                f64   = 273.15;
const T20C:                               f64   = 293.15;
// Tanks
const TANK_MELT_TEMPERATURE:              usize = 1_000_000;
const TANK_LEAK_PRESSURE:                 f64   = 30.0 * ONE_ATMOSPHERE;
const TANK_RUPTURE_PRESSURE:              f64   = 35.0 * ONE_ATMOSPHERE;
const TANK_FRAGMENT_PRESSURE:             f64   = 40.0 * ONE_ATMOSPHERE;
const TANK_FRAGMENT_SCALE:                f64   = 6.0  * ONE_ATMOSPHERE;
const TANK_MAX_RELEASE_PRESSURE:          f64   = 3.0  * ONE_ATMOSPHERE;

// 'Defines'/constants used in gas reactions 
const OXYGEN_BURN_RATE_BASE:              f64   = 1.4;
const PLASMA_BURN_RATE_DELTA:             usize = 9;
const PLASMA_MINIMUM_OXYGEN_NEEDED:       usize = 2;
const PLASMA_MINIMUM_OXYGEN_PLASMA_RATIO: usize = 30;
const FIRE_CARBON_ENERGY_RELEASED:        usize = 100_000;
const FIRE_HYDROGEN_ENERGY_RELEASED:      usize = 280_000;

const WATER_VAPOR_FREEZE:                 usize = 200;

const N2O_DECOMPOSITION_MIN_ENERGY:       usize = 1400;
const N2O_DECOMPOSITION_ENERGY_RELEASED:  usize = 200_000;

const NITRYL_FORMATION_ENERGY:            usize = 100_000;
const TRITIUM_BURN_OXY_FACTOR:            usize = 100;
const TRITIUM_BURN_TRIT_FACTOR:           usize = 10;
const TRITIUM_BURN_RADIOACTIVITY_FACTOR:  usize = 50_000;
const TRITIUM_MINIMUM_RADIATION_ENERGY:   f64   = 0.1;
const MINIMUM_TRIT_OXYBURN_ENERGY:        usize = 2_000_000;
const SUPER_SATURATION_THRESHOLD:         usize = 96;
const STIMULUM_HEAT_SCALE:                usize = 100_000;
const STIMULUM_FIRST_RISE:                f64   = 0.65;
const STIMULUM_FIRST_DROP:                f64   = 0.065; 
const STIMULUM_SECOND_RISE:               f64   = 0.0009;
const STIMULUM_ABSOLUTE_DROP:             f64   = 0.00000335;
const REACTION_OPPRESSION_THRESHOLD:      usize = 5;
const NOBLIUM_FORMATION_ENERGY:           usize = 2_000_000_000;
const STIM_BALL_GAS_AMOUNT:               usize = 5;

const NOBLIUM_RESEARCH_AMOUNT:            usize = 1000;
const BZ_RESEARCH_SCALE:                  usize = 4;
const BZ_RESEARCH_MAX_AMOUNT:             usize = 400;
const MIASMA_RESEARCH_AMOUNT:             usize = 40;
const STIMULUM_RESEARCH_AMOUNT:           usize = 50;

const FUSION_ENERGY_THRESHOLD:            usize = 3_000_000_000;
const FUSION_MOLE_THRESHOLD:              usize = 250;
const FUSION_TRIT_CONVERSION_COEFFICIENT: f64   = 0.00000000001;
const INSTABILITY_GAS_POWER_FACTOR:       f64   = 0.003;
const FUSION_TRITIUM_MOLES_USED:          usize = 1;
const PLASMA_BINDING_ENERGY:              usize = 20_000_000;
const TOROID_VOLUME_BREAKEVEN:            usize = 1000;
const FUSION_TEMPERATURE_THRESHOLD:       usize = 10000;
const PARTICLE_CHANCE_CONSTANT:           isize = -20_000_000;
const FUSION_RAD_MAX:                     usize = 2000;
const FUSION_RAD_COEFFICIENT:             isize = -1000;
const FUSION_INSTABILITY_ENDOTHERMALITY:  usize = 2;

// TODO: make sure every const thats actually used in the code we're simulating
// is here