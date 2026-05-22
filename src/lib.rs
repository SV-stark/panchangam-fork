//! # Panchangam - High-Precision Vedic Calendar Library
//!
//! This library provides Drik Ganita (astronomical precision) calculations
//! for Vedic Panchangam (five-limbed calendar) including:
//! - Tithi (lunar day)
//! - Nakshatra (lunar mansion)  
//! - Yoga (luni-solar combination)
//! - Karana (half-tithi)
//! - Vara (weekday based on sunrise)
//!
//! It uses Swiss Ephemeris for planetary positions and `spa` for sunrise/sunset.

#![no_std]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use swiss_eph; // Ensure it's linked

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Include generated Swiss Ephemeris bindings
// Re-export bindings from swiss-eph
pub(crate) use swiss_eph as swe_bindings;

// Re-export Varga types for JS
pub use vedic::vargas::{
    D10Variation, D2Variation, D3Variation, D9Variation, VargaConfig, VargaPosition, VargaType,
};
pub use vedic::calendar::{MasaResult, SamvatsaraResult, RituResult};
// Re-export Shadbala/Jaimini types
pub use vedic::ashtakavarga::{
    AshtakavargaResult, PrastaraResult, ReducedAshtakavarga, Sarvashtakavarga,
};
pub use vedic::dasha::{NarayanaPeriod, NarayanaResult, DashaInfo5Levels};
pub use vedic::dasha_advanced::{AshtottariResult, KalachakraPeriod, KalachakraResult};
pub use vedic::bhava_bala::{BhavaStrength, BhavaBalaResult};
pub use vedic::vimsopaka::{VimsopakaPlanetResult, VimsopakaResult};
pub use vedic::compatibility::{AshtakootaResult, ManglikResult, CompatibilityReport};
pub use vedic::interpretations::{NadiInterpretation, ProgenyReport, CareerReport};
pub use vedic::event_timing::{TimingWindow, EventTimingReport};
pub use vedic::jaimini::{CharaDashaPeriod, JaiminiProfile, KarakaName, KarakaObject};
pub use vedic::kp::{KPLevelInfo, KPLordInfo, KPSignificators};
pub use vedic::maitri::{MaitriResult, Relationship};
pub use vedic::muhurat::{ChoghadiyaPeriod, DayMuhurats, HoraPeriod, Muhurat};
pub use vedic::shadbala::{ShadbalaProfile, ShadbalaResult};
pub use vedic::special_lagnas::SpecialLagnas;
pub use vedic::sudarshan::{SudarshanChakra, SudarshanHouseResult};
pub use vedic::transits::{DhaiyaStatus, SadeSatiStatus, VedhaResult};

// --- Modules ---
pub mod astronomy;
pub mod constants;
pub mod geo;
pub mod muhurat;
pub mod vedic;

/// Get the library version (panchangam)
#[wasm_bindgen]
pub fn get_version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
}

/// Get the underlying Swiss Ephemeris engine version
#[wasm_bindgen]
pub fn get_swisseph_version() -> String {
    let mut buf = [0i8; 256];
    unsafe {
        swe_bindings::swe_version(buf.as_mut_ptr());
    }
    let c_str = unsafe { core::ffi::CStr::from_ptr(buf.as_ptr()) };
    String::from(c_str.to_str().unwrap_or("Unknown"))
}

/// Location struct for geo-spatial calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

#[wasm_bindgen]
impl Location {
    #[wasm_bindgen(constructor)]
    pub fn new(latitude: f64, longitude: f64, altitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
        }
    }

    #[wasm_bindgen]
    pub fn from_dms(
        lat_deg: f64,
        lat_min: f64,
        lat_sec: f64,
        lat_dir: &str,
        lon_deg: f64,
        lon_min: f64,
        lon_sec: f64,
        lon_dir: &str,
    ) -> Location {
        let mut lat = lat_deg + lat_min / 60.0 + lat_sec / 3600.0;
        if lat_dir.eq_ignore_ascii_case("S") {
            lat = -lat;
        }
        let mut lon = lon_deg + lon_min / 60.0 + lon_sec / 3600.0;
        if lon_dir.eq_ignore_ascii_case("W") {
            lon = -lon;
        }
        Location {
            latitude: lat,
            longitude: lon,
            altitude: 0.0,
        }
    }

    #[wasm_bindgen]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn from_json(json_str: &str) -> Result<Location, JsValue> {
        serde_json::from_str(json_str).map_err(|e| JsValue::from_str(&alloc::format!("{}", e)))
    }
}

#[wasm_bindgen]
pub fn format_degrees_to_dms(deg: f64, is_lat: bool) -> String {
    let abs_deg = deg.abs();
    let d = abs_deg.floor();
    let m = ((abs_deg - d) * 60.0).floor();
    let s = ((abs_deg - d - m / 60.0) * 3600.0).round();
    
    let dir = if is_lat {
        if deg >= 0.0 { "N" } else { "S" }
    } else {
        if deg >= 0.0 { "E" } else { "W" }
    };
    
    alloc::format!("{}° {}' {}\" {}", d, m, s, dir)
}

