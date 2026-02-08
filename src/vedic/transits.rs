//! Special Transit Analysis
//!
//! Provides calculations for:
//! - Sade Sati (Saturn's 7.5 year transit)
//! - Dhaiya (Saturn's 2.5 year transit)
//! - Panchak (Five-fold inauspicious period) [Placeholder]

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct SadeSatiStatus {
    /// True if Sade Sati is active (Moon sign in 12, 1, 2 from Saturn sign)
    pub is_active: bool,
    /// Phase description: "Rising", "Peak", "Setting", or "None"
    pub phase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct DhaiyaStatus {
    /// True if Dhaiya (Small Panoti) is active
    pub is_active: bool,
    /// Type: "Ardhastama" (4th) or "Ashtama" (8th) or "None"
    pub type_name: String,
}

/// Calculate Sade Sati status
///
/// Sade Sati occurs when Saturn transits the 12th, 1st, and 2nd houses from the Natal Moon.
///
/// # Arguments
/// * `moon_sign` - Natal Moon Sign (1-12)
/// * `saturn_sign` - Current Transit Saturn Sign (1-12)
#[wasm_bindgen]
pub fn check_sade_sati(moon_sign: u8, saturn_sign: u8) -> SadeSatiStatus {
    // Normalize to 0-11
    let m = (moon_sign - 1) % 12; // 0..11
    let s = (saturn_sign - 1) % 12; // 0..11

    // Calculate position of Saturn relative to Moon
    // Relative = (Saturn - Moon + 12) % 12 + 1
    // e.g. Moon=1 (Ari), Sat=12 (Pis). (11 - 0 + 12)%12 + 1 = 12. -> 12th House.

    let rel_pos = (s as i32 - m as i32 + 12) % 12 + 1;

    match rel_pos {
        12 => SadeSatiStatus {
            is_active: true,
            phase: "Rising".to_string(),
        },
        1 => SadeSatiStatus {
            is_active: true,
            phase: "Peak".to_string(),
        },
        2 => SadeSatiStatus {
            is_active: true,
            phase: "Setting".to_string(),
        },
        _ => SadeSatiStatus {
            is_active: false,
            phase: "None".to_string(),
        },
    }
}

/// Calculate Dhaiya (Small Panoti) status
///
/// Dhaiya occurs when Saturn transits the 4th (Ardhastama) or 8th (Ashtama) house from Natal Moon.
#[wasm_bindgen]
pub fn check_dhaiya(moon_sign: u8, saturn_sign: u8) -> DhaiyaStatus {
    let m = (moon_sign - 1) % 12;
    let s = (saturn_sign - 1) % 12;
    let rel_pos = (s as i32 - m as i32 + 12) % 12 + 1;

    match rel_pos {
        4 => DhaiyaStatus {
            is_active: true,
            type_name: "Ardhastama".to_string(), // 4th House
        },
        8 => DhaiyaStatus {
            is_active: true,
            type_name: "Ashtama".to_string(), // 8th House
        },
        _ => DhaiyaStatus {
            is_active: false,
            type_name: "None".to_string(),
        },
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct VedhaResult {
    pub transit_planet: i32,
    pub is_favorable: bool,
    pub is_obstructed: bool,
    pub obstruction_planet: Option<i32>,
    pub house_from_moon: i32,
}

/// Calculate Gochara Vedha (Transit Obstruction)
///
/// # Arguments
/// * `planet_id` - ID of the planet transiting (0=Sun..6=Saturn)
/// * `house_from_moon` - House the planet is in relative to Natal Moon (1..12)
/// * `other_transits` - Map of other planets and their houses from Natal Moon (planet_id -> house)
#[wasm_bindgen]
pub fn calculate_vedha(
    planet_id: i32,
    house_from_moon: i32,
    other_transits: &JsValue,
) -> VedhaResult {
    let transits: BTreeMap<i32, i32> =
        serde_wasm_bindgen::from_value(other_transits.clone()).unwrap_or_default();

    // Favorable houses and their Vedha points
    // Format: (Favorable House, Vedha House)
    let (favorable_vec, is_favorable) = match planet_id {
        0 => (
            vec![(3, 9), (6, 12), (10, 4), (11, 5)],
            [3, 6, 10, 11].contains(&house_from_moon),
        ),
        1 => (
            vec![(1, 5), (3, 9), (6, 12), (7, 2), (10, 4), (11, 8)],
            [1, 3, 6, 7, 10, 11].contains(&house_from_moon),
        ), // Note: some sources say 6->12
        2 => (
            vec![(3, 12), (6, 9), (11, 5)],
            [3, 6, 11].contains(&house_from_moon),
        ),
        3 => (
            vec![(2, 5), (4, 3), (6, 9), (8, 1), (10, 8), (11, 12)],
            [2, 4, 6, 8, 10, 11].contains(&house_from_moon),
        ),
        4 => (
            vec![(2, 12), (5, 4), (7, 3), (9, 10), (11, 8)],
            [2, 5, 7, 9, 11].contains(&house_from_moon),
        ),
        5 => (
            vec![
                (1, 8),
                (2, 7),
                (3, 1),
                (4, 10),
                (5, 9),
                (8, 5),
                (9, 11),
                (11, 3),
                (12, 6),
            ],
            [1, 2, 3, 4, 5, 8, 9, 11, 12].contains(&house_from_moon),
        ),
        6 => (
            vec![(3, 12), (6, 9), (11, 5)],
            [3, 6, 11].contains(&house_from_moon),
        ),
        _ => (vec![], false),
    };

    let mut is_obstructed = false;
    let mut obstruction_planet = None;

    if is_favorable {
        // Find corresponding Vedha house
        if let Some(&(_, vedha_house)) = favorable_vec
            .iter()
            .find(|&&(fav, _)| fav == house_from_moon)
        {
            // Check if any planet (except specific exceptions) is in that Vedha house
            for (&other_id, &other_house) in transits.iter() {
                if other_id == planet_id {
                    continue;
                }
                if other_house == vedha_house as i32 {
                    // Exceptions:
                    // Father-Son: Sun and Saturn don't obstruct each other
                    if (planet_id == 0 && other_id == 6) || (planet_id == 6 && other_id == 0) {
                        continue;
                    }
                    // Mercury and Moon don't obstruct each other
                    if (planet_id == 3 && other_id == 1) || (planet_id == 1 && other_id == 3) {
                        continue;
                    }

                    is_obstructed = true;
                    obstruction_planet = Some(other_id);
                    break;
                }
            }
        }
    }

    VedhaResult {
        transit_planet: planet_id,
        is_favorable,
        is_obstructed,
        obstruction_planet,
        house_from_moon,
    }
}
