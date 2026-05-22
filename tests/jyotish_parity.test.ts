import { assertEquals, assertNotEquals } from "@std/assert";
import {
  calculate_planets_extended,
  calculate_varga,
  calculate_vimshottari_5_levels,
  calculate_ashtottari,
  calculate_kalachakra,
  calculate_yoga_pinda,
  calculate_house_pinda,
  calculate_bhava_bala,
  calculate_vimsopaka_bala,
  calculate_ashtakoota_milan,
  analyze_manglik_dosha,
  calculate_compatibility_report,
  calculate_nadi_astrology,
  predict_progeny,
  analyze_career_guidance,
  calculate_event_timing,
  calculate_sudarshan_chakra,
  is_in_abhijit_nakshatra,
  Location,
  is_panchak_active,
  predict_sade_sati_cycles,
  calculate_aspects,
  calculate_prashna_arudha,
  calculate_prashna_sphutas,
  calculate_gulika_sphuta,
  calculate_moon_phase_details,
  calculate_vimshottari_timeline,
  calculate_combustion_info,
  analyze_ashtakavarga_transit,
  get_favorable_transit_signs,
  calculate_karakamsa,
  get_rashi_drishti,
  calculate_arudha_padas,
  calculate_argalas,
  get_kp_sub_lord,
  get_kp_sub_sub_lord,
} from "../lib/panchangam.js";

Deno.test("Jyotish Parity: Extended Planetary Positions", () => {
  const jd = 2451545.0; // J2000
  const planets = calculate_planets_extended(
    jd,
    1, // Lahiri
    true, // Topocentric
    12.9716, // Bangalore Lat
    77.5946, // Bangalore Lon
    920.0, // Bengaluru altitude
    true, // True Node
    true, // Outer planets
    true // Asteroids
  );

  console.log("Extended Planets Count:", planets.length);
  // standard 9 + 3 outer + asteroids
  assertEquals(planets.length > 9, true);

  const sun = planets.find((p: any) => p.id === 0);
  assertNotEquals(sun, undefined);
  assertEquals(sun.name, "Sun");
});

Deno.test("Jyotish Parity: D150 and D249 Vargas", () => {
  // Test D150 (Nadi Amsa)
  const v150 = calculate_varga(12.5, 150, null);
  console.log("Varga D150 Sign:", v150.sign, "Varga Lon:", v150.longitude);
  assertEquals(v150.sign >= 1 && v150.sign <= 12, true);

  // Test D249 (KP subdivisions)
  const v249 = calculate_varga(12.5, 249, null);
  console.log("Varga D249 Sign:", v249.sign, "Varga Lon:", v249.longitude);
  assertEquals(v249.sign >= 1 && v249.sign <= 12, true);
});

Deno.test("Jyotish Parity: Abhijit Nakshatra Boundary Check", () => {
  // Abhijit Nakshatra boundaries are from 6°40' to 10°53'20" Capricorn (276.6667 to 280.8889 deg)
  assertEquals(is_in_abhijit_nakshatra(278.0), true);
  assertEquals(is_in_abhijit_nakshatra(100.0), false);
});

Deno.test("Jyotish Parity: 5-Level Vimshottari Dasha", () => {
  // Moon at 0 deg Aries (Ashwini nakshatra, Lord Ketu)
  const dasha = calculate_vimshottari_5_levels(0.0, 0.0, 1000 * 3600 * 24 * 30);
  console.log("5-Level Dasha:", dasha.mahadasha, "->", dasha.antardasha, "->", dasha.pratyantardasha, "->", dasha.sookshmadasha, "->", dasha.pranadasha);
  assertEquals(dasha.mahadasha, "Ketu");
  assertNotEquals(dasha.sookshmadasha, "");
  assertNotEquals(dasha.pranadasha, "");
});

