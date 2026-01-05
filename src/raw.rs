use wasm_bindgen::prelude::*;
use crate::swe_bindings;
use js_sys::{Object, Reflect};
use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::CStr;

/// Swiss Ephemeris Raw API
/// 
/// This module exposes a 1:1 mapping of the C API where possible,
/// adapting pointers to JS Objects/Arrays for ease of use.
/// Function names are prefixed with `js_` internally to avoid linker collisions
/// with the C library, but exported with original names via `js_name`.

// --- Core Calculation ---

/// Calculate planetary position (UT)
/// Returns: { longitude, latitude, distance, speed_long, speed_lat, speed_dist, rc_flags }
/// Throws string error on failure
#[wasm_bindgen(js_name = raw_swe_calc_ut)]
pub fn js_swe_calc_ut(tjd_ut: f64, ipl: i32, iflag: i32) -> Result<JsValue, JsValue> {
    let mut xx = [0.0; 6];
    let mut serr = [0i8; 256];
    
    let rc = unsafe {
        swe_bindings::swe_calc_ut(tjd_ut, ipl, iflag, xx.as_mut_ptr(), serr.as_mut_ptr())
    };
    
    if rc < 0 {
        let c_str = unsafe { CStr::from_ptr(serr.as_ptr()) };
        return Err(JsValue::from_str(&c_str.to_string_lossy()));
    }

    let obj = Object::new();
    Reflect::set(&obj, &"longitude".into(), &xx[0].into())?;
    Reflect::set(&obj, &"latitude".into(), &xx[1].into())?;
    Reflect::set(&obj, &"distance".into(), &xx[2].into())?;
    Reflect::set(&obj, &"speed_long".into(), &xx[3].into())?;
    Reflect::set(&obj, &"speed_lat".into(), &xx[4].into())?;
    Reflect::set(&obj, &"speed_dist".into(), &xx[5].into())?;
    Reflect::set(&obj, &"rc_flags".into(), &rc.into())?;
    
    Ok(obj.into())
}

/// Calculate fixed star position
#[wasm_bindgen(js_name = raw_swe_fixstar_ut)]
pub fn js_swe_fixstar_ut(star: &str, tjd_ut: f64, iflag: i32) -> Result<JsValue, JsValue> {
    let mut xx = [0.0; 6];
    let mut serr = [0i8; 256];
    
    // CString handling in no_std is tricky, we'll manually copy bytes
    let mut star_cwd = [0i8; 80]; // Max star name usually 40
    for (i, b) in star.bytes().enumerate() {
        if i >= 79 { break; }
        star_cwd[i] = b as i8;
    }
    
    let rc = unsafe {
        swe_bindings::swe_fixstar_ut(star_cwd.as_mut_ptr(), tjd_ut, iflag, xx.as_mut_ptr(), serr.as_mut_ptr())
    };
    
    if rc < 0 {
        let c_str = unsafe { CStr::from_ptr(serr.as_ptr()) };
        return Err(JsValue::from_str(&c_str.to_string_lossy()));
    }
    
    // Return same structure as planets
    let obj = Object::new();
    Reflect::set(&obj, &"name".into(), &JsValue::from_str(star))?; // Echo name
    Reflect::set(&obj, &"longitude".into(), &xx[0].into())?;
    Reflect::set(&obj, &"latitude".into(), &xx[1].into())?;
    Reflect::set(&obj, &"distance".into(), &xx[2].into())?;
    Reflect::set(&obj, &"rc_flags".into(), &rc.into())?;
    
    Ok(obj.into())
}

// --- Date & Time ---

#[wasm_bindgen(js_name = raw_swe_sidtime)]
pub fn js_swe_sidtime(tjd_ut: f64) -> f64 {
    unsafe { swe_bindings::swe_sidtime(tjd_ut) }
}

#[wasm_bindgen(js_name = raw_swe_julday)]
pub fn js_swe_julday(year: i32, month: i32, day: i32, hour: f64, gregflag: i32) -> f64 {
    unsafe {
        swe_bindings::swe_julday(year, month, day, hour, gregflag)
    }
}

#[wasm_bindgen(js_name = raw_swe_revjul)]
pub fn js_swe_revjul(tjd: f64, gregflag: i32) -> JsValue {
    let mut year = 0;
    let mut month = 0;
    let mut day = 0;
    let mut hour = 0.0;
    
    unsafe {
        swe_bindings::swe_revjul(tjd, gregflag, &mut year, &mut month, &mut day, &mut hour);
    }
    
    let obj = Object::new();
    Reflect::set(&obj, &"year".into(), &year.into()).unwrap();
    Reflect::set(&obj, &"month".into(), &month.into()).unwrap();
    Reflect::set(&obj, &"day".into(), &day.into()).unwrap();
    Reflect::set(&obj, &"hour".into(), &hour.into()).unwrap();
    
    obj.into()
}

// --- Phenomena ---

/// Calculate phenomena (phase, eclipse, etc.)
/// Returns: { phase_angle, phase, elongation, diameter_app, magnitude }
#[wasm_bindgen(js_name = raw_swe_pheno_ut)]
pub fn js_swe_pheno_ut(tjd_ut: f64, ipl: i32, iflag: i32) -> Result<JsValue, JsValue> {
    let mut attr = [0.0; 20];
    let mut serr = [0i8; 256];
    
    let rc = unsafe {
        swe_bindings::swe_pheno_ut(tjd_ut, ipl, iflag, attr.as_mut_ptr(), serr.as_mut_ptr())
    };

    if rc < 0 {
        let c_str = unsafe { CStr::from_ptr(serr.as_ptr()) };
        return Err(JsValue::from_str(&c_str.to_string_lossy()));
    }
    
    let obj = Object::new();
    Reflect::set(&obj, &"phase_angle".into(), &attr[0].into())?;
    Reflect::set(&obj, &"phase".into(), &attr[1].into())?;
    Reflect::set(&obj, &"elongation".into(), &attr[2].into())?;
    Reflect::set(&obj, &"diameter_app".into(), &attr[3].into())?;
    Reflect::set(&obj, &"magnitude".into(), &attr[4].into())?;
    
    Ok(obj.into())
}

// --- Auxiliary ---

#[wasm_bindgen(js_name = raw_swe_get_planet_name)]
pub fn js_swe_get_planet_name(ipl: i32) -> String {
    let mut spname = [0i8; 80];
    unsafe {
        swe_bindings::swe_get_planet_name(ipl, spname.as_mut_ptr());
    }
    let c_str = unsafe { CStr::from_ptr(spname.as_ptr()) };
    String::from(c_str.to_str().unwrap_or("?"))
}

#[wasm_bindgen(js_name = raw_swe_set_topo)]
pub fn js_swe_set_topo(lon: f64, lat: f64, alt: f64) {
    unsafe { swe_bindings::swe_set_topo(lon, lat, alt); }
}

#[wasm_bindgen(js_name = raw_swe_set_sid_mode)]
pub fn js_swe_set_sid_mode(sid_mode: i32, t0: f64, ayan_t0: f64) {
    unsafe { swe_bindings::swe_set_sid_mode(sid_mode, t0, ayan_t0); }
}

#[wasm_bindgen(js_name = raw_swe_get_ayanamsa_ut)]
pub fn js_swe_get_ayanamsa_ut(tjd_ut: f64) -> f64 {
    unsafe { swe_bindings::swe_get_ayanamsa_ut(tjd_ut) }
}
