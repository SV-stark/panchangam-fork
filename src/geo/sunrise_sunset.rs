//! Sunrise/Sunset calculations using Swiss Ephemeris
//! Accounts for atmospheric refraction and elevation

use swiss_eph::safe::{self, RiseTransFlags, Planet, GeoPos};

/// Calculate sunrise time for a given date and location
/// Returns Unix timestamp in milliseconds
/// Accounts for atmospheric refraction
pub fn calculate_sunrise(year: i32, month: u32, day: u32, lat: f64, lon: f64, altitude: f64) -> f64 {
    // Start search from midnight to find today's sunrise
    let jd_start = safe::julday(year, month as i32, day as i32, 0.0);
    
    // Standard sunrise (with refraction)
    let flags = RiseTransFlags::new().with_rise();
    let geopos = GeoPos { longitude: lon, latitude: lat, altitude };
    
    match safe::rise_trans(jd_start, Planet::Sun, None, geopos, flags) {
        Ok(jd_rise) => jd_to_unix_ms(jd_rise),
        Err(_) => {
            // Fallback for polar regions - return 6:00 AM
            // Consistent with previous implementation behavior
            let jd_fallback = safe::julday(year, month as i32, day as i32, 6.0);
            jd_to_unix_ms(jd_fallback)
        }
    }
}

/// Calculate sunset time for a given date and location
/// Returns Unix timestamp in milliseconds
pub fn calculate_sunset(year: i32, month: u32, day: u32, lat: f64, lon: f64, altitude: f64) -> f64 {
    // Start search from noon to find today's sunset (usually after noon)
    // If we start at midnight, we might find yesterday's sunset if it was late? 
    // No, `rise_trans` searches forward. 
    // But safely, noon is good.
    let jd_start = safe::julday(year, month as i32, day as i32, 12.0);
    
    let flags = RiseTransFlags::new().with_set();
    let geopos = GeoPos { longitude: lon, latitude: lat, altitude };
    
    match safe::rise_trans(jd_start, Planet::Sun, None, geopos, flags) {
        Ok(jd_set) => jd_to_unix_ms(jd_set),
        Err(_) => {
            // Fallback for polar regions - return 6:00 PM
            let jd_fallback = safe::julday(year, month as i32, day as i32, 18.0);
            jd_to_unix_ms(jd_fallback)
        }
    }
}

/// Calculate rise, set, or meridian transit for any planet
/// Returns Unix timestamp in milliseconds
pub fn calculate_planet_rise_set_transit(
    jd: f64,
    planet_id: i32,
    is_rise: bool,
    is_set: bool,
    is_transit: bool,
    lat: f64,
    lon: f64,
    altitude: f64,
) -> Result<f64, wasm_bindgen::JsValue> {
    let planet = match planet_id {
        0 => Planet::Sun,
        1 => Planet::Moon,
        2 => Planet::Mercury,
        3 => Planet::Venus,
        4 => Planet::Mars,
        5 => Planet::Jupiter,
        6 => Planet::Saturn,
        _ => return Err(wasm_bindgen::JsValue::from_str("Unsupported planet ID for rise/set")),
    };

    let mut flags = RiseTransFlags::new();
    if is_rise {
        flags = flags.with_rise();
    } else if is_set {
        flags = flags.with_set();
    } else if is_transit {
        flags = flags.with_mtransit();
    } else {
        return Err(wasm_bindgen::JsValue::from_str("Must select either rise, set, or transit"));
    }

    let geopos = GeoPos { longitude: lon, latitude: lat, altitude };
    match safe::rise_trans(jd, planet, None, geopos, flags) {
        Ok(jd_res) => Ok(jd_to_unix_ms(jd_res)),
        Err(e) => Err(wasm_bindgen::JsValue::from_str(&alloc::format!("Rise/set calculation error: {:?}", e))),
    }
}

/// Convert Julian Day to Unix timestamp in milliseconds
fn jd_to_unix_ms(jd: f64) -> f64 {
    // Unix epoch = JD 2440587.5
    (jd - 2440587.5) * 86400000.0
}

/// Convert Unix timestamp to Julian Day
pub fn unix_ms_to_jd(unix_ms: f64) -> f64 {
    unix_ms / 86400000.0 + 2440587.5
}
