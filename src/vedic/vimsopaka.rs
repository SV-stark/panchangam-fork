//! Vimsopaka Divisional Chart Strength Calculations
//! Based on BPHS rules for 20-point divisional strength (Shadvarga and Saptavarga)

use crate::vedic::vargas::{calculate_varga_position, VargaConfig, VargaType};
use crate::vedic::dignity::{calculate_dignity, Dignity};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

/// Vimsopaka score for an individual planet
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct VimsopakaPlanetResult {
    pub planet: String,
    pub shadvarga_score: f64,
    pub saptavarga_score: f64,
}

/// Vimsopaka Bala scores for all planets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct VimsopakaResult {
    #[wasm_bindgen(getter_with_clone)]
    pub planet_scores: Vec<VimsopakaPlanetResult>,
}

/// Calculate Vimsopaka Divisional Strength scores for the 7 main planets (Sun to Saturn)
pub fn calculate_vimsopaka_bala(
    planet_longs: &[f64],
) -> VimsopakaResult {
    let planet_names = ["Sun", "Moon", "Mercury", "Venus", "Mars", "Jupiter", "Saturn"];
    let mut planet_scores = Vec::new();
    let config = VargaConfig::default();

    for (pid, &long) in planet_longs.iter().enumerate() {
        if pid >= 7 {
            break;
        }
        let p_name = planet_names[pid];

        // 1. Shadvarga weights (Sum = 20): D1=6, D2=2, D3=4, D9=5, D12=2, D30=1
        let shadvarga_weights = [
            (VargaType::D1, 6.0),
            (VargaType::D2, 2.0),
            (VargaType::D3, 4.0),
            (VargaType::D9, 5.0),
            (VargaType::D12, 2.0),
            (VargaType::D30, 1.0),
        ];

        let mut shadvarga_score = 0.0;
        for &(v_type, weight) in &shadvarga_weights {
            let pos = calculate_varga_position(long, v_type, &config);
            let dig = calculate_dignity(p_name, pos.full_longitude);
            let dig_points = match dig {
                Dignity::Exalted => 20.0,
                Dignity::Moolatrikona => 18.0,
                Dignity::OwnSign => 16.0,
                Dignity::GreatFriend => 14.0,
                Dignity::Friend => 12.0,
                Dignity::Neutral => 10.0,
                Dignity::Enemy => 8.0,
                Dignity::GreatEnemy => 6.0,
                Dignity::Debilitated => 4.0,
            };
            shadvarga_score += (dig_points * weight) / 20.0;
        }

        // 2. Saptavarga weights (Sum = 20): D1=5, D2=2, D3=3, D7=2, D9=3, D12=2, D30=3
        let saptavarga_weights = [
            (VargaType::D1, 5.0),
            (VargaType::D2, 2.0),
            (VargaType::D3, 3.0),
            (VargaType::D7, 2.0),
            (VargaType::D9, 3.0),
            (VargaType::D12, 2.0),
            (VargaType::D30, 3.0),
        ];

        let mut saptavarga_score = 0.0;
        for &(v_type, weight) in &saptavarga_weights {
            let pos = calculate_varga_position(long, v_type, &config);
            let dig = calculate_dignity(p_name, pos.full_longitude);
            let dig_points = match dig {
                Dignity::Exalted => 20.0,
                Dignity::Moolatrikona => 18.0,
                Dignity::OwnSign => 16.0,
                Dignity::GreatFriend => 14.0,
                Dignity::Friend => 12.0,
                Dignity::Neutral => 10.0,
                Dignity::Enemy => 8.0,
                Dignity::GreatEnemy => 6.0,
                Dignity::Debilitated => 4.0,
            };
            saptavarga_score += (dig_points * weight) / 20.0;
        }

        planet_scores.push(VimsopakaPlanetResult {
            planet: p_name.to_string(),
            shadvarga_score,
            saptavarga_score,
        });
    }

    VimsopakaResult { planet_scores }
}
