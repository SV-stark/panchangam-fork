//! Moon Phase Calculations
//!
//! Provides moon phase details including:
//! - Illumination percentage
//! - Lunar age (days since new moon)
//! - Phase name (Waxing Crescent, Full Moon, etc.)

use alloc::string::{String, ToString};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Moon phase details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct MoonPhaseDetails {
    /// Illumination percentage (0-100)
    pub illumination_pct: f64,
    /// Lunar age in days (0-29.5)
    pub lunar_age_days: f64,
    /// Phase name
    pub phase_name: String,
    /// Elongation angle (Sun-Moon angle, 0-360)
    pub elongation: f64,
    /// True if waxing (Moon moving away from Sun)
    pub is_waxing: bool,
}

/// Calculate Moon phase details
/// Arguments:
/// - sun_long: Sun longitude (0-360)
/// - moon_long: Moon longitude (0-360)
pub fn calculate_moon_phase_details(sun_long: f64, moon_long: f64) -> MoonPhaseDetails {
    // Elongation = Moon longitude - Sun longitude (0-360)
    let elongation = (moon_long - sun_long).rem_euclid(360.0);

    // Illumination: (1 - cos(elongation)) / 2 * 100
    let rad = elongation * core::f64::consts::PI / 180.0;
    let illumination_pct = ((1.0 - rad.cos()) / 2.0) * 100.0;

    // Lunar age: elongation / 360 * 29.53 days
    let lunar_age_days = elongation / 360.0 * 29.53;

    // Is waxing: elongation 0-180 (moving away from Sun)
    let is_waxing = elongation < 180.0;

    // Phase name
    let phase_name = match elongation as u32 {
        0..=11 => "New Moon",
        12..=56 => "Waxing Crescent",
        57..=101 => "First Quarter",
        102..=168 => "Waxing Gibbous",
        169..=191 => "Full Moon",
        192..=236 => "Waning Gibbous",
        237..=281 => "Last Quarter",
        282..=348 => "Waning Crescent",
        _ => "New Moon",
    };

    MoonPhaseDetails {
        illumination_pct,
        lunar_age_days,
        phase_name: phase_name.to_string(),
        elongation,
        is_waxing,
    }
}
