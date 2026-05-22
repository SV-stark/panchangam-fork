use alloc::string::{String, ToString};
use alloc::boxed::Box;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Represents a specific time interval (Muhurat)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Muhurat {
    /// Name of the Muhurat (e.g., "Rahu Kalam")
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
    /// Start time in Unix milliseconds
    pub start: f64,
    /// End time in Unix milliseconds
    pub end: f64,
}

/// Collection of daily Muhurats
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct DayMuhurats {
    /// Period of Raahu (Inauspicious for starting new ventures)
    #[wasm_bindgen(getter_with_clone)]
    pub rahu_kalam: Muhurat,
    /// Period of Yama (Inauspicious)
    #[wasm_bindgen(getter_with_clone)]
    pub yamaganda: Muhurat,
    /// Period of Gulika (Neutral/Inauspicious)
    #[wasm_bindgen(getter_with_clone)]
    pub gulika: Muhurat,
    /// Brahma Muhurta (Pre-dawn)
    #[wasm_bindgen(getter_with_clone)]
    pub brahma_muhurta: Muhurat,
    /// Abhijit Muhurta (Mid-day victory period)
    #[wasm_bindgen(getter_with_clone)]
    pub abhijit_muhurta: Muhurat,
}

/// Calculate Raahu, Yamaganda, and Gulika for a given day
///
/// # Arguments
/// * `sunrise_ms` - Unix timestamp of sunrise in ms
/// * `sunset_ms` - Unix timestamp of sunset in ms
/// * `weekday` - 0=Sunday, 1=Monday, ..., 6=Saturday
#[wasm_bindgen]
pub fn calculate_muhurats(sunrise_ms: f64, sunset_ms: f64, weekday: u8) -> DayMuhurats {
    let duration = sunset_ms - sunrise_ms;
    let octad = duration / 8.0;

    // 1-based octad indices for start times
    // (Rules: 0=Sun, 1=Mon, 2=Tue, 3=Wed, 4=Thu, 5=Fri, 6=Sat)

    // Rahu Kalam (Inauspicious)
    // Sun:8, Mon:2, Tue:7, Wed:5, Thu:6, Fri:4, Sat:3
    let rahu_octad = match weekday {
        0 => 8,
        1 => 2,
        2 => 7,
        3 => 5,
        4 => 6,
        5 => 4,
        6 => 3,
        _ => 8, // Fallback
    };

    // Yamaganda (Inauspicious)
    // Sun:5, Mon:4, Tue:3, Wed:2, Thu:1, Fri:7, Sat:6
    let yama_octad = match weekday {
        0 => 5,
        1 => 4,
        2 => 3,
        3 => 2,
        4 => 1,
        5 => 7,
        6 => 6,
        _ => 5,
    };

    // Gulika Kalam (Neutral/Inauspicious)
    // Sun:7, Mon:6, Tue:5, Wed:4, Thu:3, Fri:2, Sat:1
    let gulika_octad = match weekday {
        0 => 7,
        1 => 6,
        2 => 5,
        3 => 4,
        4 => 3,
        5 => 2,
        6 => 1,
        _ => 7,
    };

    // Helper to create Muhurat
    let make_muhurat = |name: &str, start_octad: u8| -> Muhurat {
        let start_idx = (start_octad - 1) as f64;
        let start_time = sunrise_ms + (start_idx * octad);
        let end_time = start_time + octad;
        Muhurat {
            name: name.to_string(),
            start: start_time,
            end: end_time,
        }
    };

    // Brahma Muhurta: 96 minutes before Sunrise -> Sunrise
    // 96 minutes = 5760000 ms
    let brahma_start = sunrise_ms - 5_760_000.0;
    let brahma = Muhurat {
        name: "Brahma Muhurta".to_string(),
        start: brahma_start,
        end: sunrise_ms,
    };

    // Abhijit Muhurta: 8th Muhurta of 15 day divisions
    // Day duration = sunset - sunrise
    let day_len = sunset_ms - sunrise_ms;
    let muhurta_len = day_len / 15.0;
    let abhijit_start = sunrise_ms + (7.0 * muhurta_len); // Start of 8th
    let abhijit_end = abhijit_start + muhurta_len;

    let abhijit = Muhurat {
        name: "Abhijit Muhurta".to_string(),
        start: abhijit_start,
        end: abhijit_end,
    };

    DayMuhurats {
        rahu_kalam: make_muhurat("Rahu Kalam", rahu_octad),
        yamaganda: make_muhurat("Yamaganda", yama_octad),
        gulika: make_muhurat("Gulika Kalam", gulika_octad),
        brahma_muhurta: brahma,
        abhijit_muhurta: abhijit,
    }
}

