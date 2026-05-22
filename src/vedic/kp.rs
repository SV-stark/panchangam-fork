//! Krishnamurti Paddhati (KP) System Calculations
//!
//! Provides calculations for:
//! - Sign Lord
//! - Star Lord (Nakshatra Lord)
//! - Sub Lord
//! - Sub-Sub Lord
//!
//! Based on standard KP tables and Vimshottari Dasha proportions.

use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct KPLordInfo {
    /// Sign ID (1-12)
    pub sign_id: u8,
    /// Sign Lord (Planet ID)
    pub sign_lord: i32,
    /// Star Lord (Planet ID)
    pub star_lord: i32,
    /// Sub Lord (Planet ID)
    pub sub_lord: i32,
    /// Sub-Sub Lord (Planet ID)
    pub sub_sub_lord: i32,
}

// Vimshottari Lords order (Ketu..Mercury) matches IDs 0..8?
// Standard IDs in this lib: Sun=1, Moon=2, Mars=3, Mer=4, Jup=5, Ven=6, Sat=7, Rahu=8, Ketu=9
// But NAKSHATRA_LORDS usually returns string names.
// We need a mapping from Nakshatra Lord index to Planet ID.

// Vimshottari Order:
// Ketu(9), Ven(6), Sun(1), Moon(2), Mars(3), Rahu(8), Jup(5), Sat(7), Mer(4)
const KP_LORD_ORDER: [i32; 9] = [9, 6, 1, 2, 3, 8, 5, 7, 4];
const KP_LORD_DURATIONS: [f64; 9] = [7.0, 20.0, 6.0, 10.0, 7.0, 18.0, 16.0, 19.0, 17.0];

