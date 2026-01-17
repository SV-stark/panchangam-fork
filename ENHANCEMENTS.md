# Library Enhancement TODOs

These enhancements would allow consumers to eliminate duplicate code.

## 1. Export Swiss Ephemeris Constants

**Priority**: High\
**Impact**: Eliminates ~20 lines of hardcoded constants in consumers

```typescript
// To export from lib
export const SwePlanet = {
    Sun: 0,
    Moon: 1,
    Mercury: 2,
    Venus: 3,
    Mars: 4,
    Jupiter: 5,
    Saturn: 6,
    Uranus: 7,
    Neptune: 8,
    Pluto: 9,
    MeanNode: 10,
    TrueNode: 11, // Rahu/Ketu
};

export const SweFlag = {
    SWIEPH: 2, // Use Swiss Eph data
    SPEED: 256, // Include speed in results
    SIDEREAL: 64 * 1024,
};
```

---

## 2. Add Tithi Boundary Functions

**Priority**: High\
**Impact**: Eliminates 80+ lines of binary search in consumers

```rust
/// Find when a Tithi starts (searching backwards from jd)
pub fn tithi_start_time(jd: f64, tithi_index: u8) -> f64;

/// Find when a Tithi ends (searching forwards from jd)
pub fn tithi_end_time(jd: f64, tithi_index: u8) -> f64;
```

---

## 3. Add Ascendant/House Calculation

**Priority**: Medium\
**Impact**: Eliminates 40 lines of manual obliquity formula

```rust
/// Calculate Ascendant (Lagna) for given time and location
pub fn calculate_ascendant(jd: f64, lat: f64, lng: f64, ayan: AyanamshaMode) -> f64;

/// Calculate all house cusps using swe_houses()
pub fn calculate_houses(jd: f64, lat: f64, lng: f64, system: HouseSystem) -> HouseResult;
```

---

## 4. Export Reference Data (Optional)

**Priority**: Low\
**Impact**: Ensures consistency across all consumers

```rust
pub static NAKSHATRA_DATA: [NakshatraInfo; 27];
pub static YOGA_NAMES: [&str; 27];
pub static TITHI_NAMES: [&str; 30];
pub static VARA_NAMES: [&str; 7];
```
