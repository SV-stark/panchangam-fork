//! Vara (Weekday) calculations
//! In Vedic time, the day begins at sunrise, not midnight

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::ToString;

/// Weekday names (Sanskrit)
pub const VARA_NAMES: [&str; 7] = [
    "Ravivara",    // Sunday
    "Somavara",    // Monday
    "Mangalavara", // Tuesday
    "Budhavara",   // Wednesday
    "Guruvara",    // Thursday
    "Shukravara",  // Friday
    "Shanivara",   // Saturday
];

/// Weekday planetary lords
pub const VARA_LORDS: [&str; 7] = [
    "Sun", "Moon", "Mars", "Mercury", "Jupiter", "Venus", "Saturn"
];

/// Vara (weekday) information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct VaraInfo {
    /// Day index (0=Sunday, 6=Saturday)
    pub index: u8,
    /// Sanskrit weekday name
    pub name: alloc::string::String,
    /// Planetary lord
    pub lord: alloc::string::String,
}

/// Calculate Vara (weekday) for a given Julian Day
/// Note: This returns the astronomical weekday. 
/// For Vedic Vara, compare with sunrise time.
#[wasm_bindgen]
pub fn calculate_vara(jd: f64) -> VaraInfo {
    // Julian Day 0 was Monday
    // JD % 7: 0=Monday, 1=Tuesday, ..., 6=Sunday
    // We want: 0=Sunday, 1=Monday, etc.
    let day_num = ((jd + 1.5).floor() as i64 % 7) as u8;
    
    VaraInfo {
        index: day_num,
        name: VARA_NAMES[day_num as usize].to_string(),
        lord: VARA_LORDS[day_num as usize].to_string(),
    }
}

/// Calculate Vedic Vara considering sunrise
/// If current time is before sunrise, return previous day's Vara
pub fn calculate_vedic_vara(jd: f64, sunrise_jd: f64) -> VaraInfo {
    let effective_jd = if jd < sunrise_jd {
        jd - 1.0 // Previous day
    } else {
        jd
    };
    calculate_vara(effective_jd)
}
