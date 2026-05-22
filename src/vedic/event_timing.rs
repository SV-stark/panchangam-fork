//! Event Timing Engine (EventTimingService)
//!
//! Intersects a user's Vimshottari Dasha periods with real-time Planetary Transits (Gochara)
//! and Gochara Vedha (transit obstructions) to predict favorable and challenging timing windows
//! for Marriage, Career, Finance, and Health.

use crate::vedic::dasha::calculate_vimshottari_5_levels;
use crate::vedic::transits::{check_sade_sati, check_dhaiya};
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// A timing window representation for a specific checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct TimingWindow {
    pub start_date: f64, // Unix ms
    pub end_date: f64,   // Unix ms
    pub score: f64,      // 0 to 100
    pub status: String,  // "Highly Favorable", "Favorable", "Neutral", "Challenging", "Highly Challenging"
    pub description: String,
}

/// Consolidated event timing report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct EventTimingReport {
    pub marriage_windows: Vec<TimingWindow>,
    pub career_windows: Vec<TimingWindow>,
    pub finance_windows: Vec<TimingWindow>,
    pub health_windows: Vec<TimingWindow>,
}

/// Input transit checkpoint for event timing calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitCheckpoint {
    pub time_ms: f64,
    /// Planet ID -> Longitude
    /// 0: Sun, 1: Moon, 2: Mars, 3: Mercury, 4: Jupiter, 5: Venus, 6: Saturn, 7: Rahu, 8: Ketu
    pub planet_longs: BTreeMap<i32, f64>,
}

/// Maps a planet ID to its traditional name
fn get_planet_name(id: i32) -> &'static str {
    match id {
        0 => "Sun",
        1 => "Moon",
        2 => "Mars",
        3 => "Mercury",
        4 => "Jupiter",
        5 => "Venus",
        6 => "Saturn",
        7 => "Rahu",
        8 => "Ketu",
        _ => "Unknown",
    }
}

/// Map house ruler sign indexes (0-11 for Aries-Pisces)
/// Aries (0): Mars (2), Taurus (1): Venus (5), Gemini (2): Mercury (3), Cancer (3): Moon (1),
/// Leo (4): Sun (0), Virgo (5): Mercury (3), Libra (6): Venus (5), Scorpio (7): Mars (2),
/// Sagittarius (8): Jupiter (4), Capricorn (9): Saturn (6), Aquarius (10): Saturn (6), Pisces (11): Jupiter (4)
fn get_lord_for_sign(sign_idx: usize) -> i32 {
    const HOUSE_LORDS: [i32; 12] = [2, 5, 3, 1, 0, 3, 5, 2, 4, 6, 6, 4];
    HOUSE_LORDS[sign_idx % 12]
}

