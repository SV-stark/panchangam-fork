use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Ashtottari Dasha Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct AshtottariResult {
    #[wasm_bindgen(getter_with_clone)]
    pub lord: String,
    pub start_ms: f64,
    pub end_ms: f64,
    pub duration_years: f64,
}

/// Calculate Ashtottari Dasha (108 Years)
///
/// Sequence of Lords:
/// Sun (6), Moon (15), Mars (8), Mercury (17), Saturn (10), Jupiter (19), Rahu (12), Venus (21).
/// Total: 108.
///
/// Order of Nakshatras:
/// Different from Vimshottari.
/// Start from Ardra? No.
/// Rule:
/// 1. Count Nakshatra from Ardra (6) or Krittika (3)?
/// There are two variations: Ardra-adi and Krittika-adi.
/// Standard is Ardra-adi for certain conditions (day/night birth, Krishna/Shukla).
/// Simple implementation: **Krittika-adi** (Start from Krittika).
///
/// Sequence of Nakshatras (Abhijit included? Usually yes in Ashtottari, 28 stars).
/// If 28 stars:
/// 4 Nakshatras per Dasha? 28 / 8 = 3.5? No.
/// Lengths vary.
///
/// Groups:
/// 1. Sun (6y): Krittika, Rohini, Mrigashira (3 stars? No)
/// Actually, standard division:
/// Sun: 3 (Krittika, Rohini, Mrigshira? No)
///
/// Let's look up standard mapping.
/// 4 nakshatras -> Sun
/// 3 -> Moon
/// 4 -> Mars
/// 3 -> Mercury
/// 4 -> Saturn
/// 3 -> Jupiter
/// 4 -> Rahu
/// 3 -> Venus
/// Total: 4+3+4+3+4+3+4+3 = 28 Nakshatras.
/// (Includes Abhijit).
///
/// Nakshatra List with Abhijit:
/// (1) Ashwini ... (21) Uttarashadha (22) Abhijit (23) Shravana ...
///
/// But standard Ashtottari starts from Ardra or Krittika based on Lagna/Birth?
/// Lagna in Oja/Yugma?
/// For MVP: Let's implement the most common **Ardra-adi** system used when Rahu is in Kendra/Trikona from Lagna?
/// Wait, default usually Vimshottari. Ashtottari is conditional.
/// Let's implement the calculation assuming the user wants it.
///
/// Scheme (Starting Star):
/// Ardra (6) starts the cycle?
///
/// Pattern (Nakshatra -> Lord):
/// Ardra, Punarvasu, Pushya, Ashlesha -> Sun (6y)
/// Magha, P.Phalguni, U.Phalguni -> Moon (15y)
/// Hasta, Chitra, Swati, Vishakha -> Mars (8y)
/// Anuradha, Jyeshtha, Mula -> Mercury (17y)
/// P.Ashadha, U.Ashadha, Abhijit, Shravana -> Saturn (10y)
/// Dhanishta, Satabhisha, P.Bhadra -> Jupiter (19y)
/// U.Bhadra, Revati, Ashwini, Bharani -> Rahu (12y)
/// Krittika, Rohini, Mrigashira -> Venus (21y)
///
/// Total stars: 4+3+4+3+4+3+4+3 = 28. Perfect.
///
/// Steps:
/// 1. Find Moon's Nakshatra index (0-27 in 28-star system).
/// 2. If Abhijit is used, we need to map 0-360 degrees to 28 stars.
///
/// 28-star mapping:
/// Abhijit is usually placed between U.Ashadha and Shravana.
/// U.Ashadha ends at 276 deg 40 min.
/// Abhijit: 276:40 to 280:53:20 approx?
/// Actually equal division: 360 / 28 = 12 deg 51 min 25 sec.
///
/// But in Dasha, they often use fixed span for Abhijit or equal?
/// Standard Ashtottari uses **Abhijit included**.
/// Let's assume equal division for logic or specific ranges?
/// "In Ashtottari Dasha, the 28 nakshatras are reckoned..."
///
/// Implementation:
/// 1. Identify Nakshatra (1-28).
/// 2. Find Balance of Dasha.
///    - Dasha Lord duration
///    - Elapsed percentage in Nakshatra.
///    - Balance = Total Duration * (1 - Percentage).
///
/// # Arguments
/// * `moon_long` - Moon longitude (0-360)
/// * `birth_time_ms` - Birth Time
/// * `current_time_ms` - Current Time
///
pub fn calculate_ashtottari(
    moon_long: f64,
    birth_time_ms: f64,
    current_time_ms: f64,
) -> Result<Box<[AshtottariResult]>, JsValue> {
    // 1. Get 28-star Index
    // 360 / 28 = 12.8571428... degrees
    let deg_per_star = 360.0 / 28.0;
    let n_idx_float = moon_long / deg_per_star;
    let n_idx = n_idx_float.floor() as usize; // 0-27
    let traversed = n_idx_float - n_idx as f64; // 0.0 to 1.0 fraction
                                                // traversed is fraction *passed*. Remaining = 1.0 - traversed.
    let remaining_frac = 1.0 - traversed;

    // 2. Define Lords and Durations
    // Order from logic above (Ardra Start):
    // Ardra is #6 in standard 27-list.
    // In 28-list:
    // Ashwini(0), Bharani(1), Krittika(2), Rohini(3), Mrigashira(4), Ardra(5).
    // So Ardra is Index 5.

    // Mapping 28 stars (0..27) to Dasha Lords.
    // Group 1 (Sun): Ardra(5), Punar(6), Pushya(7), Ashle(8). (Indices 5,6,7,8)
    // Group 2 (Moon): Magha(9), P.Phal(10), U.Phal(11).
    // ...
    // Let's create a map of Nakshatra Abs Index -> Lord Index.

    // Lord sequence: Sun, Moon, Mar, Mer, Sat, Jup, Rah, Ven
    // Durations: 6, 15, 8, 17, 10, 19, 12, 21. Total 108.

    struct LordInfo {
        name: &'static str,
        dur: f64,
    }
    let lords = [
        LordInfo {
            name: "Sun",
            dur: 6.0,
        },
        LordInfo {
            name: "Moon",
            dur: 15.0,
        },
        LordInfo {
            name: "Mars",
            dur: 8.0,
        },
        LordInfo {
            name: "Mercury",
            dur: 17.0,
        },
        LordInfo {
            name: "Saturn",
            dur: 10.0,
        },
        LordInfo {
            name: "Jupiter",
            dur: 19.0,
        },
        LordInfo {
            name: "Rahu",
            dur: 12.0,
        },
        LordInfo {
            name: "Venus",
            dur: 21.0,
        },
    ];

    // Define start range for each lord (by Nakshatra count)
    // Star 0 (Ashwini) is in Rahu group? No.
    // Let's trace back from Ardra(5)=Sun.
    // Venus ends at 4.
    // Venus has 3 stars: Krittika(2), Rohini(3), Mrigashira(4). Matches!
    // Rahu ends at 1.
    // Rahu has 4 stars: U.Bhadra, Revati, Ashwini(0), Bharani(1). Matches!
    //
    // So mapping:
    // 0,1 -> Rahu (Index 6)
    // 2,3,4 -> Venus (Index 7)
    // 5,6,7,8 -> Sun (Index 0)
    // 9,10,11 -> Moon (Index 1)
    // 12,13,14,15 -> Mars (Index 2)
    // 16,17,18 -> Mercury (Index 3)
    // 19,20,21,22 -> Saturn (Index 4)
    // 23,24,25 -> Jupiter (Index 5)

    // Wait, Abhijit?
    // In 27 star list:
    // ... U.Ashadha(20), Shravana(21).
    // In 28 list:
    // ... P.Ashadha(19), U.Ashadha(20), Abhijit(21), Shravana(22).
    //
    // Saturn check: P.Ash(19), U.Ash(20), Abhijit(21), Shravana(22). -> 4 stars.
    // Matches logic above.

    let (lord_idx, stars_in_group) = match n_idx {
        0 | 1 => (6, 4), // Rahu (part 2) - Wait, U.Bhadra(26), Revati(27) are Rahu too?
        // Let's cycle.
        // 26, 27, 0, 1 -> Rahu.
        26 | 27 => (6, 4),

        2 | 3 | 4 => (7, 3),         // Venus
        5 | 6 | 7 | 8 => (0, 4),     // Sun
        9 | 10 | 11 => (1, 3),       // Moon
        12 | 13 | 14 | 15 => (2, 4), // Mars
        16 | 17 | 18 => (3, 3),      // Mercury
        19 | 20 | 21 | 22 => (4, 4), // Saturn
        23 | 24 | 25 => (5, 3),      // Jupiter

        _ => (0, 4), // Fallback
    };

    // Calculate Balance
    // Total duration of *this specific nakshatra* in the dasha?
    // In Ashtottari, the dasha duration is spread over the group's nakshatras.
    // E.g. Sun (6 years) covers 4 nakshatras.
    // So each nakshatra contributes 6/4 = 1.5 years? YES.

    let lord = &lords[lord_idx];
    let duration_per_star = lord.dur / (stars_in_group as f64);
    let balance_years = remaining_frac * duration_per_star;

    // We also need to add remaining years of the Dasha Lord if we are not in the last star of the group?
    // Wait. "Balance at birth".
    // Is it balance of the *Lord's Dasha* or just the Star's portion?
    // It's usually balance of the Lord.
    //
    // We need to know which star in the group we are.
    // E.g. Sun Group: 5,6,7,8.
    // If we are at 5 (Ardra), we have (Balance of 5) + (Full 6) + (Full 7) + (Full 8).
    //
    // Find Position in group (0-based)
    // Start index of group?
    let group_start_idx = match lord_idx {
        0 => 5, // Sun start at 5
        1 => 9,
        2 => 12,
        3 => 16,
        4 => 19,
        5 => 23,
        6 => 26, // Rahu starts at 26 (U.Bhadra)
        7 => 2,
        _ => 0,
    };

    // Handle wrap for Rahu (26, 27, 0, 1)
    let pos_in_group = if lord_idx == 6 {
        if n_idx >= 26 {
            n_idx - 26
        } else {
            n_idx + 2
        }
    } else {
        n_idx - group_start_idx
    };

    // Stars remaining *after* current one in this group
    let stars_after = (stars_in_group - 1) - pos_in_group;
    let total_balance_years = balance_years + (stars_after as f64 * duration_per_star);

    // Generate Sequence
    // Current Lord (Balance) -> Next Lord (Full) -> ...
    let ms_per_year = 365.25 * 24.0 * 3600.0 * 1000.0;

    let mut results = Vec::new();
    let mut current_start = birth_time_ms - ((lord.dur - total_balance_years) * ms_per_year);

    // If birth time is start of life, the Dasha effectively started somewhat before birth.
    // The "Start" listed in result is usually the calendrical start.
    // First entry: Start = Birth - elapsed. End = Start + Full Duration.
    // But for the user, they care about dates.
    // Usually we list from Birth?
    // Let's output the full cycle of the current lord, but clamp start to birth?
    // No, standard practice: Show the full block, so user knows when it ends.

    let mut running_ms = current_start;

    // Create vector of indices starting from current lord_idx
    // We want to list enough to cover 100 years or so.

    for i in 0..12 {
        // 8 lords + overlap
        let idx = (lord_idx + i) % 8;
        let l_info = &lords[idx];

        let start = running_ms;
        let end = start + (l_info.dur * ms_per_year);

        results.push(AshtottariResult {
            lord: l_info.name.to_string(),
            start_ms: start,
            end_ms: end,
            duration_years: l_info.dur,
        });

        running_ms = end;
    }

    Ok(results.into_boxed_slice())
}

