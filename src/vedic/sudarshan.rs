//! Sudarshan Chakra Analysis
//!
//! Provides triple-perspective house analysis from Lagna, Moon, and Sun.

use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct SudarshanHouseResult {
    pub house_number: i32,
    pub lagna_sign: u8,
    pub moon_sign: u8,
    pub sun_sign: u8,
    /// Overall score or strength
    pub strength_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct SudarshanChakra {
    pub houses: Vec<SudarshanHouseResult>,
}

/// Calculate Sudarshan Chakra
///
/// # Arguments
/// * `lagna_sign` - Natal Lagna Sign (1-12)
/// * `moon_sign` - Natal Moon Sign (1-12)
/// * `sun_sign` - Natal Sun Sign (1-12)
#[wasm_bindgen]
pub fn calculate_sudarshan_chakra(lagna_sign: u8, moon_sign: u8, sun_sign: u8) -> SudarshanChakra {
    let mut houses = vec![];

    for h in 1..=12 {
        // Calculate sign for each house from the three reference points
        let h_lagna = ((lagna_sign as i32 - 1 + h - 1) % 12 + 1) as u8;
        let h_moon = ((moon_sign as i32 - 1 + h - 1) % 12 + 1) as u8;
        let h_sun = ((sun_sign as i32 - 1 + h - 1) % 12 + 1) as u8;

        // Simple strength score: 1.0 per "favorable" sign (placeholder logic)
        // In real Sudarshan, you'd look at planet positions in these signs.
        let strength_score = 0.0;

        houses.push(SudarshanHouseResult {
            house_number: h,
            lagna_sign: h_lagna,
            moon_sign: h_moon,
            sun_sign: h_sun,
            strength_score,
        });
    }

    SudarshanChakra { houses }
}