Deno.test("Jyotish Parity: Ashtottari & Kalachakra Dashas", () => {
  // Ashtottari
  const ashtottari = calculate_ashtottari(0.0, 0.0, 0.0);
  console.log("Ashtottari Periods Count:", ashtottari.length);
  assertEquals(ashtottari.length > 0, true);

  // Kalachakra
  const kalachakra = calculate_kalachakra(0.0, 0.0);
  console.log("Kalachakra Nakshatra:", kalachakra.nakshatra_name, " Savya:", kalachakra.is_savya);
  assertEquals(kalachakra.periods.length > 0, true);
});

Deno.test("Jyotish Parity: Yoga Pinda & House Pinda in Ashtakavarga", () => {
  const reduced_bindus = [1, 2, 0, 3, 1, 0, 2, 1, 0, 2, 1, 0];
  const planets = [
    { id: 0, longitude: 10.0 },
    { id: 1, longitude: 40.0 },
    { id: 2, longitude: 100.0 },
    { id: 3, longitude: 160.0 },
    { id: 4, longitude: 220.0 },
    { id: 5, longitude: 280.0 },
    { id: 6, longitude: 340.0 },
  ];

  const yoga_pinda = calculate_yoga_pinda(reduced_bindus, planets, 0);
  console.log("Calculated Yoga Pinda:", yoga_pinda);
  assertEquals(yoga_pinda > 0, true);

  const house_pinda = calculate_house_pinda(150, 4);
  console.log("Calculated House Pinda:", house_pinda);
  assertEquals(house_pinda, 21);
});

Deno.test("Jyotish Parity: Bhava Bala & Vimsopaka Bala", () => {
  const planet_longs = [10.0, 40.0, 100.0, 160.0, 220.0, 280.0, 340.0];
  const planet_shadbalas = [400.0, 350.0, 300.0, 380.0, 420.0, 390.0, 320.0];

  const bhava_bala = calculate_bhava_bala(1, planet_longs, planet_shadbalas);
  console.log("Bhava Bala House 1 Total Strength:", bhava_bala.houses[0].total_bala);
  assertEquals(bhava_bala.houses.length, 12);

  const vimsopaka = calculate_vimsopaka_bala(planet_longs);
  console.log("Vimsopaka Scores Count:", vimsopaka.planet_scores.length);
  assertEquals(vimsopaka.planet_scores.length > 0, true);
});

Deno.test("Jyotish Parity: Marriage Compatibility Reports", () => {
  const milan = calculate_ashtakoota_milan(0.0, 0.0);
  console.log("Ashtakoota Milan Total Score:", milan.total_score);
  assertEquals(milan.total_score >= 0 && milan.total_score <= 36, true);

  const manglik = analyze_manglik_dosha(120.0, 0.0, 45.0, 90.0);
  console.log("Manglik Status:", manglik.is_manglik, " Severity:", manglik.severity);

  const compatibility = calculate_compatibility_report(
    0.0, 120.0, 0.0, 90.0, // boy
    30.0, 200.0, 45.0, 150.0 // girl
  );
  console.log("Overall Compatibility Percentage:", compatibility.overall_compatibility_pct);
  assertEquals(compatibility.overall_compatibility_pct >= 0 && compatibility.overall_compatibility_pct <= 100, true);
});

Deno.test("Jyotish Parity: Interpretive Engines", () => {
  // Nadi Astrology
  const nadi = calculate_nadi_astrology(125.5);
  console.log("Nadi Interpretation:", nadi.nadi_name, " Early Life Focus:", nadi.early_life_focus);
  assertNotEquals(nadi.nadi_name, "");

  // Progeny Predictor
  const progeny = predict_progeny(12.5, 45.0, 380.0, 1);
  console.log("Progeny Guidance:", progeny.guidance, " Fertility Score:", progeny.fertility_score);
  assertEquals(progeny.fertility_score >= 0 && progeny.fertility_score <= 100, true);

  // Career Guidance
  const career = analyze_career_guidance(92.0, 180.0, 420.0, 4);
  console.log("Career Recommended Professions:", career.recommended_professions);
  assertEquals(career.recommended_professions.length > 0, true);
});