/// Calculate sunrise time for a given date and location
/// Returns Unix timestamp in milliseconds
#[wasm_bindgen]
pub fn calculate_sunrise(year: i32, month: u32, day: u32, location: &Location) -> f64 {
    geo::sunrise_sunset::calculate_sunrise(
        year,
        month,
        day,
        location.latitude,
        location.longitude,
        location.altitude,
    )
}

/// Calculate sunset time for a given date and location
/// Returns Unix timestamp in milliseconds
#[wasm_bindgen]
pub fn calculate_sunset(year: i32, month: u32, day: u32, location: &Location) -> f64 {
    geo::sunrise_sunset::calculate_sunset(
        year,
        month,
        day,
        location.latitude,
        location.longitude,
        location.altitude,
    )
}

/// Calculate rise time for any planet on a given Julian Day
/// Returns Unix timestamp in milliseconds
#[wasm_bindgen]
pub fn calculate_planet_rise_time(jd: f64, planet_id: i32, location: &Location) -> Result<f64, JsValue> {
    geo::sunrise_sunset::calculate_planet_rise_set_transit(
        jd, planet_id, true, false, false,
        location.latitude, location.longitude, location.altitude,
    )
}

/// Calculate set time for any planet on a given Julian Day
/// Returns Unix timestamp in milliseconds
#[wasm_bindgen]
pub fn calculate_planet_set_time(jd: f64, planet_id: i32, location: &Location) -> Result<f64, JsValue> {
    geo::sunrise_sunset::calculate_planet_rise_set_transit(
        jd, planet_id, false, true, false,
        location.latitude, location.longitude, location.altitude,
    )
}

/// Calculate meridian transit time for any planet on a given Julian Day
/// Returns Unix timestamp in milliseconds
#[wasm_bindgen]
pub fn calculate_planet_meridian_transit(jd: f64, planet_id: i32, location: &Location) -> Result<f64, JsValue> {
    geo::sunrise_sunset::calculate_planet_rise_set_transit(
        jd, planet_id, false, false, true,
        location.latitude, location.longitude, location.altitude,
    )
}

pub use astronomy::planets::PlanetData;

/// Calculate house system (Ascendant, MC, House Cusps)
///
/// # Arguments
/// * `jd` - Julian Day
/// * `lat` - Latitude
/// * `lon` - Longitude
/// * `hsys` - House System (e.g. 'P' for Placidus, 'W' for Whole Sign)
/// * `ayan_mode` - Ayanamsha mode (-1 for tropical, others for sidereal)
#[wasm_bindgen]
pub fn calculate_houses(
    jd: f64,
    lat: f64,
    lon: f64,
    hsys: char,
    ayan_mode: i32,
) -> Result<astronomy::houses::HouseInfo, JsValue> {
    let mode = if ayan_mode < 0 {
        None
    } else {
        Some(astronomy::ayanamsha::AyanamshaMode::from_i32(ayan_mode))
    };
    astronomy::houses::calculate_houses(jd, lat, lon, hsys, mode)
}

/// Calculate Sidereal Planet Positions for all 9 core planets
///
/// # Arguments
/// * `jd` - Julian Day
/// * `ayan_mode` - Ayanamsha mode (e.g., 1 for Lahiri)
#[wasm_bindgen]
pub fn calculate_planets(jd: f64, ayan_mode: i32) -> Result<JsValue, JsValue> {
    let mode = astronomy::ayanamsha::AyanamshaMode::from_i32(ayan_mode);
    let ayan_val = astronomy::ayanamsha::get_ayanamsha(mode, jd);

    let planets = astronomy::planets::get_planet_positions_bulk(jd, ayan_val);

    Ok(serde_wasm_bindgen::to_value(&planets)?)
}

/// Calculate Sidereal Planet Positions with extended options (topocentric, true node, outer planets, asteroids)
#[wasm_bindgen]
pub fn calculate_planets_extended(
    jd: f64,
    ayan_mode: i32,
    is_topo: bool,
    topo_lat: f64,
    topo_lon: f64,
    topo_alt: f64,
    use_true_node: bool,
    include_outer: bool,
    include_asteroids: bool,
) -> Result<JsValue, JsValue> {
    let mode = astronomy::ayanamsha::AyanamshaMode::from_i32(ayan_mode);
    let ayan_val = astronomy::ayanamsha::get_ayanamsha(mode, jd);

    let planets = astronomy::planets::get_planet_positions_bulk_extended(
        jd,
        ayan_val,
        is_topo,
        topo_lat,
        topo_lon,
        topo_alt,
        use_true_node,
        include_outer,
        include_asteroids,
    );

    Ok(serde_wasm_bindgen::to_value(&planets)?)
}

