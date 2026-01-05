import {
  calculate_sunrise,
  calculate_sunset,
  calculate_tithi,
  get_version,
  Location,
} from "../lib/panchangam.js";

// Helper to format dates
const fmt = (ts: number) => new Date(ts).toLocaleString();

console.log("=".repeat(50));
console.log(`🌌 Panchangam Library Demo (Swiss Eph v${get_version()})`);
console.log("=".repeat(50));

// 1. Configuration
const date = new Date("2024-01-22T06:00:00Z"); // Ram Mandir Consecration Day
const jd = 2460331.75; // Approx JD for that time
const location = new Location(26.7922, 82.1998, 0); // Ayodhya, India

console.log(`\n📍 Location: Ayodhya (26.79°N, 82.20°E)`);
console.log(`📅 Date: ${date.toDateString()}`);

// 2. Sunrise / Sunset
const sunrise = calculate_sunrise(2024, 1, 22, location);
const sunset = calculate_sunset(2024, 1, 22, location);

console.log("\n☀️  Solar Details:");
console.log(`   Sunrise: ${fmt(sunrise)}`);
console.log(`   Sunset:  ${fmt(sunset)}`);

// 3. Vedic Time (Tithi)
const tithi = calculate_tithi(jd);

console.log("\n🌙 Lunar Details:");
console.log(`   Tithi:      ${tithi.name} (${tithi.index})`);
console.log(`   Paksha:     ${tithi.paksha_name}`);
console.log(`   Completion: ${(tithi.completion * 100).toFixed(2)}%`);

// 4. Clean up Wasm memory
location.free();
tithi.free();

console.log("\n✅ Demo completed successfully.");
