//! Ashtakoota Milan (36 Gunas) and Manglik Dosha compatibility engines

use crate::vedic::dignity::{calculate_dignity, Dignity};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// Ashtakoota Milan matching points
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct AshtakootaResult {
    pub varna_score: f64,
    pub vashya_score: f64,
    pub tara_score: f64,
    pub yoni_score: f64,
    pub maitri_score: f64,
    pub gana_score: f64,
    pub bhakoot_score: f64,
    pub nadi_score: f64,
    pub total_score: f64,
    pub has_nadi_dosha: bool,
    pub has_bhakoot_dosha: bool,
}

/// Manglik Dosha analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct ManglikResult {
    pub is_manglik: bool,
    pub severity: f64, // 0.0 to 1.0
    pub placements: Vec<String>,
    pub cancellations: Vec<String>,
}

/// Consolidated compatibility report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct CompatibilityReport {
    #[wasm_bindgen(getter_with_clone)]
    pub guna_milan: AshtakootaResult,
    #[wasm_bindgen(getter_with_clone)]
    pub boy_manglik: ManglikResult,
    #[wasm_bindgen(getter_with_clone)]
    pub girl_manglik: ManglikResult,
    pub overall_compatibility_pct: f64,
}

// Animal mappings for Yoni Guna
// 0: Horse, 1: Elephant, 2: Sheep, 3: Serpent, 4: Dog, 5: Cat, 6: Rat, 7: Cow,
// 8: Buffalo, 9: Tiger, 10: Hare, 11: Monkey, 12: Lion, 13: Mongoose
const NAKSHATRA_YONI: [usize; 27] = [
    0, 0, 1, 1, 2, 3, 3, 4, 5, // Ashwini to Ashlesha
    6, 6, 7, 7, 9, 9, 10, 10, 11, // Magha to Jyeshtha
    13, 11, 11, 8, 8, 12, 12, 1, 2 // Moola to Revati
];

// Yoni Compatibility Matrix (0 to 4 points)
const YONI_MATRIX: [[u8; 14]; 14] = [
    // Hor Ele She Ser Dog Cat Rat Cow Buf Tig Har Mon Lio Mon
    [4, 2, 2, 3, 2, 2, 2, 1, 2, 1, 3, 3, 2, 3], // Horse
    [2, 4, 3, 3, 2, 2, 2, 2, 3, 1, 2, 3, 2, 3], // Elephant
    [2, 3, 4, 2, 1, 2, 2, 3, 3, 0, 3, 2, 1, 2], // Sheep
    [3, 3, 2, 4, 2, 1, 1, 1, 2, 2, 2, 2, 1, 0], // Serpent
    [2, 2, 1, 2, 4, 0, 2, 2, 2, 1, 2, 2, 1, 2], // Dog
    [2, 2, 2, 1, 0, 4, 2, 2, 2, 2, 3, 2, 1, 2], // Cat
    [2, 2, 2, 1, 2, 2, 4, 2, 2, 1, 2, 2, 1, 1], // Rat
    [1, 2, 3, 1, 2, 2, 2, 4, 3, 0, 3, 2, 1, 2], // Cow
    [2, 3, 3, 2, 2, 2, 2, 3, 4, 1, 2, 2, 1, 2], // Buffalo
    [1, 1, 0, 2, 1, 2, 1, 0, 1, 4, 1, 1, 2, 1], // Tiger
    [3, 2, 3, 2, 2, 3, 2, 3, 2, 1, 4, 2, 1, 2], // Hare
    [3, 3, 2, 2, 2, 2, 2, 2, 2, 1, 2, 4, 3, 2], // Monkey
    [2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 3, 4, 2], // Lion
    [3, 3, 2, 0, 2, 2, 1, 2, 2, 1, 2, 2, 2, 4], // Mongoose
];