/// Calculate Vimshottari Dasha details
///
/// # Arguments
/// * `moon_long` - Moon's sidereal longitude (degrees)
/// * `birth_time_ms` - Birth time (Unix ms)
/// * `current_time_ms` - Current time (Unix ms)
#[wasm_bindgen]
pub fn calculate_vimshottari(
    moon_long: f64,
    birth_time_ms: f64,
    current_time_ms: f64,
) -> vedic::dasha::DashaInfo {
    vedic::dasha::calculate_vimshottari(moon_long, birth_time_ms, current_time_ms)
}

/// Calculate Yogini Dasha details
#[wasm_bindgen]
pub fn calculate_yogini(
    moon_long: f64,
    birth_time_ms: f64,
    current_time_ms: f64,
) -> vedic::dasha::YoginiInfo {
    vedic::dasha::calculate_yogini(moon_long, birth_time_ms, current_time_ms)
}

/// Calculate Narayana Dasha details
#[wasm_bindgen]
pub fn calculate_narayana(
    lagna_sign: u8,
    planet_longs: &JsValue,
    birth_time_ms: f64,
) -> vedic::dasha::NarayanaResult {
    vedic::dasha::calculate_narayana(lagna_sign, planet_longs, birth_time_ms)
}

/// Calculate specific Varga position
/// Calculate specific Varga position
/// Calculate specific Varga position
#[wasm_bindgen]
pub fn calculate_varga(
    long: f64,
    varga_val: i32,
    config: JsValue,
) -> Result<VargaPosition, JsValue> {
    let v_type = match varga_val {
        1 => VargaType::D1,
        2 => VargaType::D2,
        3 => VargaType::D3,
        4 => VargaType::D4,
        7 => VargaType::D7,
        9 => VargaType::D9,
        10 => VargaType::D10,
        12 => VargaType::D12,
        16 => VargaType::D16,
        20 => VargaType::D20,
        24 => VargaType::D24,
        27 => VargaType::D27,
        30 => VargaType::D30,
        40 => VargaType::D40,
        45 => VargaType::D45,
        60 => VargaType::D60,
        150 => VargaType::D150,
        249 => VargaType::D249,
        _ => return Err(JsValue::from_str("Invalid Varga ID")),
    };

    let default_conf = VargaConfig::new();
    let conf: VargaConfig = if config.is_undefined() || config.is_null() {
        default_conf
    } else {
        #[derive(Deserialize)]
        struct PartialConfig {
            #[serde(default)]
            d2_method: Option<i32>,
            #[serde(default)]
            d3_method: Option<i32>,
            #[serde(default)]
            d9_method: Option<i32>,
            #[serde(default)]
            d10_method: Option<i32>,
        }

        match serde_wasm_bindgen::from_value::<PartialConfig>(config) {
            Ok(p) => VargaConfig {
                d2_method: p.d2_method.unwrap_or(default_conf.d2_method),
                d3_method: p.d3_method.unwrap_or(default_conf.d3_method),
                d9_method: p.d9_method.unwrap_or(default_conf.d9_method),
                d10_method: p.d10_method.unwrap_or(default_conf.d10_method),
            },
            Err(e) => {
                return Err(JsValue::from_str(&alloc::format!(
                    "Invalid config object: {}",
                    e
                )))
            }
        }
    };

    Ok(vedic::vargas::calculate_varga_position(long, v_type, &conf))
}

/// Calculate Shadbala for a single planet (Stub)
#[wasm_bindgen]
pub fn calculate_planet_strength(
    long: f64,
    planet_id: i32,
    jd: f64,
    ascendant: f64,
) -> ShadbalaResult {
    vedic::shadbala::calculate_planet_shadbala(long, planet_id, jd, ascendant)
}

/// Calculate Full Shadbala Profile (All 7 planets)
#[wasm_bindgen]
pub fn calculate_full_shadbala(
    planet_longs: &JsValue,
    jd: f64,
    ascendant: f64,
) -> Result<ShadbalaProfile, JsValue> {
    let data: Vec<vedic::shadbala::PlanetInput> =
        serde_wasm_bindgen::from_value(planet_longs.clone())?;
    Ok(vedic::shadbala::calculate_shadbala_profile(
        &data, jd, ascendant,
    ))
}

/// Calculate Jaimini Karakas
#[wasm_bindgen]
pub fn calculate_jaimini_karakas(
    planet_longs: &JsValue, // Array of {id, long} objects? Or flat array?
    use_8_karakas: bool,
) -> Result<Box<[KarakaObject]>, JsValue> {
    // Parse input array of tuples/objects
    // For simplicity, accept Float64Array of longitudes indexed by planet ID [0..8]?
    // Or accept flexible array.
    // Let's use simple deserialization of `Vec<(i32, f64)>`?

    let data: Vec<(i32, f64)> = serde_wasm_bindgen::from_value(planet_longs.clone())?;

    let karakas = vedic::jaimini::calculate_charakarakas(&data, use_8_karakas);
    Ok(karakas.into_boxed_slice())
}