/// Represents a Choghadiya Period
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ChoghadiyaPeriod {
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
    #[wasm_bindgen(getter_with_clone)]
    pub nature: String, // Good, Bad, Neutral
    pub start: f64,
    pub end: f64,
}

/// Calculate Day Choghadiya Periods
#[wasm_bindgen]
pub fn calculate_day_choghadiya(
    sunrise_ms: f64,
    sunset_ms: f64,
    weekday: u8,
) -> Box<[ChoghadiyaPeriod]> {
    let duration = (sunset_ms - sunrise_ms) / 8.0;

    // Order: Udveg, Chal, Labh, Amrit, Kaal, Shubh, Rog
    // Sunday: Udveg start
    // Mon: Amrit start
    // Tue: Rog start
    // Wed: Labh start
    // Thu: Shubh start
    // Fri: Chal start
    // Sat: Kaal start

    let periods = ["Udveg", "Chal", "Labh", "Amrit", "Kaal", "Shubh", "Rog"];
    // Natures corresponding to periods
    // Udveg (Bad), Chal (Neutral), Labh (Good), Amrit (Good), Kaal (Bad), Shubh (Good), Rog (Bad)
    let natures = ["Bad", "Neutral", "Good", "Good", "Bad", "Good", "Bad"];

    let start_idx = match weekday {
        0 => 0, // Sun: Udveg
        1 => 3, // Mon: Amrit
        2 => 6, // Tue: Rog
        3 => 2, // Wed: Labh
        4 => 5, // Thu: Shubh
        5 => 1, // Fri: Chal
        6 => 4, // Sat: Kaal
        _ => 0,
    };

    let mut results = Vec::with_capacity(8);
    for i in 0..8 {
        let current_idx = (start_idx + i) % 7;
        let start_time = sunrise_ms + (i as f64 * duration);
        let end_time = start_time + duration;

        results.push(ChoghadiyaPeriod {
            name: periods[current_idx].to_string(),
            nature: natures[current_idx].to_string(),
            start: start_time,
            end: end_time,
        });
    }

    results.into_boxed_slice()
}

/// Calculate Night Choghadiya Periods
/// Note: Requires next day's sunrise for accurate duration.
/// If not provided, assumes equal night length (approximation).
#[wasm_bindgen]
pub fn calculate_night_choghadiya(
    sunset_ms: f64,
    next_sunrise_ms: f64,
    weekday: u8,
) -> Box<[ChoghadiyaPeriod]> {
    let duration = (next_sunrise_ms - sunset_ms) / 8.0;

    // Night Order Starts:
    // Sun: Shubh
    // Mon: Chal
    // Tue: Kaal
    // Wed: Udveg
    // Thu: Amrit
    // Fri: Rog
    // Sat: Labh

    // Using same period names/natures from Day function
    let periods = ["Udveg", "Chal", "Labh", "Amrit", "Kaal", "Shubh", "Rog"];
    let natures = ["Bad", "Neutral", "Good", "Good", "Bad", "Good", "Bad"];

    // Map to indices in `periods` array
    // Shubh=5, Chal=1, Kaal=4, Udveg=0, Amrit=3, Rog=6, Labh=2
    let start_idx = match weekday {
        0 => 5, // Sun: Shubh
        1 => 1, // Mon: Chal
        2 => 4, // Tue: Kaal
        3 => 0, // Wed: Udveg
        4 => 3, // Thu: Amrit
        5 => 6, // Fri: Rog
        6 => 2, // Sat: Labh
        _ => 0,
    };

    let mut results = Vec::with_capacity(8);
    for i in 0..8 {
        let current_idx = (start_idx + i) % 7;
        let start_time = sunset_ms + (i as f64 * duration);
        let end_time = start_time + duration;

        results.push(ChoghadiyaPeriod {
            name: periods[current_idx].to_string(),
            nature: natures[current_idx].to_string(),
            start: start_time,
            end: end_time,
        });
    }

    results.into_boxed_slice()
}

