## spec 1

Here is a pure research and technical specification for a high-performance,
comprehensive Panchangam library (`lib-panchangam`). This specification focuses
strictly on astronomical accuracy, Vedic algorithms, and data modeling, ignoring
UI concerns.

### **1. Research Foundation & Mathematical Core**

The library must transition from "Table Lookup" methods (common in older apps)
to "Real-Time Vector Calculation" (Drik Ganita) to ensure precision across all
geolocations.

#### **1.1. The Ephemeris Kernel (Rust/Wasm)**

- **Requirement:** Accurate planetary positions (Geocentric Longitude &
  Latitude).
- **Standard:** **Drik Ganita** (Astronomical Precision). Sources explicitly
  state that calculations based on _Surya Siddhanta_ result in "erroneous
  moments" and strictly prefer Drik Ganita.
- **The "Gap" to Fill:** Most libraries default to the **Lahiri Ayanamsha**.
  Your library must support a **Dynamic Ayanamsha Factory**.
  - _Why:_ Calculations vary by approx. $24^{\circ}$ depending on the system.
  - _Feature:_ Support **Lahiri (Chitrapaksha)** (Govt of India standard),
    **Raman**, **Krishnamurti**, and **True Chitrapaksha** modes.
- **Implementation Strategy:**
  - **Base:** Wrap the **Swiss Ephemeris** (C library) using Rust bindings
    (`bindgen`). This provides NASA-grade planetary data.Do not write your own
    planetary physics engine. Reuse the Swiss Ephemeris (standard for NASA/JPL
    data), which is the most "established and trusted" library available.
  - **Ayanamsha (Precession):** The library must convert _Tropical_ (Sayana)
    positions to _Sidereal_ (Nirayana) positions using a subtraction algorithm.
  - **Supported Algorithms:**
    - **Lahiri (Chitrapaksha):** The standard for Indian Govt.
    - Support **Lahiri (Chitrapaksha)** (Govt of India standard), **Raman**,
      **Krishnamurti**, and **True Chitrapaksha** modes.
    - Optional configurations for specific astrological schools.
    - **Formula:** $L_{nirayana} = L_{sayana} - Ayanamsha(t)$
  - **Compilation Target:** **WebAssembly (Wasm)**. To run near-native speed in
    browsers and Node.js.
  - Use wasmbuild to compile wasm https://github.com/denoland/wasmbuild

#### **1.2. The Geo-Spatial Engine / Observer (The "Udaya" Logic/Topocentric Correction)**