/// Calculate Jaimini Chara Dasha Periods
#[wasm_bindgen]
pub fn calculate_chara_dasha_periods(
    planet_longs: &JsValue,
    ascendant_sign: i32,
    start_year: f64,
) -> Result<Box<[CharaDashaPeriod]>, JsValue> {
    // Parse as tuples (id, f64)
    let data: Vec<(i32, f64)> = serde_wasm_bindgen::from_value(planet_longs.clone())?;

    // Ensure all 9 planets for best results
    let periods = vedic::jaimini::calculate_chara_dasha(&data, ascendant_sign as usize, start_year);
    Ok(periods.into_boxed_slice())
}

/// Calculate Karakamsa sign (Navamsha sign of Atmakaraka)
#[wasm_bindgen]
pub fn calculate_karakamsa(planet_longs: &JsValue) -> Result<usize, JsValue> {
    let data: Vec<(i32, f64)> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    Ok(vedic::jaimini::calculate_karakamsa(&data))
}

/// Get Rashi Drishti (sign aspects) for a given sign
#[wasm_bindgen]
pub fn get_rashi_drishti(sign: usize) -> Result<JsValue, JsValue> {
    let aspects = vedic::jaimini::get_rashi_drishti(sign);
    Ok(serde_wasm_bindgen::to_value(&aspects)?)
}

/// Calculate Arudha Padas with BPHS exceptions
#[wasm_bindgen]
pub fn calculate_arudha_padas(lagna_sign: usize, planet_longs: &JsValue) -> Result<JsValue, JsValue> {
    let data: Vec<(i32, f64)> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    let padas = vedic::jaimini::calculate_arudha_padas(lagna_sign, &data);
    Ok(serde_wasm_bindgen::to_value(&padas)?)
}

/// Calculate Argalas and Vi-Argalas for a sign
#[wasm_bindgen]
pub fn calculate_argalas(sign: usize, planet_longs: &JsValue) -> Result<JsValue, JsValue> {
    let data: Vec<(i32, f64)> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    Ok(vedic::jaimini::calculate_argalas(sign, &data))
}

/// Get KP Sub Lord for a longitude
#[wasm_bindgen]
pub fn get_kp_sub_lord(longitude: f64) -> i32 {
    vedic::kp::get_sub_lord(longitude)
}

/// Get KP Sub-Sub Lord for a longitude
#[wasm_bindgen]
pub fn get_kp_sub_sub_lord(longitude: f64) -> i32 {
    vedic::kp::get_sub_sub_lord(longitude)
}

/// Calculate Sarvashtakavarga (Ashtakavarga Totals)
#[wasm_bindgen]
pub fn calculate_ashtakavarga(
    planet_longs: &JsValue, // Expecting 7 planets longitudes
    ascendant: f64,
) -> Result<Sarvashtakavarga, JsValue> {
    // Parse input. List of longitudes?
    // Or simpler: array of f64.
    // The previous Shadbala used PlanetInput objects.
    // We can reuse that or accept simpler [f64] array if user passes just longitudes.
    // Consistent with Shadbala, let's accept `Vec<PlanetInput>` or `Vec<f64>`.
    // Wait, the input logic needs to be robust.
    // Let's accept the SAME structure as Shadbala for consistency: Array of objects.
    // We extract longitudes for id 0..6.

    let data: Vec<vedic::shadbala::PlanetInput> =
        serde_wasm_bindgen::from_value(planet_longs.clone())?;

    // Sort or map 0..6
    let mut longs = vec![0.0; 7];
    for p in data {
        if p.id >= 0 && p.id <= 6 {
            longs[p.id as usize] = p.longitude;
        }
    }

    Ok(vedic::ashtakavarga::calculate_sarvashtakavarga(
        &longs, ascendant,
    ))
}

/// Calculate Binna Ashtakavarga for a single planet
#[wasm_bindgen]
pub fn calculate_binna_ashtakavarga(
    target_planet_id: i32,
    planet_longs: &JsValue,
    ascendant: f64,
) -> Result<AshtakavargaResult, JsValue> {
    let data: Vec<vedic::shadbala::PlanetInput> =
        serde_wasm_bindgen::from_value(planet_longs.clone())?;

    let mut longs = vec![0.0; 7];
    for p in data {
        if p.id >= 0 && p.id <= 6 {
            longs[p.id as usize] = p.longitude;
        }
    }

    Ok(vedic::ashtakavarga::calculate_binna_av(
        target_planet_id,
        &longs,
        ascendant,
    ))
}

