//! Planetary Combustion Analysis
//!
//! Planets become combust when too close to the Sun.
//! Based on Parashari combustion limits.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Combustion analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct CombustionInfo {
    /// Whether the planet is combust
    pub is_combust: bool,
    /// Angular distance from Sun (degrees)
    pub distance: f64,
    /// Combustion orb limit for this planet (degrees)
    pub orb: f64,
    /// Severity ratio (0.0 = not close, 1.0 = exact conjunction)
    pub severity: f64,
    /// Planet is retrograde (retrograde planets have wider combustion orb)
    pub is_retrograde: bool,
}

/// Combustion orb limits per planet (degrees from Sun)
/// planet_id: 0=Sun, 1=Moon, 2=Mercury, 3=Venus, 4=Mars, 5=Jupiter, 6=Saturn
fn get_combustion_orb(planet_id: i32, is_retrograde: bool) -> f64 {
    // Standard Parashara limits:
    // Moon: 12°
    // Mars: 17° (direct), 19° (retrograde)
    // Mercury: 14° (direct), 12° (retrograde)
    // Jupiter: 11°
    // Venus: 10° (direct), 8° (retrograde)
    // Saturn: 15°
    match planet_id {
        1 => 12.0,  // Moon
        2 => if is_retrograde { 12.0 } else { 14.0 }, // Mercury
        3 => if is_retrograde { 8.0 } else { 10.0 },  // Venus
        4 => if is_retrograde { 19.0 } else { 17.0 }, // Mars
        5 => 11.0,  // Jupiter
        6 => 15.0,  // Saturn
        _ => 0.0,   // Sun itself or unknown
    }
}

fn angular_distance(a: f64, b: f64) -> f64 {
    let diff = (a - b).abs() % 360.0;
    if diff > 180.0 { 360.0 - diff } else { diff }
}

/// Calculate combustion information for a planet
/// Arguments:
/// - sun_longitude: Sun's sidereal longitude
/// - planet_longitude: Planet's sidereal longitude
/// - planet_id: 0=Sun..6=Saturn
/// - is_retrograde: whether the planet is retrograde
pub fn calculate_combustion_info(
    sun_longitude: f64,
    planet_longitude: f64,
    planet_id: i32,
    is_retrograde: bool,
) -> CombustionInfo {
    if planet_id == 0 {
        // Sun can't combust itself
        return CombustionInfo {
            is_combust: false,
            distance: 0.0,
            orb: 0.0,
            severity: 0.0,
            is_retrograde: false,
        };
    }

    let distance = angular_distance(sun_longitude, planet_longitude);
    let orb = get_combustion_orb(planet_id, is_retrograde);
    let is_combust = distance < orb;
    let severity = if orb > 0.0 {
        (1.0 - (distance / orb)).max(0.0)
    } else {
        0.0
    };

    CombustionInfo {
        is_combust,
        distance,
        orb,
        severity,
        is_retrograde,
    }
}