/// Represents a Hora Period (Planetary Hour)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct HoraPeriod {
    #[wasm_bindgen(getter_with_clone)]
    pub lord: String,
    pub start: f64,
    pub end: f64,
}

/// Calculate Day Hora Periods (12 hours)
#[wasm_bindgen]
pub fn calculate_day_hora(sunrise_ms: f64, sunset_ms: f64, weekday: u8) -> Box<[HoraPeriod]> {
    let duration = (sunset_ms - sunrise_ms) / 12.0;

    // Lords: Sun(0), Ven(1), Mer(2), Moon(3), Sat(4), Jup(5), Mars(6)
    // Actually standard ids: Sun=0, Moon=1, Mars=2, Mer=3, Jup=4, Ven=5, Sat=6
    // But Hora order is specific: Sun -> Ven -> Mer -> Moon -> Sat -> Jup -> Mars
    // Let's use names for simplicity.
    let lords = [
        "Sun", "Venus", "Mercury", "Moon", "Saturn", "Jupiter", "Mars",
    ];

    // Determine 1st Hora Lord index based on Weekday
    // Weekday: 0=Sun, 1=Mon, 2=Tue, 3=Wed, 4=Thu, 5=Fri, 6=Sat
    // Sun(0) -> Sun(0)
    // Mon(1) -> Moon(3)
    // Tue(2) -> Mars(6)
    // Wed(3) -> Mer(2)
    // Thu(4) -> Jup(5)
    // Fri(5) -> Ven(1)
    // Sat(6) -> Sat(4)

    let start_lord_idx = match weekday {
        0 => 0, // Sun
        1 => 3, // Moon
        2 => 6, // Mars
        3 => 2, // Mercury
        4 => 5, // Jupiter
        5 => 1, // Venus
        6 => 4, // Saturn
        _ => 0,
    };

    let mut results = Vec::with_capacity(12);
    for i in 0..12 {
        let current_idx = (start_lord_idx + i) % 7;
        let start = sunrise_ms + (i as f64 * duration);
        let end = start + duration;

        results.push(HoraPeriod {
            lord: lords[current_idx].to_string(),
            start,
            end,
        });
    }

    results.into_boxed_slice()
}

/// Calculate Night Hora Periods (12 hours)
#[wasm_bindgen]
pub fn calculate_night_hora(
    sunset_ms: f64,
    next_sunrise_ms: f64,
    weekday: u8,
) -> Box<[HoraPeriod]> {
    let duration = (next_sunrise_ms - sunset_ms) / 12.0;
    let lords = [
        "Sun", "Venus", "Mercury", "Moon", "Saturn", "Jupiter", "Mars",
    ];

    // Night starts after 12th day hora.
    // 1st Day Hora = Weekday Lord.
    // 12th Day Hora -> 6th lord from start (since 12 % 7 = 5).
    // 1st Night Hora = 13th hour = (Start + 12) % 7 = Start + 5.

    // Example: Sunday. Start=Sun(0). 12th=Sat(4). 13th=Jup(5) -- Wait.
    // Sequence: Sun, Ven, Mer, Moon, Sat, Jup, Mars.
    // 0:Sun, 1:Ven, 2:Mer, 3:Moon, 4:Sat, 5:Jup, 6:Mars.
    // Index 12 corresponds to (0 + 12) % 7 = 5 (Jupiter). Correct.
    // So 1st Night Hora Lord index is (Start + 5) % 7.

    let day_start_lord_idx = match weekday {
        0 => 0,
        1 => 3,
        2 => 6,
        3 => 2,
        4 => 5,
        5 => 1,
        6 => 4,
        _ => 0,
    };

    let night_start_lord_idx = (day_start_lord_idx + 12) % 7; // Same as +5

    let mut results = Vec::with_capacity(12);
    for i in 0..12 {
        let current_idx = (night_start_lord_idx + i) % 7;
        let start = sunset_ms + (i as f64 * duration);
        let end = start + duration;

        results.push(HoraPeriod {
            lord: lords[current_idx].to_string(),
            start,
            end,
        });
    }

    results.into_boxed_slice()
}