/// Calculate Ashtakavarga Reductions (Trikona & Ekadhipatya)
#[wasm_bindgen]
pub fn calculate_reduced_ashtakavarga(
    bindus: &JsValue,       // Int32Array or [numbers]
    planet_longs: &JsValue, // PlanetInput array
) -> Result<ReducedAshtakavarga, JsValue> {
    let bindus_vec: Vec<i32> = serde_wasm_bindgen::from_value(bindus.clone())?;

    if bindus_vec.len() != 12 {
        return Err(JsValue::from_str("Bindus array must have 12 elements."));
    }

    let data: Vec<(i32, f64)> = serde_wasm_bindgen::from_value(planet_longs.clone())?;

    // We pass the data directly
    // We pass the data directly
    Ok(vedic::ashtakavarga::calculate_reductions(
        &bindus_vec,
        &data,
    ))
}

/// Calculate Shodhya Pinda
#[wasm_bindgen]
pub fn calculate_shodhya_pinda(
    reduced_bindus: &JsValue,
    planet_longs: &JsValue,
    target_planet_id: i32,
) -> Result<i32, JsValue> {
    let bindus_vec: Vec<i32> = serde_wasm_bindgen::from_value(reduced_bindus.clone())?;
    let data: Vec<vedic::shadbala::PlanetInput> =
        serde_wasm_bindgen::from_value(planet_longs.clone())?;

    // Extract longs
    let mut longs = vec![0.0; 7];
    for p in data {
        if p.id >= 0 && p.id <= 6 {
            longs[p.id as usize] = p.longitude;
        }
    }

    Ok(vedic::ashtakavarga::calculate_pinda(
        &bindus_vec,
        &longs,
        target_planet_id,
    ))
}

// --- Calendar ---

#[wasm_bindgen]
pub fn calculate_masa(tithi_idx: u8, sun_long: f64, is_purnimanta: bool) -> MasaResult {
    vedic::calendar::calculate_masa(tithi_idx, sun_long, 0.0, is_purnimanta)
}

#[wasm_bindgen]
pub fn calculate_masa_precise(jd: f64, is_purnimanta: bool) -> MasaResult {
    vedic::calendar::calculate_masa_precise(jd, is_purnimanta)
}

#[wasm_bindgen]
pub fn calculate_samvatsara(kali_year: i32) -> SamvatsaraResult {
    vedic::calendar::calculate_samvatsara(0.0, kali_year)
}

#[wasm_bindgen]
pub fn calculate_ritu(masa_index: u8) -> RituResult {
    vedic::calendar::calculate_ritu(masa_index)
}

/// Calculate Prastara Ashtakavarga (Detailed Grid)
#[wasm_bindgen]
pub fn calculate_prastara_ashtakavarga(
    planet_id: i32,
    planet_longs: &JsValue,
    ascendant: f64,
) -> Result<PrastaraResult, JsValue> {
    let data: Vec<vedic::shadbala::PlanetInput> =
        serde_wasm_bindgen::from_value(planet_longs.clone())?;
    let mut longs = vec![0.0; 7];
    for p in data {
        if p.id >= 0 && p.id <= 6 {
            longs[p.id as usize] = p.longitude;
        }
    }
    Ok(vedic::ashtakavarga::calculate_prastara_av(
        planet_id, &longs, ascendant,
    ))
}

/// Calculate Special Lagnas (Hora, Ghati, Sree Lagna)
#[wasm_bindgen]
pub fn calculate_special_lagnas(
    birth_jd: f64,
    sunrise_jd: f64,
    sunrise_sun_long: f64,
    lagna_long: f64,
    moon_long: f64,
) -> SpecialLagnas {
    vedic::special_lagnas::calculate_special_lagnas(
        birth_jd,
        sunrise_jd,
        sunrise_sun_long,
        lagna_long,
        moon_long,
    )
}

/// KP System Calculation
#[wasm_bindgen]
pub fn calculate_kp(long: f64) -> KPLordInfo {
    vedic::kp::calculate_kp_lords(long)
}

/// Calculate KP Significators
#[wasm_bindgen]
pub fn calculate_kp_significators(planet_longs: &JsValue, cusps: Vec<f64>) -> KPSignificators {
    vedic::kp::calculate_kp_significators(planet_longs, cusps)
}

/// Special Transit Analysis (Sade Sati)
#[wasm_bindgen]
pub fn analyze_sade_sati(moon_sign: u8, saturn_sign: u8) -> SadeSatiStatus {
    vedic::transits::check_sade_sati(moon_sign, saturn_sign)
}

/// Special Transit Analysis (Dhaiya)
#[wasm_bindgen]
pub fn analyze_dhaiya(moon_sign: u8, saturn_sign: u8) -> DhaiyaStatus {
    vedic::transits::check_dhaiya(moon_sign, saturn_sign)
}

/// Calculate Gochara Vedha
#[wasm_bindgen]
pub fn analyze_vedha(
    planet_id: i32,
    house_from_moon: i32,
    other_transits: &JsValue,
) -> VedhaResult {
    vedic::transits::calculate_vedha(planet_id, house_from_moon, other_transits)
}

