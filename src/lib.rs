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
use alloc::string::String;
use alloc::vec::Vec;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

// Include generated Swiss Ephemeris bindings
pub(crate) mod swe_bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    
    // Manually declare functions that bindgen missed
    extern "C" {
        pub fn swe_version(s: *mut i8) -> *mut i8;
        pub fn swe_julday(year: i32, month: i32, day: i32, hour: f64, gregflag: i32) -> f64;
        pub fn swe_calc_ut(tjd_ut: f64, ipl: i32, iflag: i32, xx: *mut f64, serr: *mut i8) -> i32;
        pub fn swe_pheno_ut(tjd_ut: f64, ipl: i32, iflag: i32, attr: *mut f64, serr: *mut i8) -> i32;
        pub fn swe_set_sid_mode(sid_mode: i32, t0: f64, ayan_t0: f64);
        pub fn swe_get_ayanamsa_ut(tjd_ut: f64) -> f64;
        pub fn swe_revjul(tjd: f64, gregflag: i32, year: *mut i32, month: *mut i32, day: *mut i32, hour: *mut f64);
        pub fn swe_fixstar_ut(star: *mut i8, tjd_ut: f64, iflag: i32, xx: *mut f64, serr: *mut i8) -> i32;
        pub fn swe_sidtime(tjd_ut: f64) -> f64;
        pub fn swe_get_planet_name(ipl: i32, spname: *mut i8) -> *mut i8;
        pub fn swe_set_topo(geolon: f64, geolat: f64, geoalt: f64);
    }
}



pub mod astronomy;
pub mod vedic;
pub mod geo;
pub mod muhurat;
pub mod raw;

/// Get the library version from Swiss Ephemeris
#[wasm_bindgen]
pub fn get_version() -> String {
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
        Self { latitude, longitude, altitude }
    }
}

/// Calculate sunrise time for a given date and location
/// Returns Unix timestamp in milliseconds
#[wasm_bindgen]
pub fn calculate_sunrise(year: i32, month: u32, day: u32, location: &Location) -> f64 {
    geo::sunrise_sunset::calculate_sunrise(year, month, day, location.latitude, location.longitude, location.altitude)
}

/// Calculate sunset time for a given date and location
/// Returns Unix timestamp in milliseconds
#[wasm_bindgen]
pub fn calculate_sunset(year: i32, month: u32, day: u32, location: &Location) -> f64 {
    geo::sunrise_sunset::calculate_sunset(year, month, day, location.latitude, location.longitude, location.altitude)
}

// --- Shims for C functions in Wasm ---

// --- Shims for C functions in Wasm ---
// We use libm crate to avoid potential recursion if Rust's f64 methods link back to these symbols

#[no_mangle]
pub unsafe extern "C" fn sin(x: f64) -> f64 { libm::sin(x) }
#[no_mangle]
pub unsafe extern "C" fn cos(x: f64) -> f64 { libm::cos(x) }
#[no_mangle]
pub unsafe extern "C" fn tan(x: f64) -> f64 { libm::tan(x) }
#[no_mangle]
pub unsafe extern "C" fn asin(x: f64) -> f64 { libm::asin(x) }
#[no_mangle]
pub unsafe extern "C" fn acos(x: f64) -> f64 { libm::acos(x) }
#[no_mangle]
pub unsafe extern "C" fn atan(x: f64) -> f64 { libm::atan(x) }
#[no_mangle]
pub unsafe extern "C" fn atan2(y: f64, x: f64) -> f64 { libm::atan2(y, x) }
#[no_mangle]
pub unsafe extern "C" fn sqrt(x: f64) -> f64 { libm::sqrt(x) }
#[no_mangle]
pub unsafe extern "C" fn log(x: f64) -> f64 { libm::log(x) }
#[no_mangle]
pub unsafe extern "C" fn exp(x: f64) -> f64 { libm::exp(x) }
#[no_mangle]
pub unsafe extern "C" fn pow(x: f64, y: f64) -> f64 { libm::pow(x, y) }
#[no_mangle]
pub unsafe extern "C" fn fabs(x: f64) -> f64 { libm::fabs(x) }
#[no_mangle]
pub unsafe extern "C" fn ceil(x: f64) -> f64 { libm::ceil(x) }
#[no_mangle]
pub unsafe extern "C" fn floor(x: f64) -> f64 { libm::floor(x) }
#[no_mangle]
pub unsafe extern "C" fn fmod(x: f64, y: f64) -> f64 { libm::fmod(x, y) }
#[no_mangle]
pub unsafe extern "C" fn log10(x: f64) -> f64 { libm::log10(x) }




