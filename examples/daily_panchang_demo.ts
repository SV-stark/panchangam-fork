import {
  calculate_daily_panchang,
  get_version,
  Location,
} from "../lib/panchangam.js";

console.log("=".repeat(60));
console.log(`📅 Daily Panchang & Udaya Tithi Demo`);
console.log(`   Lib Version: ${get_version()}`);
console.log("=".repeat(60));

// Test Data: Jan 5, 2026, Bangalore
// From Spec: 12.9716 N, 77.5946 E
const loc = new Location(12.9716, 77.5946, 920.0);
const year = 2026;
const month = 1;
const day = 5;

// Ayanamsha: Lahiri (1)
const result = calculate_daily_panchang(year, month, day, loc, 1);

// Helpers
const fmtTime = (ms: number | undefined | null) => {
  if (!ms) return "---";
  return new Date(ms).toISOString().replace("T", " ").split(".")[0] + " UTC";
};

try {
  const p = result; // unwrapped result
  console.log(
    `\n📍 Location: ${loc.latitude.toFixed(4)}, ${loc.longitude.toFixed(4)}`,
  );
  console.log(`🗓️  Date:     ${year}-${month}-${day}`);
  console.log(`🌅 Sunrise:  ${fmtTime(p.sunrise)}`);
  console.log(`🌇 Sunset:   ${fmtTime(p.sunset)}`);

  console.log("\n--- Udaya Panchang (At Sunrise) ---");
  console.log(`🌔 Tithi:     ${p.tithi_name} (${p.tithi_index})`);
  console.log(`   Ends at:   ${fmtTime(p.tithi_end_time)}`);

  console.log(`🌟 Nakshatra: ${p.nakshatra_name} (${p.nakshatra_index})`);
  console.log(`   Ends at:   ${fmtTime(p.nakshatra_end_time)}`);

  console.log(`🧘 Yoga:      ${p.yoga_name} (${p.yoga_index})`);
  console.log(`   Ends at:   ${fmtTime(p.yoga_end_time)}`);

  console.log(`📅 Vara:      ${p.vara_name}`);

  console.log(`\n📐 Ayanamsha: ${p.ayanamsha_value.toFixed(6)}°`);

  console.log("\n⏳ Muhurats (Time Qualities) ---");
  const m = p.muhurats;
  console.log(
    `   ⛔ Rahu:   ${fmtTime(m.rahu_kalam.start)} - ${
      fmtTime(m.rahu_kalam.end)
    }`,
  );
  console.log(
    `   ☠️  Yama:   ${fmtTime(m.yamaganda.start)} - ${
      fmtTime(m.yamaganda.end)
    }`,
  );
  console.log(
    `   🐍 Gulika: ${fmtTime(m.gulika.start)} - ${fmtTime(m.gulika.end)}`,
  );
} catch (e) {
  console.error("Calculation failed:", e);
}