/// Helper to map sign index to names
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

/// Kalachakra Dasha Period information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct KalachakraPeriod {
    pub sign: u8,
    pub sign_name: String,
    pub duration_years: f64,
    pub start_ms: f64,
    pub end_ms: f64,
}

/// Kalachakra Dasha calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct KalachakraResult {
    pub is_savya: bool,
    pub nakshatra_name: String,
    pub nakshatra_pada: u8,
    pub deha_sign: u8,
    pub deha_sign_name: String,
    pub jeeva_sign: u8,
    pub jeeva_sign_name: String,
    pub periods: Vec<KalachakraPeriod>,
}

/// Calculate Kalachakra Dasha sign and period cycles
pub fn calculate_kalachakra(
    moon_long: f64,
    birth_time_ms: f64,
) -> KalachakraResult {
    // 1. Calculate Nakshatra (0-26)
    let mut normalized_moon = moon_long % 360.0;
    if normalized_moon < 0.0 {
        normalized_moon += 360.0;
    }
    let nakshatra_span = 360.0 / 27.0;
    let nak_val = normalized_moon / nakshatra_span;
    let nak_idx = nak_val.floor() as usize; // 0-26
    let fraction = nak_val - nak_val.floor(); // 0.0 to 1.0 within Nakshatra

    // Pada (1-4)
    let pada = (fraction * 4.0).floor() as u8 + 1;
    let fraction_of_pada = (fraction * 4.0) - (fraction * 4.0).floor();

    let is_savya = match nak_idx {
        0..=8 | 18..=26 => true,
        _ => false,
    };

    // Sequences of signs (1-12) for each of the 4 padas
    let sequence: Vec<u8> = if is_savya {
        match pada {
            1 => alloc::vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            2 => alloc::vec![10, 11, 12, 8, 7, 6, 4, 5, 3],
            3 => alloc::vec![2, 1, 12, 9, 10, 11, 3, 4, 5],
            _ => alloc::vec![6, 7, 8, 9, 10, 11, 12, 8, 7],
        }
    } else {
        match pada {
            1 => alloc::vec![7, 8, 12, 11, 10, 9, 8, 7, 6],
            2 => alloc::vec![5, 4, 3, 11, 10, 9, 12, 1, 2],
            3 => alloc::vec![3, 5, 4, 6, 7, 8, 12, 11, 10],
            _ => alloc::vec![9, 8, 7, 6, 5, 4, 3, 2, 1],
        }
    };

    let deha_sign = sequence[0];
    let jeeva_sign = sequence[8];

    // Total duration of the cycle
    let mut total_duration = 0.0;
    let mut sign_durations = Vec::new();
    for &sign in &sequence {
        let d = match sign {
            1 => 7.0,
            2 => 16.0,
            3 => 9.0,
            4 => 21.0,
            5 => 5.0,
            6 => 9.0,
            7 => 16.0,
            8 => 7.0,
            9 => 10.0,
            10 => 4.0,
            11 => 4.0,
            12 => 10.0,
            _ => 0.0,
        };
        total_duration += d;
        sign_durations.push(d);
    }

    // Find the starting sign in the sequence at birth
    let mut start_seq_idx = 0;
    let mut cumulative_frac = 0.0;
    let mut balance_years = 0.0;

    for i in 0..9 {
        let f_i = sign_durations[i] / total_duration;
        if fraction_of_pada >= cumulative_frac && fraction_of_pada < cumulative_frac + f_i {
            start_seq_idx = i;
            balance_years = sign_durations[i] - (fraction_of_pada - cumulative_frac) * total_duration;
            break;
        }
        cumulative_frac += f_i;
    }
    // Fallback if rounding issue
    if balance_years <= 0.0 {
        start_seq_idx = 8;
        balance_years = sign_durations[8];
    }

    // Generate period dates (e.g. 18 periods to cover full lifespan)
    let mut periods = Vec::new();
    let ms_per_year = 365.25 * 24.0 * 3600.0 * 1000.0;

    // The birth period
    let first_dur = sign_durations[start_seq_idx];
    let elapsed_ms = (first_dur - balance_years) * ms_per_year;
    let mut running_ms = birth_time_ms - elapsed_ms;

    for i in 0..18 {
        let seq_idx = (start_seq_idx + i) % 9;
        let sign = sequence[seq_idx];
        let dur = sign_durations[seq_idx];

        let start = running_ms;
        let end = start + (dur * ms_per_year);

        periods.push(KalachakraPeriod {
            sign,
            sign_name: get_sign_name(sign),
            duration_years: dur,
            start_ms: start,
            end_ms: end,
        });

        running_ms = end;
    }

    let nak_names = [
        "Ashwini", "Bharani", "Krittika", "Rohini", "Mrigashira", "Ardra",
        "Punarvasu", "Pushya", "Ashlesha", "Magha", "Purva Phalguni", "Uttara Phalguni",
        "Hasta", "Chitra", "Swati", "Vishakha", "Anuradha", "Jyeshtha", "Moola",
        "Purva Ashadha", "Uttara Ashadha", "Shravana", "Dhanishta", "Satabhisha",
        "Purva Bhadrapada", "Uttara Bhadrapada", "Revati"
    ];
    let nak_name = if nak_idx < 27 {
        nak_names[nak_idx].to_string()
    } else {
        "Unknown".to_string()
    };

    KalachakraResult {
        is_savya,
        nakshatra_name: nak_name,
        nakshatra_pada: pada,
        deha_sign,
        deha_sign_name: get_sign_name(deha_sign),
        jeeva_sign,
        jeeva_sign_name: get_sign_name(jeeva_sign),
        periods,
    }
}

