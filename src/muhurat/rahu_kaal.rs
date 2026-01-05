//! Rahu Kaal, Yamaganda, and Gulika Kaal calculations
//! Based on 8-part division of the day

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

/// Day part offsets for Rahu Kaal by weekday (0=Sunday)
/// Each value represents which 8th part of the day is Rahu Kaal
const RAHU_OFFSETS: [u8; 7] = [8, 2, 7, 5, 6, 4, 3];

/// Day part offsets for Yamaganda by weekday
const YAMA_OFFSETS: [u8; 7] = [5, 4, 3, 2, 1, 7, 6];

/// Day part offsets for Gulika by weekday
const GULIKA_OFFSETS: [u8; 7] = [7, 6, 5, 4, 3, 2, 1];

/// Time interval
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct TimeInterval {
    /// Start time as Unix timestamp in ms
    pub start_ms: f64,
    /// End time as Unix timestamp in ms
    pub end_ms: f64,
    /// Duration in minutes
    pub duration_minutes: f64,
}

/// Calculate Rahu Kaal for a given day
/// sunrise_ms and sunset_ms are Unix timestamps in milliseconds
/// weekday is 0=Sunday, 6=Saturday
#[wasm_bindgen]
pub fn calculate_rahu_kaal(sunrise_ms: f64, sunset_ms: f64, weekday: u8) -> TimeInterval {
    calculate_inauspicious_period(sunrise_ms, sunset_ms, weekday, &RAHU_OFFSETS)
}

/// Calculate Yamaganda for a given day
#[wasm_bindgen]
pub fn calculate_yamaganda(sunrise_ms: f64, sunset_ms: f64, weekday: u8) -> TimeInterval {
    calculate_inauspicious_period(sunrise_ms, sunset_ms, weekday, &YAMA_OFFSETS)
}

/// Calculate Gulika Kaal for a given day
#[wasm_bindgen]
pub fn calculate_gulika(sunrise_ms: f64, sunset_ms: f64, weekday: u8) -> TimeInterval {
    calculate_inauspicious_period(sunrise_ms, sunset_ms, weekday, &GULIKA_OFFSETS)
}

fn calculate_inauspicious_period(sunrise_ms: f64, sunset_ms: f64, weekday: u8, offsets: &[u8; 7]) -> TimeInterval {
    let day_duration = sunset_ms - sunrise_ms;
    let part_duration = day_duration / 8.0;
    
    let offset = offsets[weekday as usize % 7] as f64 - 1.0;
    let start_ms = sunrise_ms + part_duration * offset;
    let end_ms = start_ms + part_duration;
    
    TimeInterval {
        start_ms,
        end_ms,
        duration_minutes: part_duration / 60000.0,
    }
}
