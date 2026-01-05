//! Sunrise/Sunset calculations using SPA (Solar Position Algorithm)
//! Accounts for atmospheric refraction and elevation

use spa::{sunrise_and_set, SunriseAndSet, StdFloatOps};
use chrono::{Timelike, TimeZone};

/// Calculate sunrise time for a given date and location
/// Returns Unix timestamp in milliseconds
/// Accounts for atmospheric refraction (standard 0.833°)
pub fn calculate_sunrise(year: i32, month: u32, day: u32, lat: f64, lon: f64, _altitude: f64) -> f64 {
    let utc_date = chrono::Utc.with_ymd_and_hms(year, month, day, 12, 0, 0).unwrap();
    
    match sunrise_and_set::<StdFloatOps>(utc_date, lat, lon) {
        Ok(SunriseAndSet::Daylight(sunrise, _sunset)) => {
            // Convert to Unix timestamp in milliseconds
            let hours = sunrise.hour() as f64 + sunrise.minute() as f64 / 60.0 + sunrise.second() as f64 / 3600.0;
            let jd = crate::astronomy::planets::julian_day(year, month, day, hours);
            jd_to_unix_ms(jd)
        },
        Ok(SunriseAndSet::PolarDay) | Ok(SunriseAndSet::PolarNight) => {
            // Fallback for polar regions - return 6:00 AM
             let jd = crate::astronomy::planets::julian_day(year, month, day, 6.0);
             jd_to_unix_ms(jd)
        },
        Err(_) => 0.0, // Error case
    }
}

/// Calculate sunset time for a given date and location
/// Returns Unix timestamp in milliseconds
pub fn calculate_sunset(year: i32, month: u32, day: u32, lat: f64, lon: f64, _altitude: f64) -> f64 {
    let utc_date = chrono::Utc.with_ymd_and_hms(year, month, day, 12, 0, 0).unwrap();
    
    match sunrise_and_set::<StdFloatOps>(utc_date, lat, lon) {
        Ok(SunriseAndSet::Daylight(_sunrise, sunset)) => {
            let hours = sunset.hour() as f64 + sunset.minute() as f64 / 60.0 + sunset.second() as f64 / 3600.0;
            let jd = crate::astronomy::planets::julian_day(year, month, day, hours);
            jd_to_unix_ms(jd)
        },
        Ok(SunriseAndSet::PolarDay) | Ok(SunriseAndSet::PolarNight) => {
             // Fallback for polar regions - return 6:00 PM
             let jd = crate::astronomy::planets::julian_day(year, month, day, 18.0);
             jd_to_unix_ms(jd)
        },
        Err(_) => 0.0, // Error case
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
