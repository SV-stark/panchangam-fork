//! Planetary position calculations using Swiss Ephemeris

use crate::swe_bindings;

/// Calculate Julian Day from calendar date
pub fn julian_day(year: i32, month: u32, day: u32, hour: f64) -> f64 {
    unsafe {
        swe_bindings::swe_julday(year, month as i32, day as i32, hour, swe_bindings::SE_GREG_CAL as i32)
    }
}

#[derive(Clone, Copy)]
pub enum PlanetId {
    Sun = 0,
    Moon = 1,
    Mercury = 2,
    Venus = 3, 
    Mars = 4,
    Jupiter = 5,
    Saturn = 6,
    Rahu = 11, // True Node
}

#[derive(Debug, Clone, Copy)]
pub struct PlanetPos {
    pub longitude: f64,
    pub latitude: f64,
    pub distance: f64,
    pub speed: f64,
}

pub fn get_planet_position_sidereal(planet: PlanetId, jd: f64, ayanamsha_val: f64) -> PlanetPos {
    let mut xx = [0.0; 6];
    let mut serr = [0i8; 256];
    let flags = swe_bindings::SEFLG_SWIEPH | swe_bindings::SEFLG_SPEED;
    
    unsafe {
        swe_bindings::swe_calc_ut(jd, planet as i32, flags as i32, xx.as_mut_ptr(), serr.as_mut_ptr());
    }
    
    let mut sid_lon = xx[0] - ayanamsha_val;
    if sid_lon < 0.0 { sid_lon += 360.0; }
    
    PlanetPos {
        longitude: sid_lon,
        latitude: xx[1],
        distance: xx[2],
        speed: xx[3],
    }
}


/// Calculate planetary position returning longitude
fn calculate_planet(jd: f64, planet_id: i32) -> f64 {
    let mut xx = [0.0; 6];
    let mut serr = [0i8; 256];
    let flags = swe_bindings::SEFLG_SWIEPH | swe_bindings::SEFLG_SPEED;
    
    unsafe {
        swe_bindings::swe_calc_ut(jd, planet_id, flags as i32, xx.as_mut_ptr(), serr.as_mut_ptr());
    }
    
    // xx[0] is longitude
    let mut long = xx[0];
    if long < 0.0 { long += 360.0; }
    long % 360.0
}

/// Calculate planet magnitude (brightness)
pub fn get_planet_magnitude(planet: PlanetId, jd: f64) -> f64 {
    let mut attr = [0.0; 20];
    let mut serr = [0i8; 256];
    let flags = swe_bindings::SEFLG_SWIEPH; // No speed needed for magnitude
    
    unsafe {
        swe_bindings::swe_pheno_ut(jd, planet as i32, flags as i32, attr.as_mut_ptr(), serr.as_mut_ptr());
    }
    
    // attr[4] is visual magnitude
    attr[4]
}

/// Calculate Sun's geocentric ecliptic longitude (degrees)
pub fn sun_longitude(jd: f64) -> f64 {
    calculate_planet(jd, swe_bindings::SE_SUN as i32)
}

/// Calculate Moon's geocentric ecliptic longitude (degrees)
pub fn moon_longitude(jd: f64) -> f64 {
    calculate_planet(jd, swe_bindings::SE_MOON as i32)
}

/// Calculate Moon's illumination fraction (0.0 to 1.0)
pub fn moon_illumination(jd: f64) -> f64 {
    let mut xmas = [0.0; 6];
    let mut serr = [0i8; 256];
    
    unsafe {
        swe_bindings::swe_pheno_ut(jd, swe_bindings::SE_MOON as i32, swe_bindings::SEFLG_SWIEPH as i32, xmas.as_mut_ptr(), serr.as_mut_ptr());
    }
    
    // xmas[1] is phase (illumination fraction)
    xmas[1]
}
