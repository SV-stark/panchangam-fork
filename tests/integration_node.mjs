
import { strict as assert } from 'assert';
import { calculate_daily_panchang, Location, get_swisseph_version } from '../lib/panchangam.js';
import * as BrowserLib from '../lib/browser/panchangam.js';

console.log("Testing Node.js Integration...");

async function testDefault() {
    console.log("Testing Default Import...");
    try {
        const loc = new Location(12.97, 77.59, 920);
        const result = calculate_daily_panchang(2026, 1, 5, loc, 1);
        assert.ok(result, "Calculation result should exist");
        console.log("Default Import OK");
    } catch (e) {
        console.error("Default Import Failed:", e);
        process.exit(1);
    }
}

async function testBrowser() {
    console.log("Testing Browser (Inline) Import...");
    try {
        const loc = new BrowserLib.Location(12.97, 77.59, 920);
        const result = BrowserLib.calculate_daily_panchang(2026, 1, 5, loc, 1);
        assert.ok(result, "Browser Calculation result should exist");
        console.log("Browser Import OK");
    } catch (e) {
         console.error("Browser Import Failed:", e);
         process.exit(1);
    }
}

await testDefault();
await testBrowser();
console.log("All Node.js tests passed.");
