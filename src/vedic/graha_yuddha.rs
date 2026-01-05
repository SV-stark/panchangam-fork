use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use alloc::vec::Vec;
use alloc::string::String;
use crate::astronomy::planets::{self, PlanetId};
use crate::astronomy::ayanamsha::{self, AyanamshaMode};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct WarDetails {
    pub planet1_id: u8,
    #[wasm_bindgen(getter_with_clone)]
    pub planet1_name: String,
    pub planet1_long: f64,
    pub planet1_mag: f64,
    
    pub planet2_id: u8,
    #[wasm_bindgen(getter_with_clone)]
    pub planet2_name: String,
    pub planet2_long: f64,
    pub planet2_mag: f64,
    
    pub longitude_diff: f64,
    pub winner_id: u8, // The ID of the planet that is brighter
}

/// Check for Planetary War (Graha Yuddha)
/// Occurs when two Tara Grahas (Mars, Mercury, Jupiter, Venus, Saturn) 
/// are within 1 degree of each other.
#[wasm_bindgen]
pub fn check_graha_yuddha(jd: f64, ayan_mode: i32) -> JsValue {
    let mode = match ayan_mode {
        1 => AyanamshaMode::Lahiri,
        3 => AyanamshaMode::Raman,
        5 => AyanamshaMode::Krishnamurti,
        27 => AyanamshaMode::TrueCitra,
        _ => AyanamshaMode::Lahiri,
    };
    
    let ayanamsha_val = ayanamsha::get_ayanamsha(mode, jd);
    
    let candidates = [
        (PlanetId::Mercury, "Mercury", 2),
        (PlanetId::Venus,   "Venus",   3),
        (PlanetId::Mars,    "Mars",    4),
        (PlanetId::Jupiter, "Jupiter", 5),
        (PlanetId::Saturn,  "Saturn",  6),
    ];
    
    let mut positions = Vec::new();
    
    // 1. Calculate Positions
    for (pid, name, id_num) in candidates.iter() {
        let pos = planets::get_planet_position_sidereal(*pid, jd, ayanamsha_val);
        let mag = planets::get_planet_magnitude(*pid, jd);
        positions.push((*id_num, *name, pos.longitude, mag));
    }
    
    let mut wars = Vec::new();
    
    // 2. Pairwise Check
    for i in 0..positions.len() {
        for j in (i+1)..positions.len() {
            let (id1, name1, l1, mag1) = positions[i];
            let (id2, name2, l2, mag2) = positions[j];
            
            let mut diff = (l1 - l2).abs();
            if diff > 180.0 { diff = 360.0 - diff; }
            
            if diff < 1.0 {
                // WAR DETECTED
                // Winner is brighter (lower magnitude)
                let winner = if mag1 < mag2 { id1 } else { id2 };
                
                wars.push(WarDetails {
                    planet1_id: id1,
                    planet1_name: String::from(name1),
                    planet1_long: l1,
                    planet1_mag: mag1,
                    
                    planet2_id: id2,
                    planet2_name: String::from(name2),
                    planet2_long: l2,
                    planet2_mag: mag2,
                    
                    longitude_diff: diff,
                    winner_id: winner,
                });
            }
        }
    }
    
    serde_wasm_bindgen::to_value(&wars).unwrap()
}
