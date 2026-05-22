//! Prashna (Horary) Astrology Module
//!
//! Provides Prashna-specific calculations:
//! - Prashna Arudha (seed-based sign mapping)
//! - Prashna Sphutas (Trisphuta, Chatursphuta)
//! - Gulika Sphuta (Poison/Gulika position)

use alloc::string::{String, ToString};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Calculate Prashna Arudha from seed number
/// Maps seed 1-249 to a sign index 0-11
pub fn calculate_prashna_arudha(seed: i32) -> usize {
    // KP Seed (1-249 maps to sub-lords), simpler Prashna uses 1-12 cycle
    // Standard: map seed modulo 12 to sign
    let normalized = ((seed - 1).rem_euclid(12)) as usize;
    normalized
}

/// Prashna Sphutas result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct PrashnaSphutas {
    /// Trisphuta = Lagna + Moon + Sun (mod 360)
    pub trisphuta: f64,
    /// Chatursphuta = Trisphuta + Rahu
    pub chatursphuta: f64,
    /// Prana Sphuta = Lagna + Moon - Sun (mod 360)
    pub prana_sphuta: f64,
    /// Deha Sphuta = Lagna + Sun - Moon (mod 360)
    pub deha_sphuta: f64,
    /// Mrityu Sphuta = Moon * 2 - Mars (mod 360)
    pub mrityu_sphuta: f64,
    /// Trisphuta sign (0-11)
    pub trisphuta_sign: usize,
    /// Chatursphuta sign (0-11)
    pub chatursphuta_sign: usize,
}

/// Calculate all Prashna Sphutas
/// Arguments:
/// - lagna_long: Ascendant longitude (0-360)
/// - moon_long: Moon longitude (0-360)
/// - sun_long: Sun longitude (0-360)
/// - rahu_long: Rahu longitude (0-360)
/// - mars_long: Mars longitude (0-360)
pub fn calculate_prashna_sphutas(
    lagna_long: f64,
    moon_long: f64,
    sun_long: f64,
    rahu_long: f64,
    mars_long: f64,
) -> PrashnaSphutas {
    let trisphuta = (lagna_long + moon_long + sun_long).rem_euclid(360.0);
    let chatursphuta = (trisphuta + rahu_long).rem_euclid(360.0);
    let prana_sphuta = (lagna_long + moon_long - sun_long + 360.0).rem_euclid(360.0);
    let deha_sphuta = (lagna_long + sun_long - moon_long + 360.0).rem_euclid(360.0);
    let mrityu_sphuta = (moon_long * 2.0 - mars_long + 360.0).rem_euclid(360.0);

    PrashnaSphutas {
        trisphuta,
        chatursphuta,
        prana_sphuta,
        deha_sphuta,
        mrityu_sphuta,
        trisphuta_sign: (trisphuta / 30.0).floor() as usize,
        chatursphuta_sign: (chatursphuta / 30.0).floor() as usize,
    }
}

/// Calculate Gulika Sphuta
/// Gulika divides the day into 8 equal parts. In each part, one planet rules.
/// The Saturn's slot contains Gulika.
/// Arguments:
/// - sunrise_ms: sunrise in Unix ms  
/// - sunset_ms: sunset in Unix ms
/// - weekday_idx: 0=Sunday, 1=Monday ... 6=Saturday
/// - is_day: true for daytime birth, false for night
pub fn calculate_gulika_sphuta(
    sunrise_ms: f64,
    sunset_ms: f64,
    weekday_idx: u8,
    is_day: bool,
) -> f64 {
    // Day is divided into 8 equal parts. Each part = (sunset - sunrise) / 8.
    // Each day-lord rules one part. Gulika is Saturn's part.
    // Day lords sequence starting from Sunday: Sun, Venus, Mercury, Moon, Saturn, Jupiter, Mars
    // For each weekday, Saturn's slot is at a fixed position in the 8-part sequence.

    // Daytime Gulika start slots (0-indexed, 0=first part):
    // Sun:6, Mon:5, Tue:4, Wed:3, Thu:2, Fri:1, Sat:0
    // (Saturn gets 7th slot on Sunday = index 6, etc.)
    let day_slot_start: u8 = match weekday_idx % 7 {
        0 => 6, // Sunday
        1 => 5, // Monday
        2 => 4, // Tuesday
        3 => 3, // Wednesday
        4 => 2, // Thursday
        5 => 1, // Friday
        6 => 0, // Saturday
        _ => 0,
    };

    // Nighttime Gulika start slots:
    // Sun:1, Mon:0, Tue:6, Wed:5, Thu:4, Fri:3, Sat:2
    let night_slot_start: u8 = match weekday_idx % 7 {
        0 => 1,
        1 => 0,
        2 => 6,
        3 => 5,
        4 => 4,
        5 => 3,
        6 => 2,
        _ => 0,
    };

    let slot = if is_day { day_slot_start } else { night_slot_start };

    // Calculate the time at the start of Gulika's slot
    if is_day {
        let day_duration = sunset_ms - sunrise_ms;
        let part_duration = day_duration / 8.0;
        let gulika_start_ms = sunrise_ms + (slot as f64 * part_duration);
        // Convert ms to approximate longitude
        // 24 hours = 360 degrees, so 1 ms = 360/(86400000) degrees
        // But actually Gulika Sphuta is the longitude of the ascendant at Gulika's time
        // For simplification, return the ms time of Gulika start
        // (caller can compute Lagna at that time)
        gulika_start_ms
    } else {
        // Night period: sunset to next sunrise (approximate as same duration)
        let night_duration = sunset_ms - sunrise_ms; // same span
        let part_duration = night_duration / 8.0;
        let gulika_start_ms = sunset_ms + (slot as f64 * part_duration);
        gulika_start_ms
    }
}
