//! Panchadha Maitri (Five-fold Friendship) System
//!
//! Calculates Natural, Temporal, and Compound relationships between planets.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[wasm_bindgen]
pub enum Relationship {
    GreatFriend = 2,
    Friend = 1,
    Neutral = 0,
    Enemy = -1,
    GreatEnemy = -2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct MaitriResult {
    pub natural: i32,
    pub temporal: i32,
    pub compound: i32,
}

/// Natural Friendship Table (Naisargika Maitri)
/// Standard Parashara definition.
pub fn get_natural_relationship(p1: i32, p2: i32) -> Relationship {
    if p1 == p2 {
        return Relationship::Neutral;
    }

    match p1 {
        0 => {
            // Sun
            match p2 {
                1 | 2 | 4 => Relationship::Friend, // Moon, Mars, Jupiter
                3 => Relationship::Neutral,        // Mercury
                5 | 6 => Relationship::Enemy,      // Venus, Saturn
                _ => Relationship::Neutral,
            }
        }
        1 => {
            // Moon
            match p2 {
                0 | 3 => Relationship::Friend,          // Sun, Mercury
                2 | 4 | 5 | 6 => Relationship::Neutral, // Mars, Jup, Ven, Sat
                _ => Relationship::Neutral,
            }
        }
        2 => {
            // Mars
            match p2 {
                0 | 1 | 4 => Relationship::Friend, // Sun, Moon, Jup
                5 | 6 => Relationship::Neutral,    // Ven, Sat
                3 => Relationship::Enemy,          // Mercury
                _ => Relationship::Neutral,
            }
        }
        3 => {
            // Mercury
            match p2 {
                0 | 5 => Relationship::Friend,      // Sun, Venus
                2 | 4 | 6 => Relationship::Neutral, // Mars, Jup, Sat
                1 => Relationship::Enemy,           // Moon
                _ => Relationship::Neutral,
            }
        }
        4 => {
            // Jupiter
            match p2 {
                0 | 1 | 2 => Relationship::Friend, // Sun, Moon, Mars
                6 => Relationship::Neutral,        // Saturn
                3 | 5 => Relationship::Enemy,      // Mer, Ven
                _ => Relationship::Neutral,
            }
        }
        5 => {
            // Venus
            match p2 {
                3 | 6 => Relationship::Friend,  // Mer, Sat
                2 | 4 => Relationship::Neutral, // Mars, Jup
                0 | 1 => Relationship::Enemy,   // Sun, Moon
                _ => Relationship::Neutral,
            }
        }
        6 => {
            // Saturn
            match p2 {
                3 | 5 => Relationship::Friend,    // Mer, Ven
                4 => Relationship::Neutral,       // Jupiter
                0 | 1 | 2 => Relationship::Enemy, // Sun, Moon, Mars
                _ => Relationship::Neutral,
            }
        }
        _ => Relationship::Neutral,
    }
}

/// Temporal Friendship (Tatkalika Maitri)
/// Rules: Planets in 2, 3, 4, 10, 11, 12 houses from each other are friends.
/// Others are enemies.
pub fn get_temporal_relationship(p1_long: f64, p2_long: f64) -> Relationship {
    let p1_sign = (p1_long / 30.0).floor() as i32;
    let p2_sign = (p2_long / 30.0).floor() as i32;

    let diff = (p2_sign - p1_sign + 12) % 12;

    // Houses: 2, 3, 4, 10, 11, 12 (0-indexed distances from p1_sign)
    // d=1 (2nd), d=2 (3rd), d=3 (4th), d=9 (10th), d=10 (11th), d=11 (12th)
    if [1, 2, 3, 9, 10, 11].contains(&diff) {
        Relationship::Friend
    } else {
        Relationship::Enemy
    }
}

/// Compound Friendship (Panchadha Maitri)
/// Sum of Natural and Temporal.
pub fn get_compound_relationship(natural: Relationship, temporal: Relationship) -> Relationship {
    let score = (natural as i32) + (temporal as i32);
    match score {
        2 => Relationship::GreatFriend,
        1 => Relationship::Friend,
        0 => Relationship::Neutral,
        -1 => Relationship::Enemy,
        -2 => Relationship::GreatEnemy,
        _ => Relationship::Neutral,
    }
}