/// Calculate Sudarshan Chakra
#[wasm_bindgen]
pub fn calculate_sudarshan_chakra(lagna_sign: u8, moon_sign: u8, sun_sign: u8) -> SudarshanChakra {
    vedic::sudarshan::calculate_sudarshan_chakra(lagna_sign, moon_sign, sun_sign)
}

/// Calculate Panchadha Maitri
#[wasm_bindgen]
pub fn calculate_panchadha_maitri(p1: i32, p2: i32, p1_long: f64, p2_long: f64) -> MaitriResult {
    let natural = vedic::maitri::get_natural_relationship(p1, p2);
    let temporal = vedic::maitri::get_temporal_relationship(p1_long, p2_long);
    let compound = vedic::maitri::get_compound_relationship(natural, temporal);

    MaitriResult {
        natural: natural as i32,
        temporal: temporal as i32,
        compound: compound as i32,
    }
}

// Re-export Yoga types
pub use vedic::yogas::YogaResult;

/// Find active Yogas (Planetary Combinations)
#[wasm_bindgen]
pub fn find_active_yogas(
    planet_longs: &JsValue,
    ascendant: f64,
) -> Result<Box<[YogaResult]>, JsValue> {
    let data: Vec<vedic::shadbala::PlanetInput> =
        serde_wasm_bindgen::from_value(planet_longs.clone())?;

    // Convert to slice
    let yogas = vedic::yogas::check_yogas(&data, ascendant);

    Ok(yogas.into_boxed_slice())
}

/// Calculate Julian Day number
///
/// # Arguments
/// * `year` - Year
/// * `month` - Month
/// * `day` - Day
/// * `hour` - Hour
/// * `gregflag` - Calendar flag (1 = Gregorian, 0 = Julian)
#[wasm_bindgen]
pub fn p_julday(year: i32, month: i32, day: i32, hour: f64, gregflag: i32) -> f64 {
    unsafe { swe_bindings::swe_julday(year, month, day, hour, gregflag) }
}

#[derive(Serialize)]
pub struct PlanetaryPosition {
    pub longitude: f64,
    pub latitude: f64,
    pub distance: f64,
    pub speed_long: f64,
    pub speed_lat: f64,
    pub speed_dist: f64,
}

/// Calculate planetary position (UT)
///
/// Returns simple struct with longitude, latitude, distance, speed values.
#[wasm_bindgen]
pub fn p_calc_ut(tjd_ut: f64, ipl: i32, iflag: i32) -> Result<JsValue, JsValue> {
    let mut xx = [0.0; 6];
    let mut serr = [0i8; 256];
    unsafe {
        let ret_flag =
            swe_bindings::swe_calc_ut(tjd_ut, ipl, iflag, xx.as_mut_ptr(), serr.as_mut_ptr());
        if ret_flag < 0 {
            let c_str = core::ffi::CStr::from_ptr(serr.as_ptr());
            return Err(JsValue::from_str(c_str.to_str().unwrap_or("Unknown error")));
        }
    }

    let result = PlanetaryPosition {
        longitude: xx[0],
        latitude: xx[1],
        distance: xx[2],
        speed_long: xx[3],
        speed_lat: xx[4],
        speed_dist: xx[5],
    };

    Ok(serde_wasm_bindgen::to_value(&result)?)
}

/// Calculate 5-level Vimshottari Dasha details
#[wasm_bindgen]
pub fn calculate_vimshottari_5_levels(
    moon_long: f64,
    birth_time_ms: f64,
    current_time_ms: f64,
) -> vedic::dasha::DashaInfo5Levels {
    vedic::dasha::calculate_vimshottari_5_levels(moon_long, birth_time_ms, current_time_ms)
}

/// Calculate Ashtottari Dasha details
#[wasm_bindgen]
pub fn calculate_ashtottari(
    moon_long: f64,
    birth_time_ms: f64,
    current_time_ms: f64,
) -> Result<Box<[AshtottariResult]>, JsValue> {
    vedic::dasha_advanced::calculate_ashtottari(moon_long, birth_time_ms, current_time_ms)
}

/// Calculate Kalachakra Dasha details
#[wasm_bindgen]
pub fn calculate_kalachakra(
    moon_long: f64,
    birth_time_ms: f64,
) -> vedic::dasha_advanced::KalachakraResult {
    vedic::dasha_advanced::calculate_kalachakra(moon_long, birth_time_ms)
}

/// Calculate Yoga Pinda
#[wasm_bindgen]
pub fn calculate_yoga_pinda(
    reduced_bindus: &JsValue,
    planet_longs: &JsValue,
    target_planet_id: i32,
) -> Result<i32, JsValue> {
    let bindus_vec: Vec<i32> = serde_wasm_bindgen::from_value(reduced_bindus.clone())?;
    let data: Vec<vedic::shadbala::PlanetInput> =
        serde_wasm_bindgen::from_value(planet_longs.clone())?;

    let mut longs = vec![0.0; 7];
    for p in data {
        if p.id >= 0 && p.id <= 6 {
            longs[p.id as usize] = p.longitude;
        }
    }
    Ok(vedic::ashtakavarga::calculate_yoga_pinda(
        &bindus_vec,
        &longs,
        target_planet_id,
    ))
}

