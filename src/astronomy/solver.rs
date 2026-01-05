use crate::astronomy::planets;
use libm::fabs;

/// Tolerance for time convergence (approx 1 second in days)
const TIME_EPSILON: f64 = 1.0 / 86400.0;

/// Standard binary search to find when a geometric longitude reaches a target value.
/// 
/// `func`: A closure that takes a Julian Day and returns the value (e.g., Tithi angle).
/// `start_jd`: Search start time.
/// `end_jd`: Search end time (usually start + 1.5 days max for Tithi).
/// `target_value`: The value we want to cross.
/// `period`: The cycle length (e.g., 360.0 degrees) to handle wrap-around, or 0.0 if linear.
pub fn find_crossing_time<F>(
    mut func: F,
    start_jd: f64,
    end_jd: f64,
    target_value: f64,
    period: f64,
) -> Option<f64> 
where
    F: FnMut(f64) -> f64,
{
    let mut low = start_jd;
    let mut high = end_jd;
    let mut mid = low;

    // Check bounds
    let mut y_low = func(low);
    // Unwind wrap-around for y_low relative to target
    if period > 0.0 {
        y_low = normalize_relative(y_low, target_value, period);
    }

    // Safety check: if we are already past target, return start (or handle as "already done")
    // For Tithi end time, we assume we are currently IN the tithi, so value < target.

    for _ in 0..64 { // Max iterations to prevent infinite loop
        mid = (low + high) / 2.0;
        if (high - low) < TIME_EPSILON {
            return Some(mid);
        }

        let mut y_mid = func(mid);
        if period > 0.0 {
            y_mid = normalize_relative(y_mid, target_value, period);
        }

        if y_mid < target_value {
            low = mid;
        } else {
            high = mid;
        }
    }

    Some(mid)
}

/// Normalize val to be within [-period/2, +period/2] of ref_val
/// Useful for handling 359 -> 0 transitions close to target.
fn normalize_relative(val: f64, ref_val: f64, period: f64) -> f64 {
    let mut diff = val - ref_val;
    if diff > period / 2.0 {
        diff -= period;
    } else if diff < -period / 2.0 {
        diff += period;
    }
    ref_val + diff
}