/// Calculate Gochara Vedha obstruction in pure Rust for internal scoring
fn calculate_internal_vedha(
    planet_id: i32,
    house_from_moon: i32,
    other_transits: &BTreeMap<i32, i32>,
) -> bool {
    // Format: (Favorable House, Vedha House)
    let favorable_vec = match planet_id {
        0 => vec![(3, 9), (6, 12), (10, 4), (11, 5)],
        1 => vec![(1, 5), (3, 9), (6, 12), (7, 2), (10, 4), (11, 8)],
        2 => vec![(3, 12), (6, 9), (11, 5)],
        3 => vec![(2, 5), (4, 3), (6, 9), (8, 1), (10, 8), (11, 12)],
        4 => vec![(2, 12), (5, 4), (7, 3), (9, 10), (11, 8)],
        5 => vec![
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
        6 => vec![(3, 12), (6, 9), (11, 5)],
        _ => vec![],
    };

    let is_fav = favorable_vec.iter().any(|&(fav, _)| fav == house_from_moon);
    if !is_fav {
        return false;
    }

    if let Some(&(_, vedha_house)) = favorable_vec
        .iter()
        .find(|&&(fav, _)| fav == house_from_moon)
    {
        for (&other_id, &other_house) in other_transits.iter() {
            if other_id == planet_id {
                continue;
            }
            if other_house == vedha_house {
                // Exceptions: Sun & Saturn do not obstruct each other
                if (planet_id == 0 && other_id == 6) || (planet_id == 6 && other_id == 0) {
                    continue;
                }
                // Mercury & Moon do not obstruct each other
                if (planet_id == 3 && other_id == 1) || (planet_id == 1 && other_id == 3) {
                    continue;
                }
                return true; // Vedha is active (obstructed)
            }
        }
    }
    false
}

/// Main high-precision Event Timing Service
pub fn calculate_event_timing(
    birth_time_ms: f64,
    natal_moon_long: f64,
    natal_lagna_long: f64,
    natal_planet_longs: &JsValue, // Map of planet_id -> natal_longitude
    checkpoints_js: &JsValue,      // Array of TransitCheckpoints
) -> Result<EventTimingReport, JsValue> {
    let natal_longs: BTreeMap<i32, f64> = serde_wasm_bindgen::from_value(natal_planet_longs.clone())
        .unwrap_or_default();
    
    // Parse checkpoint array
    let checkpoints: Vec<TransitCheckpoint> = serde_wasm_bindgen::from_value(checkpoints_js.clone())?;

    let mut marriage_windows = Vec::new();
    let mut career_windows = Vec::new();
    let mut finance_windows = Vec::new();
    let mut health_windows = Vec::new();

    let natal_moon_sign = ((natal_moon_long % 360.0) / 30.0).floor() as i32 + 1;
    let lagna_sign_idx = ((natal_lagna_long % 360.0) / 30.0).floor() as usize % 12;

    // Derived natal house lords from Lagna sign
    let second_lord = get_lord_for_sign(lagna_sign_idx + 1);
    let sixth_lord = get_lord_for_sign(lagna_sign_idx + 5);
    let seventh_lord = get_lord_for_sign(lagna_sign_idx + 6);
    let tenth_lord = get_lord_for_sign(lagna_sign_idx + 9);
    let eleventh_lord = get_lord_for_sign(lagna_sign_idx + 10);
    let lagna_lord = get_lord_for_sign(lagna_sign_idx);

    let total_checkpoints = checkpoints.len();

    for (idx, cp) in checkpoints.iter().enumerate() {
        let current_time_ms = cp.time_ms;
        
        // Define interval bounds
        let start_date = current_time_ms;
        let end_date = if idx + 1 < total_checkpoints {
            checkpoints[idx + 1].time_ms
        } else {
            current_time_ms + 30.0 * 24.0 * 3600.0 * 1000.0 // +30 days default for final
        };

        // 1. Calculate Vimshottari Dasha at this checkpoint
        let dasha = calculate_vimshottari_5_levels(natal_moon_long, birth_time_ms, current_time_ms);
        let md = dasha.mahadasha.clone();
        let ad = dasha.antardasha.clone();

        // 2. Parse current transit houses from Moon
        let mut transit_houses = BTreeMap::new();
        for (&p_id, &p_long) in cp.planet_longs.iter() {
            let t_sign = ((p_long % 360.0) / 30.0).floor() as i32 + 1;
            let house = (t_sign - natal_moon_sign + 12) % 12 + 1;
            transit_houses.insert(p_id, house);
        }

        // ==========================================
        // CATEGORY A: MARRIAGE TIMING ANALYSIS
        // ==========================================
        let mut marriage_score: f64 = 50.0;
        let mut marriage_desc = alloc::format!(
            "Under {} Mahadasha and {} Antardasha. ",
            md, ad
        );

        // Dasha modifiers
        if ["Jupiter", "Venus"].contains(&md.as_str()) {
            marriage_score += 15.0;
            marriage_desc.push_str("Mahadasha lord is a prime marriage significator (Karka). ");
        }
        if ["Jupiter", "Venus"].contains(&ad.as_str()) {
            marriage_score += 10.0;
            marriage_desc.push_str("Antardasha lord is favorable for partnership. ");
        }
        let seventh_lord_name = get_planet_name(seventh_lord);
        if md == seventh_lord_name {
            marriage_score += 10.0;
            marriage_desc.push_str("Mahadasha of the 7th house lord triggers partnership themes. ");
        }
        if ad == seventh_lord_name {
            marriage_score += 10.0;
            marriage_desc.push_str("Antardasha of the 7th lord activates the house of union. ");
        }

        // Transit modifiers (Jupiter / Venus transits)
        if let Some(&jup_house) = transit_houses.get(&4) {
            let is_fav = [2, 5, 7, 9, 11].contains(&jup_house);
            let has_vedha = calculate_internal_vedha(4, jup_house, &transit_houses);
            if is_fav {
                if has_vedha {
                    marriage_score += 5.0;
                    marriage_desc.push_str("Transit Jupiter is in a favorable house from Moon but has Gochara Vedha (obstruction). ");
                } else {
                    marriage_score += 15.0;
                    marriage_desc.push_str("Transit Jupiter aspects/occupies an auspicious house from Moon, providing a major blessing. ");
                }
            } else {
                marriage_score -= 5.0;
            }
        }
        if let Some(&ven_house) = transit_houses.get(&5) {
            let is_fav = [1, 2, 3, 4, 5, 8, 9, 11, 12].contains(&ven_house);
            let has_vedha = calculate_internal_vedha(5, ven_house, &transit_houses);
            if is_fav && !has_vedha {
                marriage_score += 5.0;
            }
        }

        marriage_score = marriage_score.clamp(0.0, 100.0);
        let marriage_status = get_status_str(marriage_score);

        marriage_windows.push(TimingWindow {
            start_date,
            end_date,
            score: marriage_score,
            status: marriage_status,
            description: marriage_desc,
        });

        // ==========================================
        // CATEGORY B: CAREER TIMING ANALYSIS
        // ==========================================
        let mut career_score: f64 = 50.0;
        let mut career_desc = alloc::format!(
            "Under {} Mahadasha and {} Antardasha. ",
            md, ad
        );

        if ["Sun", "Saturn"].contains(&md.as_str()) {
            career_score += 10.0;
            career_desc.push_str("Mahadasha lord drives professional discipline/authority. ");
        }
        let tenth_lord_name = get_planet_name(tenth_lord);
        let lagna_lord_name = get_planet_name(lagna_lord);
        if md == tenth_lord_name || md == lagna_lord_name {
            career_score += 12.0;
            career_desc.push_str("Career or Lagna lord Mahadasha increases career focus. ");
        }
        if ad == tenth_lord_name {
            career_score += 10.0;
            career_desc.push_str("Antardasha of 10th lord brings vocational activity. ");
        }

        // Transit modifiers (Sun / Saturn transits)
        if let Some(&sun_house) = transit_houses.get(&0) {
            let is_fav = [3, 6, 10, 11].contains(&sun_house);
            let has_vedha = calculate_internal_vedha(0, sun_house, &transit_houses);
            if is_fav && !has_vedha {
                career_score += 8.0;
                career_desc.push_str("Sun transits a highly supportive house, giving authority. ");
            }
        }
        if let Some(&sat_house) = transit_houses.get(&6) {
            let is_fav = [3, 6, 11].contains(&sat_house);
            let has_vedha = calculate_internal_vedha(6, sat_house, &transit_houses);
            if is_fav {
                if has_vedha {
                    career_score += 3.0;
                    career_desc.push_str("Saturn transit is naturally favorable but obstructed. ");
                } else {
                    career_score += 12.0;
                    career_desc.push_str("Saturn is transiting an auspicious house, ensuring professional success. ");
                }
            } else {
                career_score -= 8.0;
                career_desc.push_str("Saturn transit is challenging, requiring patient labor. ");
            }
        }

        career_score = career_score.clamp(0.0, 100.0);
        let career_status = get_status_str(career_score);

        career_windows.push(TimingWindow {
            start_date,
            end_date,
            score: career_score,
            status: career_status,
            description: career_desc,
        });

        // ==========================================
        // CATEGORY C: FINANCE TIMING ANALYSIS
        // ==========================================
        let mut finance_score: f64 = 50.0;
        let mut finance_desc = alloc::format!(
            "Under {} Mahadasha and {} Antardasha. ",
            md, ad
        );

        if md == "Jupiter" || md == "Venus" {
            finance_score += 10.0;
            finance_desc.push_str("Mahadasha lord is a natural wealth giver (Dhana-karaka). ");
        }
        let second_lord_name = get_planet_name(second_lord);
        let eleventh_lord_name = get_planet_name(eleventh_lord);
        if md == second_lord_name || md == eleventh_lord_name {
            finance_score += 12.0;
            finance_desc.push_str("Wealth-house (2nd/11th) lord Mahadasha active. ");
        }
        if ad == eleventh_lord_name {
            finance_score += 10.0;
            finance_desc.push_str("Antardasha of 11th lord promises financial gains. ");
        }

        // Transits (Jupiter and Mercury)
        if let Some(&jup_house) = transit_houses.get(&4) {
            let is_fav = [2, 5, 7, 9, 11].contains(&jup_house);
            let has_vedha = calculate_internal_vedha(4, jup_house, &transit_houses);
            if is_fav && !has_vedha {
                finance_score += 15.0;
                finance_desc.push_str("Transit Jupiter is extremely favorable, bringing financial ease. ");
            }
        }
        if let Some(&mer_house) = transit_houses.get(&3) {
            let is_fav = [2, 4, 6, 8, 10, 11].contains(&mer_house);
            let has_vedha = calculate_internal_vedha(3, mer_house, &transit_houses);
            if is_fav && !has_vedha {
                finance_score += 5.0;
            }
        }

        finance_score = finance_score.clamp(0.0, 100.0);
        let finance_status = get_status_str(finance_score);

        finance_windows.push(TimingWindow {
            start_date,
            end_date,
            score: finance_score,
            status: finance_status,
            description: finance_desc,
        });

        // ==========================================
        // CATEGORY D: HEALTH TIMING ANALYSIS
        // ==========================================
        let mut health_score: f64 = 70.0; // Start with healthy baseline
        let mut health_desc = alloc::format!(
            "Under {} Mahadasha and {} Antardasha. ",
            md, ad
        );

        let sixth_lord_name = get_planet_name(sixth_lord);
        if md == sixth_lord_name {
            health_score -= 15.0;
            health_desc.push_str("Mahadasha of the 6th lord of struggles may invite health vulnerability. ");
        }
        if ad == sixth_lord_name {
            health_score -= 10.0;
            health_desc.push_str("Antardasha of the 6th lord highlights wellness adjustment needs. ");
        }

        // Saturn Sade Sati and Dhaiya calculations
        if let Some(saturn_sign_u8) = cp.planet_longs.get(&6).map(|long| ((long % 360.0) / 30.0).floor() as u8 + 1) {
            let sade_sati = check_sade_sati(natal_moon_sign as u8, saturn_sign_u8);
            let dhaiya = check_dhaiya(natal_moon_sign as u8, saturn_sign_u8);

            if sade_sati.is_active {
                health_score -= 15.0;
                health_desc.push_str(&alloc::format!(
                    "Active Saturn Sade Sati ({} Phase) impacts physical/emotional vitality. ",
                    sade_sati.phase
                ));
            }
            if dhaiya.is_active {
                health_score -= 10.0;
                health_desc.push_str(&alloc::format!(
                    "Active Saturn Dhaiya ({}) creates domestic and health stress. ",
                    dhaiya.type_name
                ));
            }
        }

        // Sun transit (vitality significator)
        if let Some(&sun_house) = transit_houses.get(&0) {
            let is_unfav = [8, 12].contains(&sun_house);
            if is_unfav {
                health_score -= 8.0;
                health_desc.push_str("Sun transiting the 8th/12th from Moon temporarily lowers energy. ");
            }
        }

        health_score = health_score.clamp(0.0, 100.0);
        let health_status = get_status_str(health_score);

        health_windows.push(TimingWindow {
            start_date,
            end_date,
            score: health_score,
            status: health_status,
            description: health_desc,
        });
    }

    Ok(EventTimingReport {
        marriage_windows,
        career_windows,
        finance_windows,
        health_windows,
    })
}

fn get_status_str(score: f64) -> String {
    if score >= 80.0 {
        "Highly Favorable".to_string()
    } else if score >= 60.0 {
        "Favorable".to_string()
    } else if score >= 40.0 {
        "Neutral".to_string()
    } else if score >= 20.0 {
        "Challenging".to_string()
    } else {
        "Highly Challenging".to_string()
    }
}
