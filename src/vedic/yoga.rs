//! Yoga calculations
//! Yoga = (Moon longitude + Sun longitude) / 13°20'

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use alloc::string::{String, ToString};
use crate::astronomy::planets::{sun_longitude, moon_longitude};
use crate::astronomy::ayanamsha::{AyanamshaMode, get_ayanamsha, tropical_to_sidereal};

/// Yoga names (27 yogas)
pub const YOGA_NAMES: [&str; 27] = [
    "Vishkumbha", "Priti", "Ayushman", "Saubhagya", "Shobhana",
    "Atiganda", "Sukarman", "Dhriti", "Shula", "Ganda",
    "Vriddhi", "Dhruva", "Vyaghata", "Harshana", "Vajra",
    "Siddhi", "Vyatipata", "Variyana", "Parigha", "Shiva",
    "Siddha", "Sadhya", "Shubha", "Shukla", "Brahma",
    "Indra", "Vaidhriti",
];

/// Yoga information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct YogaInfo {
    /// Yoga index (1-27)
    pub index: u8,
    /// Yoga name
    pub name: String,
}

/// Calculate Yoga for a given Julian Day
/// Formula: Yoga = floor((Moon_long + Sun_long) / 13.333) + 1
#[wasm_bindgen]
pub fn calculate_yoga(jd: f64, ayanamsha_mode: AyanamshaMode) -> YogaInfo {
    let sun_tropical = sun_longitude(jd);
    let moon_tropical = moon_longitude(jd);
    let ayanamsha = get_ayanamsha(ayanamsha_mode, jd);
    
    let sun_sidereal = tropical_to_sidereal(sun_tropical, ayanamsha);
    let moon_sidereal = tropical_to_sidereal(moon_tropical, ayanamsha);
    
    // Sum of longitudes
    let mut sum = sun_sidereal + moon_sidereal;
    if sum >= 360.0 {
        sum -= 360.0;
    }
    
    // Each Yoga = 360/27 = 13.333... degrees
    let yoga_span = 360.0 / 27.0;
    let yoga_index = (sum / yoga_span).floor() as u8 + 1;
    
    YogaInfo {
        index: yoga_index,
        name: YOGA_NAMES[(yoga_index - 1) as usize].to_string(),
    }
}