// Varna (Class): Brahmin=4, Kshatriya=3, Vaishya=2, Shudra=1
const NAKSHATRA_VARNA: [u8; 27] = [
    3, 3, 1, 1, 1, 1, 1, 4, 4, // Ashwini to Ashlesha
    3, 3, 3, 3, 2, 2, 2, 2, 2, // Magha to Jyeshtha
    3, 3, 3, 1, 1, 1, 1, 4, 4 // Moola to Revati
];

// Gana: Deva=0, Manushya=1, Rakshasa=2
const NAKSHATRA_GANA: [u8; 27] = [
    0, 1, 2, 1, 0, 1, 0, 0, 2, // Ashwini to Ashlesha
    2, 1, 1, 0, 2, 0, 0, 1, 2, // Magha to Jyeshtha
    2, 1, 1, 0, 2, 2, 1, 0, 0 // Moola to Revati
];

// Nadi: Adi=0, Madhya=1, Antya=2
const NAKSHATRA_NADI: [u8; 27] = [
    0, 1, 2, 2, 1, 0, 0, 1, 2, // Ashwini to Ashlesha
    0, 1, 2, 2, 1, 0, 0, 1, 2, // Magha to Jyeshtha
    0, 1, 2, 2, 1, 0, 0, 1, 2 // Moola to Revati
];

// Sign lords
const RASHI_LORDS: [usize; 12] = [
    4, 3, 2, 1, 0, 2, 3, 4, 5, 6, 6, 5 // Aries (Mars=4) to Pisces (Jup=5)
];

// Friendship grid between lords (0: Sun, 1: Moon, 2: Mercury, 3: Venus, 4: Mars, 5: Jupiter, 6: Saturn)
// 2: Friend, 1: Neutral, 0: Enemy
const LORD_FRIENDSHIP: [[u8; 7]; 7] = [
    // Sun Moon Mer Ven Mar Jup Sat
    [2, 2, 1, 0, 2, 2, 0], // Sun
    [2, 2, 2, 1, 1, 1, 1], // Moon
    [2, 0, 2, 2, 1, 1, 2], // Mercury
    [0, 0, 2, 2, 1, 1, 2], // Venus
    [2, 2, 0, 1, 2, 2, 1], // Mars
    [2, 2, 0, 0, 2, 2, 2], // Jupiter
    [0, 0, 2, 2, 0, 2, 2], // Saturn
];