Deno.test("Jyotish Parity: Event Timing Engine", () => {
  const natal_planets = new Map([
    [0, 10.0],
    [1, 40.0],
    [2, 100.0],
    [3, 160.0],
    [4, 220.0],
    [5, 280.0],
    [6, 340.0],
  ]);
  const checkpoints = [
    {
      time_ms: 1000 * 3600 * 24 * 365 * 10,
      planet_longs: new Map([
        [0, 45.0], // Sun
        [1, 90.0], // Moon
        [2, 120.0], // Mars
        [3, 85.0], // Mercury
        [4, 210.0], // Jupiter
        [5, 180.0], // Venus
        [6, 300.0], // Saturn
        [7, 15.0], // Rahu
        [8, 195.0], // Ketu
      ])
    },
    {
      time_ms: 1000 * 3600 * 24 * 365 * 20,
      planet_longs: new Map([
        [0, 95.0],
        [1, 180.0],
        [2, 240.0],
        [3, 120.0],
        [4, 330.0],
        [5, 270.0],
        [6, 60.0],
        [7, 285.0],
        [8, 105.0],
      ])
    }
  ];

  const timing = calculate_event_timing(
    0.0, // birth time ms
    40.0, // natal moon long
    10.0, // natal lagna long
    natal_planets,
    checkpoints
  );

  console.log("Event Timing Marriage Windows Count:", timing.marriage_windows.length);
  assertEquals(timing.marriage_windows.length > 0, true);
});

Deno.test("Jyotish Parity: Triple Perspective Sudarshan Chakra", () => {
  const chakra = calculate_sudarshan_chakra(1, 4, 8);
  console.log("Sudarshan Chakra Houses Count:", chakra.houses.length);
  assertEquals(chakra.houses.length, 12);
  assertEquals(chakra.houses[0].lagna_sign, 1);
  assertEquals(chakra.houses[0].moon_sign, 4);
  assertEquals(chakra.houses[0].sun_sign, 8);
});

Deno.test("Jyotish Parity: Phase 2 Panchak & Sade Sati & Aspects", () => {
  // 1. Panchak Check
  assertEquals(is_panchak_active(295.0), true);
  assertEquals(is_panchak_active(100.0), false);

  // 2. Sade Sati Cycles Prediction
  const cycles = predict_sade_sati_cycles(40.0, 2020, 10);
  console.log("Sade Sati Cycles predicted:", cycles);
  assertEquals(Array.isArray(cycles), true);

  // 3. Aspects Engine
  const planet_longs = [
    [0, 10.0],
    [5, 130.0], // Jupiter aspecting Sun with 5th house aspect (120 deg)
  ];
  const aspects = calculate_aspects(planet_longs);
  console.log("Calculated aspects count:", aspects.length);
  assertEquals(aspects.length > 0, true);
});

Deno.test("Jyotish Parity: Phase 2 Prashna & Moon Phase", () => {
  // 1. Prashna Arudha
  const arudha = calculate_prashna_arudha(249);
  assertEquals(arudha, 8); // (249 - 1) % 12 = 8

  // 2. Prashna Sphutas
  const sphutas = calculate_prashna_sphutas(10.0, 20.0, 30.0, 40.0, 50.0);
  console.log("Trisphuta:", sphutas.trisphuta, "Chatursphuta:", sphutas.chatursphuta);
  assertEquals(sphutas.trisphuta, 60.0);
  assertEquals(sphutas.chatursphuta, 100.0);

  // 3. Gulika Sphuta
  const gulika_start = calculate_gulika_sphuta(0.0, 36000000.0, 1, true); // Monday day Gulika (slot index 5)
  console.log("Gulika Start MS:", gulika_start);
  assertEquals(gulika_start, 22500000.0); // 0 + 5 * (36000000 / 8)

  // 4. Moon Phase Details
  const phase = calculate_moon_phase_details(0.0, 90.0);
  console.log("Moon Illumination %:", phase.illumination_pct, "Phase Name:", phase.phase_name);
  assertEquals(phase.illumination_pct > 0.0, true);
  assertEquals(phase.phase_name, "First Quarter");
});

