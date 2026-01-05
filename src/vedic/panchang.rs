use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::{Location, calculate_sunrise, calculate_sunset};
use crate::vedic::{tithi, nakshatra, yoga, karana, vara, muhurat};
use crate::astronomy::{ayanamsha, solver};
use crate::astronomy::planets::{self, PlanetId};
use crate::astronomy::ayanamsha::AyanamshaMode;
use alloc::string::String;

#[derive(Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct DailyPanchang {
    // Times
    pub sunrise: f64, // Unix ms
    pub sunset: f64,  // Unix ms
    
    // Tithi
    pub tithi_index: u8,
    #[wasm_bindgen(getter_with_clone)]
    pub tithi_name: String,
    pub tithi_end_time: Option<f64>, // Unix ms

    // Nakshatra
    pub nakshatra_index: u8,
    #[wasm_bindgen(getter_with_clone)]
    pub nakshatra_name: String,
    pub nakshatra_end_time: Option<f64>, // Unix ms
    
    // Yoga
    pub yoga_index: u8,
    #[wasm_bindgen(getter_with_clone)]
    pub yoga_name: String,
    pub yoga_end_time: Option<f64>, // Unix ms

    // Vara
    #[wasm_bindgen(getter_with_clone)]
    pub vara_name: String,
    
    // Config
    pub ayanamsha_value: f64,
    
    // Muhurat
    #[wasm_bindgen(getter_with_clone)]
    pub muhurats: muhurat::DayMuhurats,
}

#[wasm_bindgen]
pub fn calculate_daily_panchang(
    year: i32, 
    month: u32, 
    day: u32, 
    location: &Location, 
    ayan_mode: i32
) -> Result<DailyPanchang, JsValue> {
    let mode = match ayan_mode {
        1 => AyanamshaMode::Lahiri,
        3 => AyanamshaMode::Raman,
        5 => AyanamshaMode::Krishnamurti,
        27 => AyanamshaMode::TrueCitra,
        _ => AyanamshaMode::Lahiri,
    };

    // 1. Calculate Sunrise
    let sunrise_ms = calculate_sunrise(year, month, day, location);
    if sunrise_ms.is_nan() {
        return Err(JsValue::from_str("Sunrise calculation failed (polar region?)"));
    }
    
    // JD at sunrise (convert ms to JD)
    // JD = (ms / 86400000) + 2440587.5
    let sunrise_jd = (sunrise_ms / 86_400_000.0) + 2440587.5;

    // 2. Ayanamsha at Sunrise
    let ayan_val = ayanamsha::get_ayanamsha(mode, sunrise_jd);

    // 3. Udaya Tithi (Tithi at Sunrise)
    let t_info = tithi::calculate_tithi(sunrise_jd);
    let tithi_idx = t_info.index;
    
    // Find Tithi End Time
    // Tithi 1 (0-12) ends at 12. Tithi 14 (156-168) ends at 168.
    // Target is simply tithi_idx * 12.0
    // If Tithi 30 (348-360), target is 360.0 (which solver handles as 0 via period if needed, 
    // but better to keep it 360 and let diff handle it).
    
    let tithi_idx_val = tithi_idx as f64;
    let target_angle = tithi_idx_val * 12.0;
    
    let search_end_jd = sunrise_jd + 1.25; 
    
    let t_end_jd = solver::find_crossing_time(
        |jd| {
            use crate::astronomy::planets;
            let sun = planets::get_planet_position_sidereal(PlanetId::Sun, jd, ayanamsha::get_ayanamsha(mode, jd));
            let moon = planets::get_planet_position_sidereal(PlanetId::Moon, jd, ayanamsha::get_ayanamsha(mode, jd));
            let mut diff = moon.longitude - sun.longitude;
            if diff < 0.0 { diff += 360.0; }
            diff
        },
        sunrise_jd,
        search_end_jd,
        target_angle,
        360.0
    );
    let tithi_end_ms = t_end_jd.map(|jd| (jd - 2440587.5) * 86_400_000.0);


    // 4. Udaya Nakshatra
    let n_info = nakshatra::calculate_nakshatra(sunrise_jd, mode);
    let n_idx = n_info.index; // 1-27
    
    // Target: next nakshatra start. 
    // Index 1 ends at 13.33. Index 5 ends at 5 * 13.33.
    let nak_len = 360.0 / 27.0;
    let target_nak_angle = (n_idx as f64) * nak_len;
    
    let n_end_jd = solver::find_crossing_time(
        |jd| {
            let moon = planets::get_planet_position_sidereal(PlanetId::Moon, jd, ayanamsha::get_ayanamsha(mode, jd));
            moon.longitude // 0-360
        },
        sunrise_jd,
        search_end_jd,
        target_nak_angle,
        360.0
    );
    let nak_end_ms = n_end_jd.map(|jd| (jd - 2440587.5) * 86_400_000.0);


    // 5. Udaya Yoga
    let y_info = yoga::calculate_yoga(sunrise_jd, mode);
    let y_idx = y_info.index; // 1-27
    let yoga_len = 360.0 / 27.0;
    let target_yoga_angle = (y_idx as f64) * yoga_len;
    
    let y_end_jd = solver::find_crossing_time(
        |jd| {
            use crate::astronomy::planets;
            let sun = planets::get_planet_position_sidereal(PlanetId::Sun, jd, ayanamsha::get_ayanamsha(mode, jd));
            let moon = planets::get_planet_position_sidereal(PlanetId::Moon, jd, ayanamsha::get_ayanamsha(mode, jd));
            let sum = moon.longitude + sun.longitude;
            sum % 360.0
        },
        sunrise_jd,
        search_end_jd,
        target_yoga_angle, 
        360.0
    );

    let yoga_end_ms = y_end_jd.map(|jd| (jd - 2440587.5) * 86_400_000.0);


    // 6. Vara (Weekday)
    let v_info = vara::calculate_vara(sunrise_jd);
    
    // 7. Muhurats
    // Need 0-6 weekday for muhurat calc. 
    // `swe_julday` takes 12h, so day changes at noon? No, julian day standard.
    // Let's use `vara::calculate_vara`'s id (0=Sunday?)
    // Actually vara module might return name. Let's check `swe_julday` -> weekday logic or use day of week from cal.
    // Assuming `vara::calculate_vara` behaves correctly.
    // For simplicity: (jd + 1.5) % 7 gives 0=Sunday.
    let weekday_idx = ((sunrise_jd + 1.5).floor() as i64 % 7) as u8;
    
    // Sunset needed for duration
    let sunset_ms = calculate_sunset(year, month, day, location);
    
    let muhurats = muhurat::calculate_muhurats(sunrise_ms, sunset_ms, weekday_idx);

    Ok(DailyPanchang {
        sunrise: sunrise_ms,
        sunset: sunset_ms,
        
        tithi_index: tithi_idx as u8,
        tithi_name: t_info.name,
        tithi_end_time: tithi_end_ms,
        
        nakshatra_index: n_idx as u8,
        nakshatra_name: n_info.name,
        nakshatra_end_time: nak_end_ms,
        
        yoga_index: y_idx as u8,
        yoga_name: y_info.name,
        yoga_end_time: yoga_end_ms,
        
        vara_name: v_info.name,
        
        ayanamsha_value: ayan_val,
        muhurats: muhurats,
    })
}