// Memory allocation shims
// C free() doesn't pass size, but Rust dealloc() requires it.
// We store the size in an 8-byte header before the pointer.

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    let header_size = 8;
    let total_size = size + header_size;
    let layout = alloc::alloc::Layout::from_size_align_unchecked(total_size, 8);
    let ptr = alloc::alloc::alloc(layout);
    
    if ptr.is_null() { return core::ptr::null_mut(); }
    
    // Store size in header
    *(ptr as *mut usize) = total_size;
    
    // Return pointer after header
    ptr.add(header_size)
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    if ptr.is_null() { return; }
    
    let header_size = 8;
    let real_ptr = ptr.sub(header_size);
    
    // Read size from header
    let total_size = *(real_ptr as *const usize);
    let layout = alloc::alloc::Layout::from_size_align_unchecked(total_size, 8);
    
    alloc::alloc::dealloc(real_ptr, layout);
}

#[no_mangle]
pub unsafe extern "C" fn calloc(nmemb: usize, size: usize) -> *mut u8 {
    let total_size = nmemb * size;
    let ptr = malloc(total_size);
    if !ptr.is_null() {
        core::ptr::write_bytes(ptr, 0, total_size);
    }
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    if ptr.is_null() {
        return malloc(new_size);
    }
    if new_size == 0 {
        free(ptr);
        return core::ptr::null_mut();
    }
    
    let header_size = 8;
    let real_ptr = ptr.sub(header_size);
    let old_total_size = *(real_ptr as *const usize);
    let old_user_size = old_total_size - header_size;
    
    // Simple implementation: malloc new, copy, free old
    // Optimizing this would require using realloc on the real_ptr but header handling gets tricky
    let new_ptr = malloc(new_size);
    if !new_ptr.is_null() {
        let copy_size = if old_user_size < new_size { old_user_size } else { new_size };
        core::ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);
        free(ptr);
    }
    new_ptr
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    core::ptr::copy_nonoverlapping(src, dest, n);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    core::ptr::write_bytes(s, c as u8, n);
    s
}