Deno.test("Jyotish Parity: Phase 2 Dasha Timeline & Combustion & Ashtakavarga Transit", () => {
  // 1. Vimshottari Timeline
  const timeline = calculate_vimshottari_timeline(0.0, 0.0, 10.0);
  console.log("Timeline count:", timeline.length);
  assertEquals(timeline.length > 0, true);

  // 2. Combustion Info
  const combustion = calculate_combustion_info(0.0, 5.0, 2, false); // Mercury at 5 deg direct
  console.log("Mercury combust:", combustion.is_combust, "severity:", combustion.severity);
  assertEquals(combustion.is_combust, true);
  assertEquals(combustion.severity > 0.0, true);

  // 3. Ashtakavarga Transit
  const sav_bindus = [30, 25, 28, 20, 32, 24, 26, 28, 30, 22, 24, 26];
  const transit_analysis = analyze_ashtakavarga_transit(0, sav_bindus);
  console.log("Transit analysis sign 0:", transit_analysis);
  assertEquals(transit_analysis.is_favorable, true);

  const favorable_signs = get_favorable_transit_signs(sav_bindus);
  console.log("Favorable transit signs count:", favorable_signs.length);
  assertEquals(favorable_signs.includes(0), true);
  assertEquals(favorable_signs.includes(1), false);
});

Deno.test("Jyotish Parity: Phase 2 Jaimini & KP Depth", () => {
  const planet_longs = [
    [0, 15.0],  // Sun in Aries (Sign 0), 15.0 deg
    [1, 45.0],  // Moon in Taurus (Sign 1), 15.0 deg
    [2, 75.0],  // Mercury in Gemini (Sign 2), 15.0 deg
    [3, 118.0], // Venus in Cancer (Sign 3), 28.0 deg (highest degree -> Atmakaraka)
    [4, 135.0], // Mars in Leo (Sign 4), 15.0 deg
    [5, 165.0], // Jupiter in Virgo (Sign 5), 15.0 deg
    [6, 195.0], // Saturn in Libra (Sign 6), 15.0 deg
  ];

  // 1. Karakamsa (Venus is AK at 28 deg in Cancer. D9 of 28 deg Cancer:
  // Cancer is a movable sign, D9 starts from Cancer itself. 28 deg / 3.333 = 8.4 -> 9th part.
  // 9th part from Cancer is Pisces (Sign 11). So Karakamsa should be 11.)
  const karakamsa = calculate_karakamsa(planet_longs);
  console.log("Karakamsa sign index:", karakamsa);
  assertEquals(karakamsa >= 0 && karakamsa <= 11, true);

  // 2. Rashi Drishti
  const aspects = get_rashi_drishti(0); // Aries (movable)
  console.log("Aries aspectos:", aspects);
  assertEquals(aspects.includes(4), true); // Leo (fixed)
  assertEquals(aspects.includes(7), true); // Scorpio (fixed)
  assertEquals(aspects.includes(10), true); // Aquarius (fixed)
  assertEquals(aspects.includes(1), false); // Taurus (adjacent fixed - excluded)

  // 3. Arudha Padas
  const padas = calculate_arudha_padas(0, planet_longs); // Lagna sign index 0
  console.log("Arudha Padas:", padas);
  assertEquals(Array.isArray(padas), true);
  assertEquals(padas.length, 12);

  // 4. Argalas
  const argala_result = calculate_argalas(0, planet_longs);
  console.log("Argala result:", argala_result);
  assertEquals(argala_result.net_argala_strength >= 0, true);

  // 5. KP Sub/Sub-Sub Lord
  const sub_lord = get_kp_sub_lord(120.0);
  const sub_sub_lord = get_kp_sub_sub_lord(120.0);
  console.log("KP Sub Lord at 120.0:", sub_lord, "Sub-Sub Lord:", sub_sub_lord);
  assertEquals(sub_lord >= 1 && sub_lord <= 9, true);
  assertEquals(sub_sub_lord >= 1 && sub_sub_lord <= 9, true);
});
