//! Standalone Planetary Aspects Engine
//! Implements Parashari aspect rules with standard orbs.

use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// Aspect relationship between two planets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AspectRelationship {
    pub from_planet: i32,
    pub to_planet: i32,
    pub aspect_angle: f64, // degrees: 60, 90, 120, 180 etc.
    pub orb: f64,          // actual orb in degrees
    pub strength: f64,     // 0.0 to 1.0 (1.0 = exact)
    pub is_applying: bool, // faster planet approaching slower
}

/// Calculate Parashari aspects between all planets
///
/// Parashari aspect rules:
/// - All planets aspect the 7th sign (180°)
/// - Mars additionally aspects 4th (90°) and 8th (210° from sign perspective)
/// - Jupiter additionally aspects 5th (120°) and 9th (240°)
/// - Saturn additionally aspects 3rd (60°) and 10th (270°)
///
/// Orb limits: 6° for most planets, 8° for Sun/Moon/Jupiter/Saturn
pub fn calculate_aspects_impl(planet_longs: &[(i32, f64)]) -> Vec<AspectRelationship> {
    let mut aspects = Vec::new();

    for i in 0..planet_longs.len() {
        let (pid_a, long_a) = planet_longs[i];
        for j in 0..planet_longs.len() {
            if i == j {
                continue;
            }
            let (pid_b, long_b) = planet_longs[j];

            // Check each aspect angle that pid_a casts
            let aspect_angles = get_planet_aspect_angles(pid_a);

            for &angle in &aspect_angles {
                let target_long = (long_a + angle).rem_euclid(360.0);
                let diff = angular_diff(target_long, long_b);
                let orb_lim = orb_limit(pid_a);

                if diff <= orb_lim {
                    let strength = 1.0 - (diff / orb_lim);
                    aspects.push(AspectRelationship {
                        from_planet: pid_a,
                        to_planet: pid_b,
                        aspect_angle: angle,
                        orb: diff,
                        strength,
                        is_applying: false, // simplified; requires speed data
                    });
                }
            }
        }
    }

    aspects
}

/// Returns the aspect angles (in degrees) cast by a given planet (Parashari rules)
///
/// Planet IDs follow Swiss Ephemeris convention:
/// 0 = Sun, 1 = Moon, 2 = Mercury, 3 = Venus, 4 = Mars,
/// 5 = Jupiter, 6 = Saturn, 7 = Rahu, 8 = Ketu
fn get_planet_aspect_angles(planet_id: i32) -> Vec<f64> {
    // All planets cast a 7th house aspect (180°)
    let mut angles = vec![180.0];
    match planet_id {
        // Mars: additionally aspects 4th (90°) and 8th (7 houses × 30° = 210°)
        4 => {
            angles.push(90.0);
            angles.push(210.0);
        }
        // Jupiter: additionally aspects 5th (120°) and 9th (240°)
        5 => {
            angles.push(120.0);
            angles.push(240.0);
        }
        // Saturn: additionally aspects 3rd (60°) and 10th (270°)
        6 => {
            angles.push(60.0);
            angles.push(270.0);
        }
        _ => {}
    }
    angles
}

/// Minimum angular separation between two longitudes (always 0–180°)
fn angular_diff(a: f64, b: f64) -> f64 {
    let diff = (a - b).abs() % 360.0;
    if diff > 180.0 {
        360.0 - diff
    } else {
        diff
    }
}

/// Maximum orb allowed for the given planet
fn orb_limit(planet_id: i32) -> f64 {
    match planet_id {
        0 | 1 | 5 | 6 => 8.0, // Sun, Moon, Jupiter, Saturn — wider orb
        _ => 6.0,
    }
}