#[no_mangle]
pub unsafe extern "C" fn strcpy(dest: *mut u8, src: *const u8) -> *mut u8 {
    let mut i = 0;
    loop {
        let c = *src.add(i);
        *dest.add(i) = c;
        if c == 0 { break; }
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn strncpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        let c = *src.add(i);
        *dest.add(i) = c;
        if c == 0 {
            // Pad with zeros
            while i < n {
                *dest.add(i) = 0;
                i += 1;
            }
            break;
        }
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn strlen(s: *const i8) -> usize {
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    len
}

#[no_mangle]
pub unsafe extern "C" fn strcmp(s1: *const i8, s2: *const i8) -> i32 {
    let mut i = 0;
    loop {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        if c1 != c2 { return (c1 - c2) as i32; }
        if c1 == 0 { return 0; }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn strncmp(s1: *const i8, s2: *const i8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let c1 = *s1.add(i);
        let c2 = *s2.add(i);
        if c1 != c2 { return (c1 - c2) as i32; }
        if c1 == 0 { return 0; }
        i += 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn strstr(haystack: *const i8, needle: *const i8) -> *const i8 {
    // Naive implementation
    let needle_len = strlen(needle);
    if needle_len == 0 { return haystack; }
    let mut h = haystack;
    while *h != 0 {
        if strncmp(h, needle, needle_len) == 0 {
            return h;
        }
        h = h.add(1);
    }
    core::ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn abs(j: i32) -> i32 { j.abs() }
#[no_mangle]
pub unsafe extern "C" fn labs(j: i64) -> i64 { j.abs() }

#[no_mangle]
pub unsafe extern "C" fn atof(_str: *const i8) -> f64 { 0.0 } // Simplification
#[no_mangle]
pub unsafe extern "C" fn atoi(_str: *const i8) -> i32 { 0 } // Simplification
#[no_mangle]
pub unsafe extern "C" fn atol(_str: *const i8) -> i64 { 0 } // Simplification

// Stub file functions as files are not supported in wasm
#[no_mangle]
pub unsafe extern "C" fn fopen(_filename: *const i8, _mode: *const i8) -> *mut u8 { core::ptr::null_mut() }
#[no_mangle]
pub unsafe extern "C" fn fclose(_stream: *mut u8) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn fseek(_stream: *mut u8, _offset: i64, _whence: i32) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn ftell(_stream: *mut u8) -> i64 { 0 }
#[no_mangle]
pub unsafe extern "C" fn fread(_ptr: *mut u8, _size: usize, _nmemb: usize, _stream: *mut u8) -> usize { 0 }
#[no_mangle]
pub unsafe extern "C" fn fwrite(_ptr: *const u8, _size: usize, nmemb: usize, _stream: *mut u8) -> usize { nmemb } // Stub: pretend we wrote everything

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    core::ptr::copy(src, dest, n); // Handles overlapping
    dest
}
// Note: fprintf, sprintf, printf, sscanf cannot be implemented as variadic in stable Rust
// These are stubbed as unused - Swiss Ephemeris should not call them in Wasm environment
#[no_mangle]
pub unsafe extern "C" fn fgets(_str: *mut i8, _n: i32, _stream: *mut u8) -> *mut i8 { core::ptr::null_mut() }
#[no_mangle]
pub unsafe extern "C" fn fflush(_stream: *mut u8) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn exit(_status: i32) { panic!("exit called") }

#[no_mangle]
pub unsafe extern "C" fn isspace(c: i32) -> i32 {
    if (c as u8 as char).is_whitespace() { 1 } else { 0 }
}
#[no_mangle]
pub unsafe extern "C" fn isdigit(c: i32) -> i32 {
    if (c as u8 as char).is_ascii_digit() { 1 } else { 0 }
}
#[no_mangle]
pub unsafe extern "C" fn isalpha(c: i32) -> i32 {
    if (c as u8 as char).is_ascii_alphabetic() { 1 } else { 0 }
}
#[no_mangle]
pub unsafe extern "C" fn isalnum(c: i32) -> i32 {
    if (c as u8 as char).is_ascii_alphanumeric() { 1 } else { 0 }
}
#[no_mangle]
pub unsafe extern "C" fn isupper(c: i32) -> i32 {
    if (c as u8 as char).is_ascii_uppercase() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn fseeko(_stream: *mut u8, _offset: i64, _whence: i32) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn ftello(_stream: *mut u8) -> i64 { 0 }

#[no_mangle]
pub unsafe extern "C" fn strpbrk(s: *const i8, accept: *const i8) -> *mut i8 {
    let mut s_ptr = s;
    while *s_ptr != 0 {
        let mut a_ptr = accept;
        while *a_ptr != 0 {
            if *s_ptr == *a_ptr {
                return s_ptr as *mut i8;
            }
            a_ptr = a_ptr.add(1);
        }
        s_ptr = s_ptr.add(1);
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn memchr(s: *const u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        if *s.add(i) == c as u8 {
            return s.add(i) as *mut u8;
        }
        i += 1;
    }
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn strdup(s: *const i8) -> *mut i8 {
    let len = strlen(s);
    let layout = alloc::alloc::Layout::from_size_align_unchecked(len + 1, 8);
    let ptr = alloc::alloc::alloc(layout) as *mut i8;
    if !ptr.is_null() {
        strcpy(ptr as *mut u8, s as *const u8);
    }
    ptr
}


#[no_mangle]
pub unsafe extern "C" fn strcat(dest: *mut i8, src: *const i8) -> *mut i8 {
    let len = strlen(dest);
    strcpy(dest.add(len) as *mut u8, src as *const u8);
    dest
}

#[no_mangle]
pub unsafe extern "C" fn strchr(s: *const i8, c: i32) -> *mut i8 {
    let mut s_ptr = s;
    loop {
        if *s_ptr == c as i8 { return s_ptr as *mut i8; }
        if *s_ptr == 0 { return core::ptr::null_mut(); }
        s_ptr = s_ptr.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn strrchr(s: *const i8, c: i32) -> *mut i8 {
    let mut last = core::ptr::null_mut();
    let mut s_ptr = s;
    loop {
        if *s_ptr == c as i8 { last = s_ptr as *mut i8; }
        if *s_ptr == 0 { return last; }
        s_ptr = s_ptr.add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn tolower(c: i32) -> i32 {
    (c as u8).to_ascii_lowercase() as i32
}

// dlfcn stubs
#[no_mangle]
pub unsafe extern "C" fn dlopen(_filename: *const i8, _flag: i32) -> *mut u8 { core::ptr::null_mut() }
#[no_mangle]
pub unsafe extern "C" fn dlerror() -> *mut i8 { core::ptr::null_mut() }
#[no_mangle]
pub unsafe extern "C" fn dlsym(_handle: *mut u8, _symbol: *const i8) -> *mut u8 { core::ptr::null_mut() }
#[no_mangle]
pub unsafe extern "C" fn dlclose(_handle: *mut u8) -> i32 { 0 }
#[no_mangle]
pub unsafe extern "C" fn dladdr(_addr: *const u8, _info: *mut u8) -> i32 { 0 }

// stdlib stubs
#[no_mangle]
pub unsafe extern "C" fn getenv(_name: *const i8) -> *mut i8 { core::ptr::null_mut() }
#[no_mangle]
pub unsafe extern "C" fn qsort(_base: *mut u8, _nmemb: usize, _size: usize, _compar: *const u8) { 
    // No-op for now. If fixed stars sorting is vital, we need a real implementation but it requires C callback support.
}
#[no_mangle]
pub unsafe extern "C" fn bsearch(_key: *const u8, _base: *const u8, _nmemb: usize, _size: usize, _compar: *const u8) -> *mut u8 {
    // Return null (not found) for now.
    core::ptr::null_mut()
}

// stdio stubs
#[no_mangle]
pub unsafe extern "C" fn rewind(_stream: *mut u8) { }

// unistd stubs
#[no_mangle]
pub unsafe extern "C" fn readlink(_path: *const i8, _buf: *mut i8, _bufsiz: usize) -> isize { -1 }

// sys/stat stubs
#[no_mangle]
pub unsafe extern "C" fn stat(_path: *const i8, _buf: *mut u8) -> i32 { -1 }
