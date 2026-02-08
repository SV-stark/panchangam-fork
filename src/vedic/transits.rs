//! Special Transit Analysis
//!
//! Provides calculations for:
//! - Sade Sati (Saturn's 7.5 year transit)
//! - Dhaiya (Saturn's 2.5 year transit)
//! - Panchak (Five-fold inauspicious period) [Placeholder]

use alloc::string::{String, ToString};
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