/// Calculate Ashtakoota Milan points between a Boy and a Girl
pub fn calculate_ashtakoota_milan(
    boy_moon_long: f64,
    girl_moon_long: f64,
) -> AshtakootaResult {
    let nak_span = 360.0 / 27.0;

    let b_nak = ((boy_moon_long % 360.0) / nak_span).floor() as usize % 27;
    let g_nak = ((girl_moon_long % 360.0) / nak_span).floor() as usize % 27;

    let b_sign = ((boy_moon_long % 360.0) / 30.0).floor() as usize % 12;
    let g_sign = ((girl_moon_long % 360.0) / 30.0).floor() as usize % 12;

    // 1. Varna (1 Point)
    let b_varna = NAKSHATRA_VARNA[b_nak];
    let g_varna = NAKSHATRA_VARNA[g_nak];
    let varna_score = if b_varna >= g_varna { 1.0 } else { 0.0 };

    // 2. Vashya (2 Points)
    // Simple sign mapping for MVP: same sign gets 2, different friendly gets 1, others 0
    let vashya_score = if b_sign == g_sign {
        2.0
    } else if (b_sign + g_sign) % 2 == 0 {
        1.0
    } else {
        0.0
    };

    // 3. Tara (3 Points)
    // Count from girl to boy, and boy to girl
    let dist_g_to_b = (b_nak as i32 - g_nak as i32 + 27) % 9;
    let dist_b_to_g = (g_nak as i32 - b_nak as i32 + 27) % 9;
    let val_g_to_b = if dist_g_to_b == 0 { 9 } else { dist_g_to_b };
    let val_b_to_g = if dist_b_to_g == 0 { 9 } else { dist_b_to_g };

    let is_fav_g_to_b = val_g_to_b % 2 == 0 || val_g_to_b == 9;
    let is_fav_b_to_g = val_b_to_g % 2 == 0 || val_b_to_g == 9;

    let tara_score = if is_fav_g_to_b && is_fav_b_to_g {
        3.0
    } else if is_fav_g_to_b || is_fav_b_to_g {
        1.5
    } else {
        0.0
    };

    // 4. Yoni (4 Points)
    let b_yoni = NAKSHATRA_YONI[b_nak];
    let g_yoni = NAKSHATRA_YONI[g_nak];
    let yoni_score = YONI_MATRIX[b_yoni][g_yoni] as f64;

    // 5. Graha Maitri (5 Points)
    let b_lord = RASHI_LORDS[b_sign];
    let g_lord = RASHI_LORDS[g_sign];
    let f1 = LORD_FRIENDSHIP[b_lord][g_lord];
    let f2 = LORD_FRIENDSHIP[g_lord][b_lord];
    let maitri_score = match (f1, f2) {
        (2, 2) => 5.0,
        (2, 1) | (1, 2) => 4.0,
        (1, 1) => 3.0,
        (2, 0) | (0, 2) => 2.0,
        (1, 0) | (0, 1) => 1.0,
        _ => 0.0,
    };

    // 6. Gana (6 Points)
    let b_gana = NAKSHATRA_GANA[b_nak];
    let g_gana = NAKSHATRA_GANA[g_nak];
    let gana_score = if b_gana == g_gana {
        6.0
    } else if (b_gana == 0 && g_gana == 1) || (b_gana == 1 && g_gana == 0) {
        5.0
    } else if (b_gana == 0 && g_gana == 2) || (b_gana == 2 && g_gana == 0) {
        1.0
    } else {
        0.0
    };

    // 7. Bhakoot (7 Points)
    let sign_diff = (b_sign as i32 - g_sign as i32 + 12) % 12;
    let has_bhakoot_dosha = sign_diff == 2 || sign_diff == 10 || sign_diff == 6 || sign_diff == 8 || sign_diff == 5 || sign_diff == 7;
    
    // Bhakoot cancellation: same lord, or lords are friendly
    let bhakoot_cancelled = b_lord == g_lord || LORD_FRIENDSHIP[b_lord][g_lord] == 2;
    let bhakoot_score = if !has_bhakoot_dosha {
        7.0
    } else if bhakoot_cancelled {
        7.0
    } else {
        0.0
    };

    // 8. Nadi (8 Points)
    let b_nadi = NAKSHATRA_NADI[b_nak];
    let g_nadi = NAKSHATRA_NADI[g_nak];
    let has_nadi_dosha = b_nadi == g_nadi;
    
    // Nadi cancellation: if different nakshatras but same sign, or same nakshatra but different padas
    let nadi_cancelled = b_nak != g_nak && b_sign == g_sign;
    let nadi_score = if !has_nadi_dosha {
        8.0
    } else if nadi_cancelled {
        8.0
    } else {
        0.0
    };

    let total_score = varna_score + vashya_score + tara_score + yoni_score + maitri_score + gana_score + bhakoot_score + nadi_score;

    AshtakootaResult {
        varna_score,
        vashya_score,
        tara_score,
        yoni_score,
        maitri_score,
        gana_score,
        bhakoot_score,
        nadi_score,
        total_score,
        has_nadi_dosha: has_nadi_dosha && !nadi_cancelled,
        has_bhakoot_dosha: has_bhakoot_dosha && !bhakoot_cancelled,
    }
}