/// Calculate KP Lords for a given longitude
pub fn calculate_kp_lords(long: f64) -> KPLordInfo {
    // Normalize logic
    let mut l = long % 360.0;
    if l < 0.0 {
        l += 360.0;
    }

    // 1. Sign Lord
    let sign_idx = (l / 30.0).floor() as usize; // 0-11
    let sign_id = (sign_idx + 1) as u8;
    // Standard Rulerships:
    // Aries(0)->Mars(3), Tau(1)->Ven(6), Gem(2)->Mer(4), Can(3)->Mon(2), Leo(4)->Sun(1), Vir(5)->Mer(4)
    // Lib(6)->Ven(6), Sco(7)->Mars(3), Sag(8)->Jup(5), Cap(9)->Sat(7), Aqu(10)->Sat(7), Pis(11)->Jup(5)
    let sign_lords = [3, 6, 4, 2, 1, 4, 6, 3, 5, 7, 7, 5];
    let sign_lord = sign_lords[sign_idx];

    // 2. Star Lord (Nakshatra)
    // 360 / 27 = 13.3333... deg per star
    let star_span = 360.0 / 27.0;
    let star_idx = (l / star_span).floor() as usize; // 0-26
                                                     // Vimshottari sequence start depends on star index.
                                                     // Index % 9 gives position in [Ketu, Ven, Sun, Mon, Mar, Rah, Jup, Sat, Mer]
    let lord_idx = star_idx % 9;
    let star_lord = KP_LORD_ORDER[lord_idx];

    // 3. Sub Lord
    // Degrees traversed in current star
    let deg_in_star = l - (star_idx as f64 * star_span);
    // Proportion traversed (0.0 to 1.0)
    // Total Vimshottari cycle = 120 years
    // The star is divided according to years of planets in Vimshottari order, starting from Star Lord.

    // Convert deg_in_star to "Years" scale:
    // 13.333 deg = 120 years
    // X deg = (X / 13.333) * 120 years
    let years_traversed = (deg_in_star / star_span) * 120.0;

    let mut current_lord_idx = lord_idx; // Variable index for sub levels
    let mut remaining_years = years_traversed;

    // Find Sub Lord
    // We subtract durations until we find where we land
    let sub_lord = loop {
        let dur = KP_LORD_DURATIONS[current_lord_idx];
        if remaining_years < dur {
            break KP_LORD_ORDER[current_lord_idx];
        }
        remaining_years -= dur;
        current_lord_idx = (current_lord_idx + 1) % 9;
    };

    // 4. Sub-Sub Lord
    // remaining_years is the "years into" the Sub-Lord's period.
    // The Sub-Lord's period (dur) corresponds to its full span in the star.
    // Now we subdivide THAT period into 120 parts again?
    // Yes. KP Sub-Sub logic:
    // The Sub-Lord span is treated as a full 120-year cycle scaled down.
    // Period of Sub-Lord = dur (in years context of star).
    // Let's say Sub-Lord is Venus (20y). We are 5 years into Venus sub.
    // We map 0..20y -> 0..120y scale for next level.
    // scale_factor = 120 / dur.
    // new_years = remaining_years * scale_factor.

    let sub_span_years = KP_LORD_DURATIONS[current_lord_idx];
    let sub_progress = remaining_years; // Years into the sub-period

    // Scale to 120y cycle for next level
    let ss_years_traversed = (sub_progress / sub_span_years) * 120.0;

    // Start sequence from Sub-Lord
    let mut ss_lord_idx = current_lord_idx;
    let mut ss_remaining = ss_years_traversed;

    let sub_sub_lord = loop {
        let dur = KP_LORD_DURATIONS[ss_lord_idx];
        if ss_remaining < dur {
            break KP_LORD_ORDER[ss_lord_idx];
        }
        ss_remaining -= dur;
        ss_lord_idx = (ss_lord_idx + 1) % 9;
    };

    KPLordInfo {
        sign_id,
        sign_lord,
        star_lord,
        sub_lord,
        sub_sub_lord,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct KPLevelInfo {
    pub level_a: Vec<i32>, // Planet in star of occupant
    pub level_b: Vec<i32>, // Occupant
    pub level_c: Vec<i32>, // Planet in star of owner
    pub level_d: Vec<i32>, // Owner
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct KPSignificators {
    #[wasm_bindgen(skip)]
    pub house_significators: Vec<KPLevelInfo>, // 0-11 corresponds to House 1-12
}

/// Calculate KP Significators for all houses
///
/// # Arguments
/// * `planet_longs` - Map of planets and their longitudes (0=Sun..8=Ketu)
/// * `cusps` - 12 house cusps (KP cusps)
pub fn calculate_kp_significators(planet_longs: &JsValue, cusps: Vec<f64>) -> KPSignificators {
    let longs: BTreeMap<i32, f64> =
        serde_wasm_bindgen::from_value(planet_longs.clone()).unwrap_or_default();

    // 1. Calculate KP Lords for all planets
    let mut planet_lords = BTreeMap::new();
    for (&id, &long) in longs.iter() {
        planet_lords.insert(id, calculate_kp_lords(long));
    }

    // 2. Determine House Ownership and Occupation
    // Note: KP uses Placidus or Cuspal intercept method.
    // Sign Lord of the Cusp is the Owner.
    let mut house_owners = [0i32; 12];
    for h in 0..12 {
        let cusp_long = cusps[h];
        let info = calculate_kp_lords(cusp_long);
        house_owners[h] = info.sign_lord;
    }

    // Identify which house each planet occupies
    let mut house_occupants: Vec<Vec<i32>> = vec![vec![]; 12];
    for (&id, &long) in longs.iter() {
        // Find which house. Cusp i to Cusp i+1.
        for h in 0..12 {
            let start = cusps[h];
            let end = cusps[(h + 1) % 12];

            let is_in = if start < end {
                long >= start && long < end
            } else {
                // Wrap around Pis -> Ari
                long >= start || long < end
            };

            if is_in {
                house_occupants[h].push(id);
                break;
            }
        }
    }

    // 3. Populate Levels A, B, C, D
    let mut house_results = vec![];
    for h in 0..12 {
        let owner = house_owners[h];
        let occupants = &house_occupants[h];

        // Level B: Occupants
        let level_b = occupants.clone();

        // Level D: Owner
        let level_d = vec![owner];

        // Level A: Planet in star of occupant
        let mut level_a = vec![];
        for &occ_id in occupants.iter() {
            // Find planets whose star lord is occ_id
            for (&p_id, p_info) in planet_lords.iter() {
                if p_info.star_lord == occ_id {
                    level_a.push(p_id);
                }
            }
        }

        // Level C: Planet in star of owner
        let mut level_c = vec![];
        for (&p_id, p_info) in planet_lords.iter() {
            if p_info.star_lord == owner {
                level_c.push(p_id);
            }
        }

        house_results.push(KPLevelInfo {
            level_a,
            level_b,
            level_c,
            level_d,
        });
    }

    KPSignificators {
        house_significators: house_results,
    }
}

/// Get KP Sub Lord for a given longitude
#[wasm_bindgen]
pub fn get_sub_lord(longitude: f64) -> i32 {
    calculate_kp_lords(longitude).sub_lord
}

/// Get KP Sub-Sub Lord for a given longitude
#[wasm_bindgen]
pub fn get_sub_sub_lord(longitude: f64) -> i32 {
    calculate_kp_lords(longitude).sub_sub_lord
}
