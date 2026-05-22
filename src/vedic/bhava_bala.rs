//! Bhava Bala (House Strength) Calculations
//! Based on traditional Vedic astrology rules incorporating House Lord Strength (Adhipati Bala),
//! House Directional Strength (Dig Bala), and Aspect/Occupancy Strength (Drishti Bala).

use crate::vedic::dignity::{calculate_dignity, Dignity};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// Result for an individual house
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct BhavaStrength {
    pub house_number: u8,
    pub sign: u8,
    pub sign_name: String,
    pub lord: String,
    pub adhipati_bala: f64,
    pub dig_bala: f64,
    pub drishti_bala: f64,
    pub total_bala: f64,
}

/// Bhava Bala results for all 12 houses
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct BhavaBalaResult {
    #[wasm_bindgen(getter_with_clone)]
    pub houses: Vec<BhavaStrength>,
}

fn get_sign_lord(sign: u8) -> usize {
    match sign {
        1 | 8 => 4,    // Mars (ruler of Aries & Scorpio)
        2 | 7 => 3,    // Venus (ruler of Taurus & Libra)
        3 | 6 => 2,    // Mercury (ruler of Gemini & Virgo)
        4 => 1,        // Moon (ruler of Cancer)
        5 => 0,        // Sun (ruler of Leo)
        9 | 12 => 5,   // Jupiter (ruler of Sagittarius & Pisces)
        10 | 11 => 6,  // Saturn (ruler of Capricorn & Aquarius)
        _ => 0,
    }
}

fn get_sign_name(sign: u8) -> String {
    let names = [
        "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
        "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"
    ];
    if sign >= 1 && sign <= 12 {
        names[sign as usize - 1].to_string()
    } else {
        "Unknown".to_string()
    }
}

/// Calculate Bhava Bala for all 12 houses from the Lagna
pub fn calculate_bhava_bala(
    lagna_sign: u8,
    planet_longs: &[f64],
    planet_shadbalas: &[f64],
) -> BhavaBalaResult {
    let planet_names = ["Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn"];
    let mut houses = Vec::new();

    for h in 1..=12 {
        // Sign index of this house (1-12)
        let house_sign = ((lagna_sign as i32 + h as i32 - 2) % 12 + 1) as u8;

        // 1. Lord Strength (Adhipati Bala)
        let lord_pid = get_sign_lord(house_sign);
        let lord_name = planet_names[lord_pid];

        let adhipati_bala = if planet_shadbalas.len() > lord_pid {
            planet_shadbalas[lord_pid]
        } else {
            // Default proxy if full Shadbala is not run
            let lord_long = planet_longs[lord_pid];
            let dig = calculate_dignity(lord_name, lord_long);
            match dig {
                Dignity::Exalted => 500.0,
                Dignity::Moolatrikona => 450.0,
                Dignity::OwnSign => 400.0,
                Dignity::GreatFriend => 350.0,
                Dignity::Friend => 300.0,
                Dignity::Neutral => 250.0,
                Dignity::Enemy => 200.0,
                Dignity::GreatEnemy => 150.0,
                Dignity::Debilitated => 100.0,
            }
        };

        // 2. Dig Bala (Directional Strength)
        // Kendra (1, 4, 7, 10): 60, Panapara (2, 5, 8, 11): 30, Apoklima (3, 6, 9, 12): 15
        let dig_bala = match h {
            1 | 4 | 7 | 10 => 60.0,
            2 | 5 | 8 | 11 => 30.0,
            _ => 15.0,
        };

        // 3. Drishti & Occupancy Bala (Aspects & Placement Strength)
        let mut drishti_bala = 0.0;

        for p in 0..7 {
            let p_long = planet_longs[p];
            let p_sign = ((p_long / 30.0).floor() as i32 % 12 + 1) as u8;
            let is_benefic = p == 1 || p == 2 || p == 3 || p == 5; // Moon, Mercury, Venus, Jupiter

            // Is the planet in the house?
            if p_sign == house_sign {
                if is_benefic {
                    drishti_bala += 30.0;
                } else {
                    drishti_bala -= 15.0;
                }
            }

            // Aspect rules (Graha Drishti)
            let dist = (house_sign as i32 - p_sign as i32 + 12) % 12; // Distance in signs (0-11)
            let is_aspecting = match p {
                4 => dist == 3 || dist == 6 || dist == 7,   // Mars (4, 7, 8 aspects)
                5 => dist == 4 || dist == 6 || dist == 8,   // Jupiter (5, 7, 9 aspects)
                6 => dist == 2 || dist == 6 || dist == 9,   // Saturn (3, 7, 10 aspects)
                _ => dist == 6,                             // Sun, Moon, Mercury, Venus (7 aspect)
            };

            if is_aspecting {
                if is_benefic {
                    drishti_bala += 30.0;
                } else {
                    drishti_bala -= 15.0;
                }
            }
        }

        // Add special bonus if the lord aspects its own house
        let lord_long = planet_longs[lord_pid];
        let lord_sign = ((lord_long / 30.0).floor() as i32 % 12 + 1) as u8;
        let lord_dist = (house_sign as i32 - lord_sign as i32 + 12) % 12;
        let lord_aspects = match lord_pid {
            4 => lord_dist == 3 || lord_dist == 6 || lord_dist == 7,
            5 => lord_dist == 4 || lord_dist == 6 || lord_dist == 8,
            6 => lord_dist == 2 || lord_dist == 6 || lord_dist == 9,
            _ => lord_dist == 6,
        };
        if lord_aspects || lord_sign == house_sign {
            drishti_bala += 30.0; // Strong bonus for lord aspect/placement
        }

        let total_bala = adhipati_bala + dig_bala + drishti_bala;

        houses.push(BhavaStrength {
            house_number: h,
            sign: house_sign,
            sign_name: get_sign_name(house_sign),
            lord: lord_name.to_string(),
            adhipati_bala,
            dig_bala,
            drishti_bala,
            total_bala,
        });
    }

    BhavaBalaResult { houses }
}
