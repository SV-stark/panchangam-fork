# Comprehensive Feature Comparison: Panchangam (Wasm) vs. Jyotish Flutter Library (Fork)

This document provides a highly detailed, tabular-wise feature comparison between two elite Vedic astrology repositories:
1. **Panchangam (Wasm)**: A high-performance, Rust-based Vedic astrology and calendar engine compiled to WebAssembly (JS/TS interfaces).
2. **Jyotish Flutter Library (Fork)**: A feature-rich, object-oriented Dart library designed for cross-platform Flutter mobile, desktop, and web applications.

---

## 🏛️ Section 1: Architectural & Platform Paradigms

| Feature/Metric | Panchangam (Wasm) (`panchangam-fork`) | Jyotish Flutter Library (`jyotish-flutter-library-fork`) | Comparison & Notes |
| :--- | :--- | :--- | :--- |
| **Primary Language** | **Rust** (compiled to `.wasm`) | **Dart** | Rust ensures bare-metal, high-performance concurrency. Dart provides rich, object-oriented ecosystem tailored for Flutter widgets. |
| **Platforms Supported** | Web browsers, Node.js, Deno, Bun, Cloudflare Workers, Edge runtimes. | Android, iOS, Windows, macOS, Linux, Flutter Web. | **Panchangam** is built for ultra-scalable edge/web backends. **Jyotish** is built for native mobile/desktop client apps. |
| **Swiss Ephemeris Bindings** | High-performance Rust bindings (`swiss-eph`) statically/stably compiled directly into the WASM binary. | Native C/C++ FFI bindings using dynamic libraries (e.g., `swisseph.dll` for Windows, shared libraries for Android/iOS). | Panchangam compiles statically with zero-dependency deployment. Jyotish relies on platform-specific dynamic binaries (.dll, .so, .dylib). |
| **Performance Profile** | Extreme performance (near-native speed, zero GC overhead in engine). Ideal for high-throughput REST APIs and serverless microservices. | High performance but bounded by Dart Virtual Machine and asynchronous FFI communication channels. | Rust's memory safety and compilation to WebAssembly allow millisecond-level calculation throughput at the edge. |
| **Module Isolation & Tree-Shaking** | Single WebAssembly bundle containing all compiled algorithms. Fully typed `d.ts` generated. | Modular design with **9 micro-targeted barrel files** (e.g., `core.dart`, `panchanga.dart`, `systems.dart`, `analysis.dart`). | **Jyotish** excels in client bundle size control, allowing tree-shaking to strip unused modules in release builds. |
| **Initialization & Cleanup** | Instantaneous Wasm loading; memory is managed automatically by Rust's RAII within the WASM boundary. | Async FFI initialization (`await jyotish.initialize()`) with mandatory `dispose()` calls to clean up persistent C pointers. | **Panchangam** requires no manual pointer cleanup in JS/TS. **Jyotish** requires strict lifecycle management to avoid memory leaks in FFI. |

---

## 🪐 Section 2: Core Astronomical & Calculation Engine

| Feature/Metric | Panchangam (Wasm) (`panchangam-fork`) | Jyotish Flutter Library (`jyotish-flutter-library-fork`) | Comparison & Notes |
| :--- | :--- | :--- | :--- |
| **Underlying Engine** | Swiss Ephemeris v2.10.03 | Swiss Ephemeris v2.10.03 | Both leverage the gold standard for NASA/JPL-compliant planetary precision. |
| **Coordinate Systems** | Geocentric calculations (Earth-centered). | Geocentric and **Topocentric** (surface-centered, adjusting for viewer longitude, latitude, and altitude). | **Jyotish** supports topocentric positioning via `CalculationFlags.topocentric()`, crucial for exact modern physical astrology. |
| **Planetary Coverage** | 7 Traditional planets + 2 Lunar Nodes (Rahu/Ketu). | 7 Traditional planets + 2 Nodes + Outer planets (Uranus, Neptune, Pluto) + Asteroids (Chiron, Ceres, Pallas, Juno, Vesta). | **Jyotish** offers wider astronomical coverage, supporting modern outer planets and asteroids. |
| **Mean vs. True Node (Rahu/Ketu)** | Dynamically calculated based on standard Swiss Ephemeris configurations. | Configurable via `CalculationFlags.withNodeType(NodeType.meanNode / trueNode)`. | **Jyotish** explicitly supports switching node types dynamically per API call, supporting both traditional and modern schools. |
| **Ayanamsha Support** | Supports **Lahiri (Chitrapaksha)**, Raman, Krishnamurti, True Chitrapaksha, and others (40+ systems). | Supports **Lahiri**, Raman, Krishnamurti, Fagan-Bradley, and 40+ other standard Swiss Ephemeris systems. | Both libraries provide comprehensive ayanamsha offsets to translate tropical coordinate grids to sidereal. |
| **House Calculation Systems** | Supported via `calculate_houses()`. Supports Placidus ('P'), Whole Sign ('W'), Equal ('E'), and others. | Supported via `calculateHouses()`. Supports Placidus, Whole Sign, Equal, and custom house divisions. | Equivalent capability; both wrap Swiss Ephemeris house cusp modules. |