/// Analyze Manglik Dosha severity and cancellations for a chart
pub fn analyze_manglik_dosha(
    mars_long: f64,
    lagna_long: f64,
    moon_long: f64,
    venus_long: f64,
) -> ManglikResult {
    let mars_sign = ((mars_long % 360.0) / 30.0).floor() as i32 + 1;
    let lagna_sign = ((lagna_long % 360.0) / 30.0).floor() as i32 + 1;
    let moon_sign = ((moon_long % 360.0) / 30.0).floor() as i32 + 1;
    let venus_sign = ((venus_long % 360.0) / 30.0).floor() as i32 + 1;

    let mut placements = Vec::new();
    let mut cancellations = Vec::new();
    let mut severity: f64 = 0.0;

    // Checks from Lagna
    let lagna_house = (mars_sign - lagna_sign + 12) % 12 + 1;
    if [1, 2, 4, 7, 8, 12].contains(&lagna_house) {
        placements.push(alloc::format!("Mars in house {} from Lagna", lagna_house));
        severity += 0.5;
    }

    // Checks from Moon
    let moon_house = (mars_sign - moon_sign + 12) % 12 + 1;
    if [1, 2, 4, 7, 8, 12].contains(&moon_house) {
        placements.push(alloc::format!("Mars in house {} from Moon", moon_house));
        severity += 0.3;
    }

    // Checks from Venus
    let venus_house = (mars_sign - venus_sign + 12) % 12 + 1;
    if [1, 2, 4, 7, 8, 12].contains(&venus_house) {
        placements.push(alloc::format!("Mars in house {} from Venus", venus_house));
        severity += 0.2;
    }

    let is_manglik = !placements.is_empty();

    // Cancellations
    if is_manglik {
        // Cancellation 1: Mars in own sign (Aries/Scorpio) or exaltation (Capricorn)
        if [1, 8, 10].contains(&mars_sign) {
            cancellations.push("Mars is in own sign or exaltation sign".to_string());
            severity *= 0.5;
        }

        // Cancellation 2: Mars in Leo
        if mars_sign == 5 {
            cancellations.push("Mars is in Leo (friendly sign of Sun)".to_string());
            severity *= 0.3;
        }

        // Cancellation 3: Mars conjunct or aspected by Jupiter (simplified here as Jupiter is in Leo/Sagittarius/Pisces or same sign)
        // We will assume a general cancellation if Mars is in a Jupiter-ruled sign (Sagittarius/Pisces)
        if [9, 12].contains(&mars_sign) {
            cancellations.push("Mars is in a Jupiter-ruled sign (Sagittarius/Pisces)".to_string());
            severity *= 0.5;
        }

        // Cancellation 4: Mars conjunct Moon (Chandra-Mangala)
        if mars_sign == moon_sign {
            cancellations.push("Chandra-Mangala cancellation (Mars conjunct Moon)".to_string());
            severity *= 0.4;
        }
    }

    ManglikResult {
        is_manglik: is_manglik && severity >= 0.15,
        severity: severity.min(1.0),
        placements,
        cancellations,
    }
}

/// Calculate overall compatibility report between a Boy and a Girl
pub fn calculate_compatibility_report(
    boy_moon_long: f64,
    boy_mars_long: f64,
    boy_lagna_long: f64,
    boy_venus_long: f64,
    girl_moon_long: f64,
    girl_mars_long: f64,
    girl_lagna_long: f64,
    girl_venus_long: f64,
) -> CompatibilityReport {
    let guna_milan = calculate_ashtakoota_milan(boy_moon_long, girl_moon_long);
    let boy_manglik = analyze_manglik_dosha(boy_mars_long, boy_lagna_long, boy_moon_long, boy_venus_long);
    let girl_manglik = analyze_manglik_dosha(girl_mars_long, girl_lagna_long, girl_moon_long, girl_venus_long);

    // Calculate percentage based on Gunas + Manglik balance
    let mut pct = (guna_milan.total_score / 36.0) * 100.0;

    // Adjust for Manglik incompatibility (if one is Manglik and other is not, deduct points)
    if boy_manglik.is_manglik != girl_manglik.is_manglik {
        pct -= 15.0; // Incompatibility deduction
    }

    if pct < 0.0 {
        pct = 0.0;
    }

    CompatibilityReport {
        guna_milan,
        boy_manglik,
        girl_manglik,
        overall_compatibility_pct: pct,
    }
}
