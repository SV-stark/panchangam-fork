import {
  AyanamshaMode,
  calculate_gulika,
  calculate_karana,
  calculate_nakshatra,
  // Muhurat
  calculate_rahu_kaal,
  calculate_sunrise,
  calculate_sunset,
  calculate_tithi,
  calculate_vara,
  calculate_yamaganda,
  calculate_yoga,
  // Astronomy
  get_ayanamsha,
  get_version,
  Location,
} from "../lib/panchangam.js";

// Helper to format dates
const fmt = (ts: number) => new Date(ts).toLocaleString();
const fmtTime = (ts: number | undefined) =>
  ts ? new Date(ts).toLocaleTimeString() : "N/A";

console.log("=".repeat(60));
console.log(
  `🌌 Panchangam Library Complete Demo (Swiss Eph v${get_version()})`,
);
console.log("=".repeat(60));

// 1. Configuration: Ram Mandir Pran Pratishtha
// Date: Jan 22, 2024, 12:29:08 PM IST (Abhijit Muhurat)
const dateStr = "2024-01-22T06:59:08Z";
const date = new Date(dateStr);
// Julian Day (approximate for the demo timestamp)
const jd = 2460331.791065;

console.log(`\n📅 Context: Ram Mandir Pran Pratishtha`);
console.log(`   Timestamp: ${date.toLocaleString()}`);
console.log(`   Julian Day: ${jd.toFixed(6)}`);

// Location: Ayodhya
const loc = new Location(26.7922, 82.1998, 0);
console.log(`   Location: Ayodhya (26.79°N, 82.20°E)`);

// ---------------------------------------------------------
// 2. Solar Calculations (Sunrise/Sunset)
// ---------------------------------------------------------
console.log("\n☀️  Solar & Astronomical");
const sunrise = calculate_sunrise(2024, 1, 22, loc);
const sunset = calculate_sunset(2024, 1, 22, loc);

console.log(`   Sunrise:     ${fmt(sunrise)}`);
console.log(`   Sunset:      ${fmt(sunset)}`);

const ayanamsha = get_ayanamsha(AyanamshaMode.Lahiri, jd);
console.log(`   Ayanamsha:   ${ayanamsha.toFixed(6)}° (Lahiri)`);

// ---------------------------------------------------------
// 3. The Five Limbs (Panchang)
// ---------------------------------------------------------
console.log("\n🤚 The Five Limbs (Panchangam)");

// Tithi
const tithi = calculate_tithi(jd);
console.log(`   1. Tithi:     ${tithi.name} (${tithi.paksha_name} Paksha)`);
console.log(
  `                 Completed: ${(tithi.completion * 100).toFixed(1)}%`,
);

// Nakshatra
const nak = calculate_nakshatra(jd, AyanamshaMode.Lahiri);
console.log(`   2. Nakshatra: ${nak.name} (Ruler: ${nak.ruler})`);
console.log(`                 Pada: ${nak.pada}, Quality: ${nak.quality}`);

// Yoga
const yoga = calculate_yoga(jd, AyanamshaMode.Lahiri);
console.log(`   3. Yoga:      ${yoga.name}`);

// Karana
const karana = calculate_karana(jd);
console.log(`   4. Karana:    ${karana.name} (Half: ${karana.half})`);

// Vara
const vara = calculate_vara(jd);
console.log(`   5. Vara:      ${vara.name} (Lord: ${vara.lord})`);

// ---------------------------------------------------------
// 4. Muhurat (Inauspicious Times)
// ---------------------------------------------------------
console.log("\n⏳ Muhurat (Durations for the day)");

// Weekday index for Monday is 1
const weekday = 1;

const rahu = calculate_rahu_kaal(sunrise, sunset, weekday);
const yama = calculate_yamaganda(sunrise, sunset, weekday);
const gulika = calculate_gulika(sunrise, sunset, weekday);

console.log(
  `   Rahu Kaal:   ${fmtTime(rahu.start_ms)} - ${fmtTime(rahu.end_ms)}`,
);
console.log(
  `   Yamaganda:   ${fmtTime(yama.start_ms)} - ${fmtTime(yama.end_ms)}`,
);
console.log(
  `   Gulika:      ${fmtTime(gulika.start_ms)} - ${fmtTime(gulika.end_ms)}`,
);

// ---------------------------------------------------------
// Cleanup
// ---------------------------------------------------------
loc.free();
tithi.free();
nak.free();
yoga.free();
karana.free();
vara.free();
rahu.free();
yama.free();
gulika.free();

console.log("\n✨ Demo finished.");