- **Problem:** Existing libraries often calculate Tithi end times but fail to
  apply the **Udaya Tithi** rule (the Tithi prevailing at sunrise dictates the
  day's ritual validity) specific to the user's GPS coordinates. A Tithi ending
  at 6:53 PM in India is irrelevant to a user in Los Angeles if the _Udaya
  Tithi_ (Sunrise Tithi) rule is not applied locally,.
- **Research Insight:** Sunrise/Sunset times must account for atmospheric
  refraction and elevation, not just geometric horizon.
- **Custom Implementation:**
  - **Sunrise/Sunset Calculator:** Implement an iterative algorithm (not a
    static lookup) that accounts for **Atmospheric Refraction** and
    **Elevation**.
  - **Visual vs. Technical Distinction:** As noted in the sources, the library
    must distinguish between the **Technical Event** (e.g., Full Moon at 5:03
    AM) and the **Visual Event** (Moonrise at Dusk/Blue Hour) for UI
    visualization.
- **Algorithm:**
  - Input: `Latitude`, `Longitude`, `Altitude`, `Date`.
  - Output: `TrueSunrise`, `TrueSunset`, `Moonrise`, `Moonset`.
  - **Edge Case:** High-latitude locations (e.g., Norway) where the sun may not
    set/rise. The library must implement fallback logic (e.g., using 6:00 AM as
    a ritual proxy) or return an error.

---

### **2. Library Architecture (Modules)**

#### **Module A: `VedicChronos` (The 5-Limb Calculator)**

This module calculates the instantaneous state of the _Pancha Anga_.

**1. Tithi (Lunar Day)** Calculate $L_m - L_s$. Return not just the index (1-30)
but the **Percentage Remaining** (e.g., "Chaturdashi 77% complete") to allow for
the "progress bar" UIs mentioned in the conversation history.

- **Definition:** The angular distance between the Moon ($L_m$) and Sun ($L_s$)
  in $12^{\circ}$ increments.
- **Formula:** $Tithi Index = \lfloor \frac{L_m - L_s}{12} \rfloor + 1$
- **Precision Spec:** Return the **percentage remaining**.
  - _Source Data:_ On Jan 2, 2026, Chaturdashi is "77% Complete".
  - _Data Structure:_

    ```
    interface TithiPoint {
      index: number; // 14 (Chaturdashi)
      name: string; // "Chaturdashi"
      paksha: "Shukla" | "Krishna";
      completion: number; // 0.77
      endsAt: Date; // ISO String (calculated via iterative solver)
    }
    ```

**2. Nakshatra (Lunar Mansion)** Calculate Moon's longitude relative to the
Ayanamsha. - _Feature:_ **Transition Boundary Detection.** Return exact
timestamps for when the moon moves from _Mrigashirsha_ (Soft) to _Ardra_ (Sharp)
for lifestyle warnings.

- **Definition:** The Moon's longitude divided into 27 segments of
  $13^{\circ}20'$.
- **Research Detail:** Transitions are abrupt. On Jan 2, 2026, Moon moves from
  _Mrigashirsha_ to _Ardra_ at 8:03 PM.
- **Spec:** Implement a "Transition Detector" that finds the exact millisecond
  $L_m \pmod{13.333}$ crosses an integer boundary.

**3. Yoga (Luni-Solar Sum)**

- **Definition:** Sum of longitudes ($L_m + L_s$) divided into 27 segments.
- **Formula:**
  $Yoga Index = \lfloor \frac{L_m + L_s}{13^{\circ}20'} \rfloor + 1$

**4. Karana (Half-Tithi)**

- **Definition:** Half of a Tithi ($6^{\circ}$ increments). There are 11 cycling
  Karanas and 4 fixed ones.
- **Logic:** A Tithi has two Karanas.
  - First half ($0^{\circ}-6^{\circ}$): _Karana A_.
  - Second half ($6^{\circ}-12^{\circ}$): _Karana B_.

**5. Vara (Solar Weekday)** Solar weekday (sunrise to sunrise logic, not 12:00
AM to 12:00 AM).

- **Constraint:** In Vedic time, the day begins at **Sunrise**, not Midnight.
- **Logic:** If `CurrentTime < Sunrise`, then `Vara = PreviousWeekday`.

---

#### **Module B: `MuhuratMatrix` (Time Quality Engine)**

This module segments the day into auspicious/inauspicious windows based on the
8-part division of the "Dinamaan" (Day Duration).

**1. The 8-Part Algorithm (Rahu/Yama/Gulika)**

- **Research:** The day (Sunrise to Sunset) is divided into 8 equal parts
  (Muhurtas). The mapping depends on the weekday.
- **Formula:**
  - `DayDuration = Sunset - Sunrise`
  - `PartLength = DayDuration / 8`
  - `RahuStart = Sunrise + (PartLength * Offset)`
- **Spec:** The library must contain a static mapping table for offsets:
  - _Friday:_ Rahu (4th part), Yamaganda (5th part via Sunday logic
    correlation), Gulika (varied).

**2. Choghadiya System**

- **Research:** Divides day and night into 1.5-hour segments (approx), adjusted
  for local sunset.
- **Output:** Intervals classified as _Amrit, Shubh, Labh, Chanchal_ (Good) vs.
  _Rog, Kaal, Udveg_ (Bad).

**3. Advanced Edge Cases (from Sources)**

- **Dur Muhurtam:** Specific short windows to avoid (e.g., 12:02 PM - 12:44 PM
  on Jan 2, 2926).
- **Vinchudo:** A specific inauspicious period (Moon in Scorpio/Sagittarius
  borders).

---

#### **Module C: `HeuristicEngine` (Context & Conflicts)**

This module handles logic that requires combining astronomy with cultural/civil
rules.

**1. Udaya Tithi (Ritual Validity)**

- **Research:** A Tithi is only valid for all-day rituals (like _Vrat_) if it
  prevails at the moment of Sunrise,.
- **Algorithm:**

  ```
  fn get_ritual_tithi(date: Date, geo: Location) -> Tithi {
      let sunrise = calculate_sunrise(date, geo);
      return calculate_tithi_at(sunrise);
  }
  ```

**2. Regional Calendar Configuration**

- **Research:**
  - **Amanta:** Month ends on New Moon (South India).
  - **Purnimanta:** Month ends on Full Moon (North India).
  - _Data Point:_ Jan 2, 2026 is "Pausha" in both, but day counts differ.
- **Spec:** The library must accept a config object
  `CalendarSystem: 'Amanta' | 'Purnimanta'`.
  - _Logic:_ If `Purnimanta`, `MonthIndex = (AmantaMonthIndex - 1)` for the
    Krishna Paksha (waning phase).

**3. Planetary War (Graha Yuddha)**

- **Research:** Proximity of non-luminary planets < 1 degree,.
- **Spec:** Monitor `Venus` and `Mars` on Jan 6, 2026.
  - Input: Planetary Longitudes.
  - Logic: `abs(Long_Venus - Long_Mars) < 1.0`.
  - Output: `ConflictStatus: "Volatile"`.

**4. Civil-Spiritual Conflict Resolver**

- **Research:** Source highlights days that are "Spiritually Good" (Score 70)
  but "Civilly Closed" (SRO Holiday).
- **Spec:** The library should accept an optional "Holiday JSON" injection to
  cross-reference auspicious dates against bank holidays.

#### **Module D: The "Lifestyle & Logic" Layer (Custom & Comprehensive)**

Most libraries lack these specific "applied astrology" features mentioned in the
sources:

1. **Planetary War (Graha Yuddha) Detector:**
   - _Logic:_ Calculate if the distance between any two non-luminary planets
     (Mars, Mercury, Jupiter, Venus, Saturn) is $< 1^{\circ}$.
   - _Output:_ Warning flag for volatility (e.g., Venus-Mars war on Jan 6,
     2026).
2. **Muhurat Friction Calculator:**
   - Calculate **Rahu Kaal**, **Yamaganda**, and **Gulika Kaal** based on the
     8-part division of the solar day (Sunrise to Sunset / 8).
   - _Innovation:_ Return these as "Time Blocks" with a "Severity Score" (0.0 to
     1.0) for heatmapping UIs.
3. **Regional Switcher (Amanta/Purnimanta):**
   - _Logic:_ Allow a configuration toggle. If `Purnimanta` is true, the month
     ends on Full Moon; if `Amanta`, on New Moon. This affects the **Month
     Name** returned by the API.

---

### **3. Development Plan**

#### **Phase 1: The Rust Core (Astronomy)**

- **Goal:** Precise planetary positions.
- **Tasks:**
  1. Set up Rust project with `bindgen` to wrap `libswe` (Swiss Ephemeris).
  2. Implement `Ayanamsha` trait to allow switching between Lahiri/Raman.
  3. Create a `BodyPosition` struct returning Right Ascension, Declination, and
     Longitude.
  4. _Validation:_ Compare outputs against **NASA JPL Horizons** API data to
     ensure "Drik Ganita" accuracy.

#### **Phase 2: The Vedic Math (Panchang)**

- **Goal:** Calculate the 5 Angas.
- **Tasks:**
  1. Implement `Tithi::from_longitudes(sun_long, moon_long)`.
  2. Implement `Nakshatra::at_time(time, ayanamsha)`.
  3. **Critical:** Implement "Next Event" finders (e.g.,
     `time_until_tithi_change()`). This requires an iterative solver (e.g.,
     Brent's method) to find the exact millisecond a Tithi ends.

#### **Phase 3: The Geo-Spatial & Logic Layer**

- **Goal:** Sunrise-dependent logic.
- **Tasks:**
  1. Implement a high-precision `SunriseCalculator` (reuse `spa-rs` or similar
     Rust solar position algorithm).
  2. Build the `DailyPanchang` struct which snaps calculations to the local
     Sunrise.
  3. Implement the **Planetary War** detection logic ($< 1^{\circ}$ proximity).
  4. Implement **Civil Integration:** A basic checking function for known "SRO
     Holidays" or "Dishashool" (Travel prohibitions) based on day/direction.

#### **Phase 4: Wasm/TS Bindings**

- **Goal:** Browser usability.
- **Tasks:**
  1. Use `wasm-bindgen` to expose Rust structs to JavaScript.
  2. Create a TypeScript definition file (`.d.ts`) that is strictly typed.
  3. _Example API:_

     ```
     import { Panchang, Locations } from 'vedic-chronos';

     const la = Locations.fromLatLong(34.05, -118.24);
     const today = new Panchang(new Date('2026-01-02'), la);

     console.log(today.getUdayaTithi()); // Returns "Chaturdashi"
     console.log(today.getVisualMoonRise()); // Returns Blue Hour time
     console.log(today.isPlanetaryWarActive()); // Returns true/false
     ```

### **4. Recommended Libraries to Reuse**

- **Swiss Ephemeris (`libswe`):** For raw planetary data.
- **`chrono` (Rust):** For robust timezone and date handling.
- **`spa-rs` (Solar Position Algorithm):** For accurate sunrise/sunset/twilight
  calculations needed for "Blue Hour".

### **5. Data Output Specifications (TypeScript Interfaces) indicating roughly how consuming apps may use**

The library should output a comprehensive JSON object encompassing all research
points.

```
// Main Result Object
interface VedicTimeState {
  meta: {
    location: { lat: number; long: number; name: string };
    timestamp: string; // ISO
    ayanamshaUsed: "Lahiri" | "Raman";
  };

  // The 5 Limbs (Instantaneous)
  panchanga: {
    tithi: {
      current: number; // 14
      name: "Chaturdashi";
      completeness: number; // 0.77 (77%)
      endsAt: string; // ISO
      isUdaya: boolean; // True if this was the Tithi at sunrise
    };
    nakshatra: {
      current: number; // 5
      name: "Mrigashirsha";
      ruler: "Mars";
      quality: "Soft/Gentle"; //
      endsAt: string; // ISO
      next: "Ardra"; //
    };
    yoga: { name: "Shukla"; endsAt: string };
    karana: { name: "Gara"; endsAt: string };
    vara: "Shukrawara";
  };

  // Astronomical Phenomena
  astronomy: {
    sun: { rise: string; set: string; zodiac: "Sagittarius" };
    moon: {
      rise: string;
      set: string;
      phase: "Waxing Gibbous";
      illumination: number; // 0.98
      isVisualSupermoon: boolean; // True for Jan 3
    };
    planetaryWar: { active: boolean; planets: string[] }; //
  };

  // Time Quality (Muhurats)
  muhurat: {
    auspicious: {
      brahma: Interval; //
      abhijit: Interval; //
    };
    inauspicious: {
      rahuKaal: Interval; //
      yamaganda: Interval;
      gulika: Interval;
    };
    currentChoghadiya: { name: "Labh"; quality: "Good" }; //
  };

  // Contextual Recommendations
  lifestyle: {
    realEstateScore: number; // 0-100
    transactionStatus: "SRO_Holiday" | "Open"; //
    ritualFocus: string[]; // ["Shiva Abhishekam", "Thiruvathirai Kali"]
    fasting: { isEkadashi: boolean; isPurnima: boolean };
  };
}

interface Interval {
  start: string;
  end: string;
  durationMinutes: number;
}
```

### **4. Verification & Testing Strategy**

To ensure "State of the Art" accuracy, the library must be tested against the
specific data points found in the research:

1. **Refraction Test:** Ensure Sunrise in Los Angeles on Jan 2, 2026, is
   `06:57 AM` (Source), not the geometric sunrise.
2. **Tithi Boundary Test:** Ensure Chaturdashi ends at `18:53` (6:53 PM) in Los
   Angeles.
3. **Visual Moon Test:** Verify that the "Visual Supermoon" flag is triggered
   for the evening of Jan 3, 2026, distinct from the astronomical Full Moon on
   Jan 3 morning.
4. **Edge Case Test:** Verify that `Rahu Kaal` shifts correctly based on the day
   of the week (e.g., Friday = 11:08 AM start in Ujjain).

### **5. Accuracy Verification & benchmark strategy**

To ensure this is "most accurate," you must benchmark against:

1. **DrikPanchang.com:** For Tithi end times (Vedic standard).
2. **NASA JPL:** For physical planet positions (Scientific standard).
3. **Regional Variations:** Verify that `Amanta` vs `Purnimanta` toggles
   correctly shift the month name for the same date.

This plan creates a library that is not just a calculator, but a logic engine
capable of powering almost all use cases and UI.