/// Calculate House Pinda
#[wasm_bindgen]
pub fn calculate_house_pinda(
    house_lord_shodhya_pinda: i32,
    bindus_in_house: i32,
) -> i32 {
    vedic::ashtakavarga::calculate_house_pinda(house_lord_shodhya_pinda, bindus_in_house)
}

/// Calculate Bhava Bala (House Strength)
#[wasm_bindgen]
pub fn calculate_bhava_bala(
    lagna_sign: u8,
    planet_longs: &JsValue,
    planet_shadbalas: &JsValue,
) -> Result<vedic::bhava_bala::BhavaBalaResult, JsValue> {
    let longs: Vec<f64> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    let shadbalas: Vec<f64> = serde_wasm_bindgen::from_value(planet_shadbalas.clone())?;
    Ok(vedic::bhava_bala::calculate_bhava_bala(lagna_sign, &longs, &shadbalas))
}

/// Calculate Vimsopaka divisional chart strength
#[wasm_bindgen]
pub fn calculate_vimsopaka_bala(
    planet_longs: &JsValue,
) -> Result<vedic::vimsopaka::VimsopakaResult, JsValue> {
    let longs: Vec<f64> = serde_wasm_bindgen::from_value(planet_longs.clone())?;
    Ok(vedic::vimsopaka::calculate_vimsopaka_bala(&longs))
}

/// Calculate Marriage Ashtakoota Milan (36 Gunas)
#[wasm_bindgen]
pub fn calculate_ashtakoota_milan(
    boy_moon_long: f64,
    girl_moon_long: f64,
) -> vedic::compatibility::AshtakootaResult {
    vedic::compatibility::calculate_ashtakoota_milan(boy_moon_long, girl_moon_long)
}

/// Analyze Manglik Dosha for a chart
#[wasm_bindgen]
pub fn analyze_manglik_dosha(
    mars_long: f64,
    lagna_long: f64,
    moon_long: f64,
    venus_long: f64,
) -> vedic::compatibility::ManglikResult {
    vedic::compatibility::analyze_manglik_dosha(mars_long, lagna_long, moon_long, venus_long)
}

/// Calculate overall compatibility report between a Boy and a Girl
#[wasm_bindgen]
pub fn calculate_compatibility_report(
    boy_moon_long: f64,
    boy_mars_long: f64,
    boy_lagna_long: f64,
    boy_venus_long: f64,
    girl_moon_long: f64,
    girl_mars_long: f64,
    girl_lagna_long: f64,
    girl_venus_long: f64,
) -> vedic::compatibility::CompatibilityReport {
    vedic::compatibility::calculate_compatibility_report(
        boy_moon_long,
        boy_mars_long,
        boy_lagna_long,
        boy_venus_long,
        girl_moon_long,
        girl_mars_long,
        girl_lagna_long,
        girl_venus_long,
    )
}

/// Calculate Nadi astrology mapping
#[wasm_bindgen]
pub fn calculate_nadi_astrology(longitude: f64) -> vedic::interpretations::NadiInterpretation {
    vedic::interpretations::calculate_nadi_astrology(longitude)
}

/// Predict progeny aspects
#[wasm_bindgen]
pub fn predict_progeny(
    jupiter_long: f64,
    fifth_house_long: f64,
    fifth_lord_shadbala: f64,
    d7_jupiter_sign: u8,
) -> vedic::interpretations::ProgenyReport {
    vedic::interpretations::predict_progeny(
        jupiter_long,
        fifth_house_long,
        fifth_lord_shadbala,
        d7_jupiter_sign,
    )
}

/// Analyze career guidance and recommendations
#[wasm_bindgen]
pub fn analyze_career_guidance(
    amatyakaraka_long: f64,
    tenth_house_long: f64,
    tenth_lord_shadbala: f64,
    amatyakaraka_planet_idx: usize,
) -> vedic::interpretations::CareerReport {
    vedic::interpretations::analyze_career_guidance(
        amatyakaraka_long,
        tenth_house_long,
        tenth_lord_shadbala,
        amatyakaraka_planet_idx,
    )
}

/// Calculate event timing favorability windows
#[wasm_bindgen]
pub fn calculate_event_timing(
    birth_time_ms: f64,
    natal_moon_long: f64,
    natal_lagna_long: f64,
    natal_planet_longs: &JsValue,
    checkpoints_js: &JsValue,
) -> Result<vedic::event_timing::EventTimingReport, JsValue> {
    vedic::event_timing::calculate_event_timing(
        birth_time_ms,
        natal_moon_long,
        natal_lagna_long,
        natal_planet_longs,
        checkpoints_js,
    )
}

