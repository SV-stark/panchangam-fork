
import { assertEquals } from "@std/assert";
import { calculate_shodhya_pinda } from "../lib/panchangam.js";

Deno.test("Ashtakavarga: Shodhya Pinda Calculation", () => {
    // Mock reduced bindus (after Trikona/Ekadhipatya reductions)
    // 12 signs.
    const reduced_bindus = [1, 2, 0, 3, 1, 0, 2, 1, 0, 2, 1, 0];

    // Mock Planet positions (Sun to Saturn: 0-6)
    // Format: { id, longitude }
    // Sun(0) in Aries (0-30 deg) -> index 0
    // Moon(1) in Taurus (30-60 deg) -> index 1
    // ...
    const planets = [
        { id: 0, longitude: 10.0 }, // Aries (idx 0). Bindus=1.
        { id: 1, longitude: 40.0 }, // Taurus (idx 1). Bindus=2.
        { id: 2, longitude: 100.0 }, // Cancer (idx 3). Bindus=3.
        { id: 3, longitude: 160.0 }, // Virgo (idx 5). Bindus=0.
        { id: 4, longitude: 220.0 }, // Scorpio (idx 7). Bindus=1.
        { id: 5, longitude: 280.0 }, // Capricorn (idx 9). Bindus=2.
        { id: 6, longitude: 340.0 }, // Pisces (idx 11). Bindus=0.
    ];

    // Target Planet = Sun(0). (Doesn't affect calculation logic in current impl).

    // Rasi Pinda:
    // Aries(0): 1 * 7 = 7
    // Tau(1): 2 * 10 = 20
    // Gem(2): 0 * 8 = 0
    // Can(3): 3 * 4 = 12
    // Leo(4): 1 * 10 = 10
    // Vir(5): 0 * 5 = 0
    // Lib(6): 2 * 7 = 14
    // Sco(7): 1 * 8 = 8
    // Sag(8): 0 * 9 = 0
    // Cap(9): 2 * 5 = 10
    // Aqu(10): 1 * 11 = 11
    // Pis(11): 0 * 12 = 0
    // Sum = 7+20+0+12+10+0+14+8+0+10+11+0 = 92

    // Graha Pinda:
    // Sun(0) in Aries(0). Bindus=1. Mult=5. -> 1*5 = 5.
    // Moon(1) in Tau(1). Bindus=2. Mult=5. -> 2*5 = 10.
    // Mars(2) in Can(3). Bindus=3. Mult=8. -> 3*8 = 24.
    // Mer(3) in Vir(5). Bindus=0. -> 0.
    // Jup(4) in Sco(7). Bindus=1. Mult=10. -> 1*10=10.
    // Ven(5) in Cap(9). Bindus=2. Mult=7. -> 2*7=14.
    // Sat(6) in Pis(11). Bindus=0. -> 0.
    // Sum = 5+10+24+0+10+14+0 = 63.

    // Total Shodhya Pinda = 92 + 63 = 155.

    const pinda = calculate_shodhya_pinda(reduced_bindus, planets, 0);
    console.log("Calculated Pinda:", pinda);

    assertEquals(pinda, 155);
});
