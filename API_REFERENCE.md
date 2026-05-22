# Panchangam API Reference

This document provides a complete reference for the `panchangam` library (Wasm/Rust). It is designed to be easily parsed by AI agents and readable by developers.

**Version**: `v0.2.x`
**Source**: `src/lib.rs` & `src/vedic/*.rs`

---

## Table of Contents

1.  [Core & Configuration](#1-core--configuration)
2.  [Astronomy & Planets](#2-astronomy--planets)
3.  [Panchang & Calendar](#3-panchang--calendar)
4.  [Muhurats (Timing)](#4-muhurats-timing)
5.  [Horoscope Analysis](#5-horoscope-analysis)
    *   [Divisional Charts](#divisional-charts)
    *   [Shadbala (Strength)](#shadbala-strength)
    *   [Ashtakavarga](#ashtakavarga)
    *   [Yogas & Special Lagnas](#yogas--special-lagnas)
6.  [Dasha Systems](#6-dasha-systems)
7.  [Jaimini Astrology](#7-jaimini-astrology)
8.  [KP System](#8-kp-system)
9.  [Transits & Compatibility](#9-transits--compatibility)

---

## 1. Core & Configuration

### `get_version()`
Returns the library version.
*   **Returns**: `String` (e.g., "0.2.0")

### `get_swisseph_version()`
Returns the underlying Swiss Ephemeris version.
*   **Returns**: `String` (e.g., "2.10.03")

### `Location` (Struct)
Represents a geographic location.
```typescript
class Location {
  constructor(latitude: number, longitude: number, altitude: number);
  latitude: number;  // Decimal degrees (North +, South -)
  longitude: number; // Decimal degrees (East +, West -)
  altitude: number;  // Meters above sea level
}
```

### `p_julday(year, month, day, hour, gregflag)`
Calculates the Julian Day number.
*   **Params**: `year` (i32), `month` (i32), `day` (i32), `hour` (f64), `gregflag` (1=Gregorian, 0=Julian).
*   **Returns**: `f64` (Julian Day Number)

---

## 2. Astronomy & Planets

### Planet IDs
| ID | Planet | ID | Planet |
|:---|:---|:---|:---|
| 0 | Sun | 5 | Jupiter |
| 1 | Moon | 6 | Saturn |
| 2 | Mercury | 7 | Rahu (North Node) |
| 3 | Venus | 8 | Ketu (South Node) |
| 4 | Mars | | |

### `calculate_planets(jd, ayan_mode)`
Calculates sidereal positions for all 9 planets.
*   **Params**:
    *   `jd`: Julian Day (UT).
    *   `ayan_mode`: Ayanamsha mode (e.g., `1` for Lahiri).
*   **Returns**: Array of `PlanetData` objects.
    ```typescript
    interface PlanetData {
      id: number;
      name: string;
      longitude: number; // Sidereal longitude (0-360)
      latitude: number;
      distance: number;  // AU (except Moon in km usually)
      speed: number;     // Degrees per day
      is_retrograde: boolean;
    }
    ```

### `calculate_houses(jd, lat, lon, hsys, ayan_mode)`
Calculates House Cusps and Ascendant.
*   **Params**:
    *   `hsys`: House system char ('P'=Placidus, 'W'=Whole Sign, 'E'=Equal, 'B'=Bhava Chalit).
*   **Returns**: `HouseInfo` object.
    ```typescript
    interface HouseInfo {
      ascendant: number; // Ascendant degree
      mc: number;        // Midheaven degree
      cusps: number[];   // 12 house cusps (Index 0 = House 1)
    }
    ```

### `p_calc_ut(tjd_ut, ipl, iflag)`
Calculate raw planetary position (UT).
*   **Returns**: `{ longitude, latitude, distance, speed_long, speed_lat, speed_dist }`

---

## 3. Panchang & Calendar

### `calculate_daily_panchang(year, month, day, location, ayan_mode)`
Calculates the complete daily Panchang (Five Limbs).
*   **Returns**: `DailyPanchang` object.
    ```typescript
    interface DailyPanchang {
      sunrise: number; // Unix ms
      sunset: number;  // Unix ms
      tithi_index: number;    // 1-30
      tithi_name: string;
      tithi_end_time: number | null; // Unix ms
      nakshatra_index: number; // 1-27
      nakshatra_name: string;
      nakshatra_end_time: number | null;
      yoga_index: number;      // 1-27
      yoga_name: string;
      yoga_end_time: number | null;
      karana_index: number;    // 1-60
      karana_name: string;
      karana_end_time: number | null;
      vara_name: string;       // Weekday
      ascendant: number;
      ayanamsha_value: number;
      planets: PlanetData[];   // Positions at Sunrise
      muhurats: DayMuhurats;   // See Section 4
    }
    ```

### `calculate_sunrise(year, month, day, location)` / `calculate_sunset(...)`
Calculates sunrise/sunset time (Unix ms).

---

## 4. Muhurats (Timing)

### `calculate_muhurats(sunrise_ms, sunset_ms, weekday)`
Calculates daily auspicious/inauspicious periods.
*   **Params**: `weekday` (0=Sun .. 6=Sat).
*   **Returns**: `DayMuhurats`
    ```typescript
    interface DayMuhurats {
      rahu_kalam: Muhurat;
      yamaganda: Muhurat;
      gulika: Muhurat;
      brahma_muhurta: Muhurat;
      abhijit_muhurta: Muhurat;
    }
    interface Muhurat { name: string; start: number; end: number; }
    ```

### `calculate_day_choghadiya(sunrise_ms, sunset_ms, weekday)`
Calculates Day Choghadiya periods (Udveg, Chal, Labh, etc.).
*   **Returns**: Array of `ChoghadiyaPeriod`.
    ```typescript
    interface ChoghadiyaPeriod {
      name: string;
      nature: string; // "Good", "Bad", "Neutral"
      start: number;
      end: number;
    }
    ```

### `calculate_night_choghadiya(sunset_ms, next_sunrise_ms, weekday)`
Calculates Night Choghadiya periods.

### `calculate_day_hora(sunrise_ms, sunset_ms, weekday)`
Calculates Day Hora (Planetary Hours).
*   **Returns**: Array of `HoraPeriod`.
    ```typescript
    interface HoraPeriod {
      lord: string; // "Sun", "Venus", etc.
      start: number;
      end: number;
    }
    ```

### `calculate_night_hora(sunset_ms, next_sunrise_ms, weekday)`
Calculates Night Hora.

---

## 5. Horoscope Analysis

### Divisional Charts
#### `calculate_varga(long, varga_val, config)`
Calculates longitude in a divisional chart.
*   **Params**: `varga_val` (e.g. 9 for Navamsa). `config`: Optional overrides.
*   **Returns**: `{ longitude: number, sign: number }`

### Shadbala (Strength)
#### `calculate_full_shadbala(planet_longs, jd, ascendant)`
Calculates complete 6-fold strength.
*   **Params**: `planet_longs` is Array of `{ id: number, longitude: number, ... }`.
*   **Returns**: `ShadbalaProfile` (Map of planet ID to `ShadbalaResult`).

### Ashtakavarga
#### `calculate_binna_ashtakavarga(target_planet_id, planet_longs, ascendant)`
Calculates Binna Ashtakavarga (BAV) for a single planet.
*   **Returns**: `AshtakavargaResult` (`bindus`: Array of 12 integers).

#### `calculate_ashtakavarga(planet_longs, ascendant)`
Calculates Sarvashtakavarga (SAV) totals.
*   **Returns**: `Sarvashtakavarga` (`totals`: Array of 12 integers).

#### `calculate_reduced_ashtakavarga(bindus, planet_longs)`
Calculates Trikona and Ekadhipatya reductions.
*   **Returns**: `ReducedAshtakavarga` (`reduced_bindus`: [12 ints], `shodaya_pinda`: int).

#### `calculate_prastara_ashtakavarga(target_planet_id, planet_longs, ascendant)`
Calculates detailed contribution grid (Prastara).
*   **Returns**: `PrastaraResult` (`grid`: Array of 96 bytes).

### Yogas & Special Lagnas
#### `find_active_yogas(planet_longs, ascendant)`
Checks for planetary combinations.
*   **Returns**: Array of `YogaResult` (`name`, `description`, `is_active`).

#### `calculate_special_lagnas(birth_jd, sunrise_jd, sunrise_sun_long, lagna_long, moon_long)`
Calculates Hora Lagna, Ghati Lagna, etc.
*   **Returns**: `SpecialLagnas` object.

#### `calculate_sudarshan_chakra(lagna_sign, moon_sign, sun_sign)`
Returns house alignments for Sudarshan Chakra analysis.

---

## 6. Dasha Systems

### `calculate_vimshottari(moon_long, birth_time_ms, current_time_ms)`
Calculates Vimshottari Dasha (Mahadasha, Antardasha, Pratyantardasha).
*   **Returns**: `DashaInfo`

### `calculate_yogini(moon_long, birth_time_ms, current_time_ms)`
Calculates Yogini Dasha (MD, AD, PD).
*   **Returns**: `YoginiInfo`

### `calculate_narayana(lagna_sign, planet_longs, birth_time_ms)`
Calculates Narayana (Padakrama) Dasha periods.
*   **Returns**: `NarayanaResult`

### `calculate_chara_dasha_periods(planet_longs, ascendant_sign, start_year)`
Calculates Jaimini Chara Dasha periods.

---

## 7. Jaimini Astrology

### `calculate_jaimini_karakas(planet_longs, use_8_karakas)`
Calculates Chara Karakas (Atmakaraka, Amatyakaraka, etc.).
*   **Returns**: Array of `KarakaObject`.

---

## 8. KP System

### `calculate_kp(long)`
Calculates KP Lords (Sign, Star, Sub, Sub-Sub).
*   **Returns**: `KPLordInfo`.

### `calculate_kp_significators(planet_longs, cusps)`
Calculates 4-fold KP Significators.
*   **Returns**: `KPSignificators` (Array of `KPHouseSignificator`).

---

## 9. Transits & Compatibility

### `analyze_sade_sati(moon_sign, saturn_sign)`
*   **Returns**: `SadeSatiStatus` ("None", "Rising", "Peak", "Setting").

### `analyze_dhaiya(moon_sign, saturn_sign)`
*   **Returns**: `DhaiyaStatus` ("None", "4th House", "8th House").

### `analyze_vedha(planet_id, house_from_moon, other_transits)`
Checks for Gochara Vedha obstruction.
*   **Returns**: `VedhaResult`.

### `calculate_panchadha_maitri(p1, p2, p1_long, p2_long)`
Calculates 5-fold relationship.
*   **Returns**: `MaitriResult` (Natural, Temporal, Compound).