#[wasm_bindgen]
pub fn calculate_nighttime_inauspicious(
    sunset_ms: f64,
    next_sunrise_ms: f64,
    weekday: u8,
) -> DayMuhurats {
    let duration = next_sunrise_ms - sunset_ms;
    let octad = duration / 8.0;

    let rahu_octad = match weekday {
        0 => 5,
        1 => 1,
        2 => 7,
        3 => 4,
        4 => 3,
        5 => 2,
        6 => 6,
        _ => 5,
    };

    let yama_octad = match weekday {
        0 => 4,
        1 => 3,
        2 => 2,
        3 => 1,
        4 => 7,
        5 => 6,
        6 => 5,
        _ => 4,
    };

    let gulika_octad = match weekday {
        0 => 6,
        1 => 5,
        2 => 4,
        3 => 3,
        4 => 2,
        5 => 1,
        6 => 7,
        _ => 6,
    };

    let make_muhurat = |name: &str, start_octad: u8| -> Muhurat {
        let start_idx = (start_octad - 1) as f64;
        let start_time = sunset_ms + (start_idx * octad);
        let end_time = start_time + octad;
        Muhurat {
            name: name.to_string(),
            start: start_time,
            end: end_time,
        }
    };

    DayMuhurats {
        rahu_kalam: make_muhurat("Rahu Kalam (Night)", rahu_octad),
        yamaganda: make_muhurat("Yamaganda (Night)", yama_octad),
        gulika: make_muhurat("Gulika Kalam (Night)", gulika_octad),
        brahma_muhurta: Muhurat {
            name: "Brahma Muhurta (Night)".to_string(),
            start: next_sunrise_ms - 5_760_000.0,
            end: next_sunrise_ms,
        },
        abhijit_muhurta: Muhurat {
            name: "Abhijit Muhurta (Night)".to_string(),
            start: sunset_ms + (3.0 * octad),
            end: sunset_ms + (4.0 * octad),
        },
    }
}

/// Represents a Gowri Period
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct GowriPeriod {
    #[wasm_bindgen(getter_with_clone)]
    pub name: String,
    #[wasm_bindgen(getter_with_clone)]
    pub nature: String,
    pub start: f64,
    pub end: f64,
}

#[wasm_bindgen]
pub fn calculate_day_gowri(
    sunrise_ms: f64,
    sunset_ms: f64,
    weekday: u8,
) -> Box<[GowriPeriod]> {
    let duration = (sunset_ms - sunrise_ms) / 8.0;
    let names = ["Udyoga", "Amrita", "Roga", "Labha", "Shubha", "Dhana", "Kali", "Shula"];
    let natures = ["Bad", "Good", "Bad", "Good", "Good", "Good", "Bad", "Bad"];

    let start_idx = weekday as usize % 8;

    let mut results = Vec::with_capacity(8);
    for i in 0..8 {
        let current_idx = (start_idx + i) % 8;
        let start = sunrise_ms + (i as f64 * duration);
        let end = start + duration;
        results.push(GowriPeriod {
            name: names[current_idx].to_string(),
            nature: natures[current_idx].to_string(),
            start,
            end,
        });
    }
    results.into_boxed_slice()
}

#[wasm_bindgen]
pub fn calculate_night_gowri(
    sunset_ms: f64,
    next_sunrise_ms: f64,
    weekday: u8,
) -> Box<[GowriPeriod]> {
    let duration = (next_sunrise_ms - sunset_ms) / 8.0;
    let names = ["Udyoga", "Amrita", "Roga", "Labha", "Shubha", "Dhana", "Kali", "Shula"];
    let natures = ["Bad", "Good", "Bad", "Good", "Good", "Good", "Bad", "Bad"];

    let start_idx = (weekday as usize + 4) % 8;

    let mut results = Vec::with_capacity(8);
    for i in 0..8 {
        let current_idx = (start_idx + i) % 8;
        let start = sunset_ms + (i as f64 * duration);
        let end = start + duration;
        results.push(GowriPeriod {
            name: names[current_idx].to_string(),
            nature: natures[current_idx].to_string(),
            start,
            end,
        });
    }
    results.into_boxed_slice()
}