---

## 🌅 Section 3: Panchanga & Muhurta Systems

| Feature/Metric | Panchangam (Wasm) (`panchangam-fork`) | Jyotish Flutter Library (`jyotish-flutter-library-fork`) | Comparison & Notes |
| :--- | :--- | :--- | :--- |
| **Core Panchanga Limbs** | Complete: Tithi, Nakshatra, Yoga, Karana, Vara. | Complete: Tithi, Nakshatra, Yoga, Karana, Vara. | Both implement authentic Vedic calendar algorithms. |
| **Sunrise-to-Sunrise "Vara" Standard** | Strictly calculated based on exact local Sunrise boundaries (a day ends at the next sunrise, not midnight). | Strictly calculated based on local Sunrise boundaries; correct day lord mapping for pre-dawn births. | Equivalent; both adhere to traditional "Udaya" standards rather than modern calendar days. |
| **High-Precision End Times** | Calculates exact transit/crossing times of Sun and Moon to find precise end times for Tithis and Nakshatras. | Includes `getTithiEndTime()` with customized accuracy threshold and `getTithiJunction()` with microsecond-level accuracy. | **Jyotish** provides more detailed parameters for junction and phase calculations. |
| **Abhijit Nakshatra (28th Mansion)** | Implicitly handled in specific astronomical conversions. | **First-Class Support**: Methods like `getNakshatraWithAbhijit()`, `isInAbhijitNakshatra()`, and `getAbhijitBoundaries()`. | **Jyotish** has highly advanced, dedicated APIs to isolate the 28th intercalary nakshatra (6°40' to 10°53'20" Capricorn). |
| **Masa & Samvatsara** | Support for Masa (Month) and Samvatsara (60-year Jupiter cycle). | Full support for **Amanta** (new moon start) and **Purnimanta** (full moon start) systems; automatic **Adhika Masa** (leap month) detection. | **Jyotish** is superior in calendar logic, supporting regional month systems (Amanta vs. Purnimanta) and leap month indicators. |
| **Daily Inauspicious Muhurtas** | Rahu Kalam, Yamaganda, Gulika (divided into 8 segments from sunrise to sunset). | Rahukalam, Gulikalam, Yamagandam (includes nighttime divisions via `calculateNighttimeInauspicious()`). | **Jyotish** adds specialized nighttime divisions for inauspicious timing windows. |
| **Daily Auspicious Muhurtas** | Brahma Muhurta, Abhijit Muhurta. | Brahma Muhurta, Abhijit Muhurta, **Hora** (planetary hours), **Choghadiya** (8 day/night divisions), **Gowri Panchangam**. | **Jyotish** has a much richer Muhurta suite, including Gowri, Choghadiya, and hourly Horas. |

---

## ⏳ Section 4: Predictive Dasha Systems

| Dasha System | Panchangam (Wasm) (`panchangam-fork`) | Jyotish Flutter Library (`jyotish-flutter-library-fork`) | Comparison & Notes |
| :--- | :--- | :--- | :--- |
| **Vimshottari Dasha (120-Year)** | **3 Levels**: Mahadasha, Antardasha, Pratyantardasha. | **5 Levels**: Mahadasha down to Sukshma Dasha. Includes refined node-lordship overrides. | **Jyotish** provides much deeper nesting (5 levels) for precise micro-event timing. |
| **Yogini Dasha (36-Year)** | **3 Levels**: Mahadasha, Antardasha, Pratyantardasha. | Complete 36-year cycle with sub-periods (Mahadasha and Antardasha). | Equivalent; both calculate nakshatra-based Yogini lord cycles. |
| **Ashtottari Dasha (108-Year)** | Calculated based on 28-star Ardra-adi cycle (includes Abhijit mapping). | Complete implementation of 108-year Ashtottari cycle. | Both support this major conditional dasha system. |
| **Kalachakra Dasha** | *Not implemented in active modules.* | **Fully Supported**: Complex nakshatra-pada zodiacal dasha. | **Jyotish** exclusively supports this highly complex nakshatra-subdivision system. |
| **Chara Dasha (Jaimini)** | Fully calculated using standard Jaimini/K.N. Rao direct/reverse sign rules. | Fully calculated using standard sign-duration rules. | Equivalent; both support the fundamental Jaimini sign-based dasha. |
| **Narayana Dasha (Jaimini)** | Calculated based on sign distance to lord (Lagna-based). | Complete Narayana sign-based dasha mapping. | Equivalent; both calculate Narayana sign progressions. |

---

## 📊 Section 5: Advanced Strength & Diagnostic Systems

| Feature/Metric | Panchangam (Wasm) (`panchangam-fork`) | Jyotish Flutter Library (`jyotish-flutter-library-fork`) | Comparison & Notes |
| :--- | :--- | :--- | :--- |
| **Ashtakavarga (BAV/SAV)** | Full implementation: calculates Bhinnashtakavarga (BAV) for all 7 planets, Sarvashtakavarga (SAV), and Prastara grids. | Full implementation: calculates BAV, SAV, Bindu allocations, and transit thresholds. | Both systems calculate full Ashtakavarga structures accurately. |
| **Ashtakavarga Reductions** | Fully implements **Trikona Shodhana** (Trine Reduction) and **Ekadhipatya Shodhana** (Lordship Reduction). | Fully implements Trikona and Ekadhipatya Shodhana reductions. | Equivalent calculations. |
| **Shodhya Pinda** | Fully calculates **Shodhya Pinda** (both Binda Pinda and Graha Pinda) for planetary strength. | Fully calculates Shodhya Pinda, **Yoga Pinda**, and **House Pinda** metrics. | **Jyotish** has specialized sub-classes (`YogaPindaResult`, `ShodhyaPindaResult`) for highly structured diagnostic reporting. |
| **Shadbala (6-fold Strength)** | Implements all six traditional components: Sthana (positional), Dig (directional), Kala (temporal), Chesta (motional), Naisargika (natural), and Drik (aspectual). | Complete traditional 6-fold Shadbala calculations with detailed breakdown models. | Both libraries provide authentic, complete Shadbala score metrics. |
| **Bhava Bala (House Strength)** | *Not explicitly modeled as a separate service.* | **Fully Supported**: Computes strength for all 12 houses based on house lord, aspects, and Kendra placement. | **Jyotish** provides native house-cusp strength analysis. |
| **Sudarshan Chakra** | Analyzes house distributions from Lagna, Moon, and Sun signs. | **Fully Supported**: Integrates a complete triple-perspective analysis with custom diagnostic models. | Both implement this powerful diagnostic technique; **Jyotish** packages it into a highly structured service. |
| **Vimsopaka Bala (Divisional Strength)** | *Not explicitly modeled.* | **Fully Supported**: Analyzes planetary strength across divisional varga charts (D1, D2, D3, D9, D12, D30) out of a 20-point scale. | **Jyotish** has a clear advantage for divisional strength diagnosis. |

---

## 🧩 Section 6: Divisional Charts, KP, & Jaimini Systems

| Feature/Metric | Panchangam (Wasm) (`panchangam-fork`) | Jyotish Flutter Library (`jyotish-flutter-library-fork`) | Comparison & Notes |
| :--- | :--- | :--- | :--- |
| **Varga Charts (Divisional)** | Supports major Shodashavargas: D1, D2, D3, D4, D7, D9, D10, D12, D16, D20, D24, D27, D30, D40, D45, D60. | Supports all 16 major divisional charts (D1 to D60), plus **D150** (Nadi Amsa) and **D249** (KP subdivisions). | **Jyotish** has broader varga subdivisions (specifically D150 and the proportional D249 subdivisions). |
| **D249 Proportional Divisions** | *Not supported.* | **Fully Supported**: Proportional subdivisions based on Vimshottari Dasha spans rather than equal linear divisions. | **Jyotish** offers this ultra-precise KP system tool (10-20x more resolution than Navamsa). |
| **Krishnamurti Paddhati (KP)** | Calculates KP Lords (Sign, Star, Sub-Lords) and House Significators. | Calculates KP Lords, House Significators, and KP-specific transit divisions. | Equivalent base; both support core KP astrological metrics. |
| **Jaimini Karakas** | Calculates Atmakaraka, Amatyakaraka, etc., supporting both **7 and 8 karaka schemes** (retaining/excluding Rahu). | Supports Atmakaraka, Karakamsa, Arudha Lagna (AL), Upapada Lagna (UL), and Rashi Drishti (sign-based aspects). | Both calculate basic Karakas; **Jyotish** has wider sign-based aspects and Arudha/Upapada Lagna helpers. |
| **Prashna (Horary) Systems** | *Not implemented in active modules.* | **Fully Supported**: Prashna Arudha based on seeds 1-249, Sphutas (Trisphuta, Chatursphuta, etc.), and Gulika calculations. | **Jyotish** is a dedicated environment for Horary (Prashna) calculations. |
| **Planetary War (Graha Yuddha)** | **Fully Supported**: Automatically detects planetary wars (< 1° proximity) and determines the winner based on brightness/latitude. | Calculated as part of planetary relationship aspects. | **Panchangam** has a dedicated, easy-to-use API (`check_graha_yuddha()`) for instant collision detection. |
| **Transits & Gochara Vedha** | Implements Sade Sati, Dhaiya, and Gochara Vedha (transit obstruction) checks. | Implements Sade Sati, Dhaiya, **Panchak**, and complex Gochara Vedha transit analysis. | **Jyotish** adds Panchak calculations and favorable snapshot lookups. |

---

## 💎 Section 7: Exclusive Advanced Interpretation Services

These services represent high-level astrological interpretations that process raw planetary data into clear human predictions. They are **exclusively available in the Jyotish Flutter Library Fork** and do not exist in the Wasm active workspace.

### 1. 💑 Marriage Compatibility (Ashtakoota Milan)
* **Ashtakoota 36 Guna Matching**: Fully implements all 8 traditional compatibility tests:
  1. *Varna* (1 Point) - Spiritual/working class compatibility.
  2. *Vashya* (2 Points) - Mutual control and attraction (handles dual-sign splits for Sagittarius and Capricorn).
  3. *Tara* (3 Points) - Health and longevity indicators calculated in both directions (Boy ↔ Girl).
  4. *Yoni* (4 Points) - Sexual and physical compatibility based on a detailed animal relationship matrix.
  5. *Graha Maitri* (5 Points) - Mental compatibility based on permanent and temporary sign lord friendships (BPHS natural rules).
  6. *Gana* (6 Points) - Temperament compatibility (Deva, Manushya, Rakshasa classification).
  7. *Bhakoot* (7 Points) - Family happiness and mutual prosperity (checks and flags 2/12, 5/9, and 6/8 positions).
  8. *Nadi* (8 Points) - Physiological compatibility and progeny health (Adi, Madhya, Antya cyclic nakshatra grouping).
* **Dosha Checks & Cancellations**:
  * **Manglik Dosha**: Full diagnostic analyzing Mars placements from Ascendant, Moon, and Venus. Implements **5 classical cancellations (Parihara)** (e.g., own sign, Jupiter conjunction, Lagna Lord status) and calculates severity.
  * **Nadi & Bhakoot Doshas**: Automatic detection of doshas and cancellation triggers (e.g., same nakshatra but different padas, or mutual friendship between sign lords).

### 2. 👶 Progeny (Child) Prediction
* Analyzes the 5th House, 5th Lord, Jupiter (putrakaraka), and the **D7 (Saptamsa) divisional chart**.
* Implements child yogas and counts favorable/unfavorable influences to assess fertility, child health, and progeny timing.

### 3. 💼 Career Guidance & Analysis
* Analyzes the 10th House, 10th Lord, Amatyakaraka (career lord), and the **D10 (Dasamsa) divisional chart**.
* Identifies dominant elements, planetary placements, and specific planetary combinations (yogas) to recommend optimal professions.

### 4. 🌴 Nadi Astrology (150 Nadis per Sign)
* Divides each zodiac sign into **150 unique micro-divisions (Nadi Amsas)**, yielding **1800 Nadis** across the entire zodiac.
* Calculates precise Nadi coordinates for the Ascendant, Sun, Moon, and planets.
* Implements Nadi rulings (e.g., Agasthiya, Bhrigu, Saptarshi) and maps them to elements, ruling planets, spiritual characteristics, and life-path interpretations (early, mid, and late-life focus).

### 5. ⏰ Event Timing Engine (`EventTimingService`)
* **Unified Diagnostic Engine**: Intersects a user's **Vimshottari Dasha** periods with real-time **Planetary Transits (Gochara)** and **Gochara Vedha (transit obstructions)**.
* **Auspicious Time Windows**: Scans a target range to output high-precision timing windows rated from *very favorable* to *challenging* for specific life events:
  * *Marriage* (correlating transits in houses 7, 2, and 11)
  * *Career* (correlating houses 10, 2, 6, and 11)
  * *Finance* (correlating houses 2, 11, 5, and 9)
  * *Health, Travel, Education, and Spiritual Pursuits*.

---

## 🎯 Summary Comparison Chart

| Category | Panchangam (Wasm) | Jyotish Flutter Library (Fork) | Winner |
| :--- | :---: | :---: | :---: |
| **High-Throughput Web Backend / Edge** | **Yes** (Wasm-first, Node/Deno/Cloudflare) | No (Requires Dart runtime / FFI files) | 🏆 **Panchangam** |
| **Cross-Platform Mobile/Desktop Client** | No (Requires JS environment wrapper) | **Yes** (Native Android, iOS, Windows, macOS) | 🏆 **Jyotish** |
| **Core Astro Calculations** | High-Precision | High-Precision | **Draw** |
| **Comprehensive Muhurta Suite** | Standard | High (Hora, Choghadiya, Gowri, Night inauspicious) | 🏆 **Jyotish** |
| **Varga Divisional Charts** | D1 to D60 | D1 to D60, D150, D249 | 🏆 **Jyotish** |
| **Dasha System Coverage** | Vimshottari, Yogini, Narayana, Chara, Ashtottari | Vimshottari (5-level), Yogini, Narayana, Chara, Ashtottari, **Kalachakra** | 🏆 **Jyotish** |
| **Marriage Guna Compatibility** | *Not supported* | **Yes** (Ashtakoota Milan, Manglik, Nadi/Bhakoot Doshas) | 🏆 **Jyotish** |
| **Nadi Astrology (150 Divisions)** | *Not supported* | **Yes** (1800-Nadi calculations & interpretations) | 🏆 **Jyotish** |
| **Career & Progeny Diagnostics** | *Not supported* | **Yes** (5th/10th house & D7/D10 analysis) | 🏆 **Jyotish** |
| **Automated Event Timing Engine** | *Not supported* | **Yes** (Intersection of Dasha + Transit + Vedha) | 🏆 **Jyotish** |

---

## 💡 Recommendation Matrix

* **Choose Panchangam (Wasm)** if you are building:
  1. A high-scale SaaS web application with a backend running on serverless environments like Cloudflare Workers, AWS Lambda, or edge engines.
  2. A high-throughput astrological API where near-instant calculation speed and low server overhead are critical.
  3. A web-based widget that needs to run client-side in the browser without installing platform-specific binaries.

* **Choose Jyotish Flutter Library (Fork)** if you are building:
  1. A native Android, iOS, Windows, macOS, or Linux mobile/desktop application using Flutter.
  2. A consumer-facing astrology app that requires rich ready-made predictive modules (Marriage compatibility/Guna Milan, Career guidance, Progeny, or Nadi interpretations).
  3. A complex scheduling or Muhurta planner that requires deep nested dasha levels, Gowri Panchangam, Choghadiya, and automated Event Timing calculations.
