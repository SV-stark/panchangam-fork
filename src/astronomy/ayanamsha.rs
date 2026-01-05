//! Ayanamsha calculations using Swiss Ephemeris

use wasm_bindgen::prelude::*;
use crate::swe_bindings;

/// Ayanamsha modes
#[derive(Debug, Clone, Copy, PartialEq)]
#[wasm_bindgen]
pub enum AyanamshaMode {
    /// Lahiri (Chitrapaksha)
    Lahiri = 0,
    /// Raman
    Raman = 1,
    /// Krishnamurti (KP)
    Krishnamurti = 2,
    /// True Chitrapaksha
    TrueChitrapaksha = 3,
}

impl AyanamshaMode {
    pub fn to_swe_mode(&self) -> i32 {
        match self {
            AyanamshaMode::Lahiri => swe_bindings::SE_SIDM_LAHIRI as i32,
            AyanamshaMode::Raman => swe_bindings::SE_SIDM_RAMAN as i32,
            AyanamshaMode::Krishnamurti => swe_bindings::SE_SIDM_KRISHNAMURTI as i32,
            AyanamshaMode::TrueChitrapaksha => swe_bindings::SE_SIDM_TRUE_CITRA as i32,
        }
    }
}

/// Get Ayanamsha value for a given mode and Julian Day
#[wasm_bindgen]
pub fn get_ayanamsha(mode: AyanamshaMode, jd: f64) -> f64 {
    unsafe {
        swe_bindings::swe_set_sid_mode(mode.to_swe_mode(), 0.0, 0.0);
        swe_bindings::swe_get_ayanamsa_ut(jd)
    }
}

/// Convert tropical (sayana) longitude to sidereal (nirayana)
/// Formula: L_nirayana = L_sayana - Ayanamsha(t)
pub fn tropical_to_sidereal(tropical_long: f64, ayanamsha: f64) -> f64 {
    let mut sidereal = tropical_long - ayanamsha;
    if sidereal < 0.0 {
        sidereal += 360.0;
    }
    sidereal % 360.0
}