/// Check if Panchak is active for given Moon longitude
#[wasm_bindgen]
pub fn is_panchak_active(moon_long: f64) -> bool {
    vedic::transits::is_panchak_active(moon_long)
}

/// Predict Sade Sati cycles over a range of years
#[wasm_bindgen]
pub fn predict_sade_sati_cycles(
    natal_moon_long: f64,
    start_year: i32,
    duration_years: i32,
) -> JsValue {
    vedic::transits::predict_sade_sati_cycles(natal_moon_long, start_year, duration_years)
}

/// Calculate all planetary aspects (Parashari rules)
#[wasm_bindgen]
pub fn calculate_aspects(planet_longs: &JsValue) -> Result<JsValue, JsValue> {
    let data: Vec<(i32, f64)> = serde_wasm_bindgen::from_value(planet_longs.clone())
        .map_err(|e| JsValue::from_str(&alloc::format!("{}", e)))?;
    let aspects = vedic::aspects::calculate_aspects_impl(&data);
    serde_wasm_bindgen::to_value(&aspects)
        .map_err(|e| JsValue::from_str(&alloc::format!("{}", e)))
}


/// Calculate Prashna Arudha from seed number
#[wasm_bindgen]
pub fn calculate_prashna_arudha(seed: i32) -> usize {
    vedic::prashna::calculate_prashna_arudha(seed)
}

/// Calculate all Prashna Sphutas
#[wasm_bindgen]
pub fn calculate_prashna_sphutas(
    lagna_long: f64,
    moon_long: f64,
    sun_long: f64,
    rahu_long: f64,
    mars_long: f64,
) -> vedic::prashna::PrashnaSphutas {
    vedic::prashna::calculate_prashna_sphutas(lagna_long, moon_long, sun_long, rahu_long, mars_long)
}

/// Calculate Gulika Sphuta time (returns ms timestamp of Gulika start)
#[wasm_bindgen]
pub fn calculate_gulika_sphuta(
    sunrise_ms: f64,
    sunset_ms: f64,
    weekday_idx: u8,
    is_day: bool,
) -> f64 {
    vedic::prashna::calculate_gulika_sphuta(sunrise_ms, sunset_ms, weekday_idx, is_day)
}

/// Calculate Moon phase details
#[wasm_bindgen]
pub fn calculate_moon_phase_details(
    sun_long: f64,
    moon_long: f64,
) -> vedic::moon_phase::MoonPhaseDetails {
    vedic::moon_phase::calculate_moon_phase_details(sun_long, moon_long)
}

/// Calculate Vimshottari Dasha complete timeline with Antardashas
#[wasm_bindgen]
pub fn calculate_vimshottari_timeline(
    moon_long: f64,
    birth_time_ms: f64,
    duration_years: f64,
) -> Result<JsValue, JsValue> {
    let timeline = vedic::dasha::calculate_vimshottari_timeline(moon_long, birth_time_ms, duration_years);
    Ok(serde_wasm_bindgen::to_value(&timeline)?)
}

/// Calculate planetary combustion analysis
#[wasm_bindgen]
pub fn calculate_combustion_info(
    sun_longitude: f64,
    planet_longitude: f64,
    planet_id: i32,
    is_retrograde: bool,
) -> astronomy::combustion::CombustionInfo {
    astronomy::combustion::calculate_combustion_info(sun_longitude, planet_longitude, planet_id, is_retrograde)
}

/// Analyze Ashtakavarga transit strength
/// Returns bindu count and favorability for a transit sign
#[wasm_bindgen]
pub fn analyze_ashtakavarga_transit(
    transit_sign: usize,        // 0-11 sign that planet is transiting into
    sav_bindus: &JsValue,       // Array of 12 SAV bindu totals
) -> Result<JsValue, JsValue> {
    let bindus: Vec<i32> = serde_wasm_bindgen::from_value(sav_bindus.clone())?;
    if transit_sign >= 12 || bindus.len() != 12 {
        return Err(JsValue::from_str("Invalid input"));
    }
    let bindu_count = bindus[transit_sign];
    let is_favorable = bindu_count >= 28;

    #[derive(serde::Serialize)]
    struct TransitAnalysis { sign: usize, bindu_count: i32, is_favorable: bool, threshold: i32 }
    let result = TransitAnalysis { sign: transit_sign, bindu_count, is_favorable, threshold: 28 };
    Ok(serde_wasm_bindgen::to_value(&result)?)
}


/// Get all favorable transit signs (signs with >= 28 SAV bindus)
#[wasm_bindgen]
pub fn get_favorable_transit_signs(sav_bindus: &JsValue) -> Result<JsValue, JsValue> {
    let bindus: Vec<i32> = serde_wasm_bindgen::from_value(sav_bindus.clone())?;
    let mut favorable: Vec<usize> = Vec::new();
    for (i, b) in bindus.iter().enumerate() {
        if *b >= 28 {
            favorable.push(i);
        }
    }
    Ok(serde_wasm_bindgen::to_value(&favorable)?)
}
