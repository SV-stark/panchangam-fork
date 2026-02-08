
import { assertEquals } from "@std/assert";
import { analyze_sade_sati, analyze_dhaiya } from "../lib/panchangam.js";

Deno.test("Transits: Sade Sati Checks", () => {
    // Moon in Aries (1)
    // Saturn in Pisces (12) -> 12th from Moon -> Rising Sade Sati
    const res1 = analyze_sade_sati(1, 12);
    assertEquals(res1.is_active, true);
    assertEquals(res1.phase, "Rising");

    // Moon in Aries (1)
    // Saturn in Aries (1) -> 1st -> Peak Sade Sati
    const res2 = analyze_sade_sati(1, 1);
    assertEquals(res2.is_active, true);
    assertEquals(res2.phase, "Peak");

    // Moon in Aries (1)
    // Saturn in Taurus (2) -> 2nd -> Setting Sade Sati
    const res3 = analyze_sade_sati(1, 2);
    assertEquals(res3.is_active, true);
    assertEquals(res3.phase, "Setting");

    // Moon in Aries (1)
    // Saturn in Gemini (3) -> None
    const res4 = analyze_sade_sati(1, 3);
    assertEquals(res4.is_active, false);
});

Deno.test("Transits: Dhaiya Checks", () => {
    // Moon in Aries (1)
    // Saturn in Cancer (4) -> 4th -> Ardhastama Dhaiya
    const res1 = analyze_dhaiya(1, 4);
    assertEquals(res1.is_active, true);
    assertEquals(res1.type_name, "Ardhastama");

    // Moon in Aries (1)
    // Saturn in Scorpio (8) -> 8th -> Ashtama Dhaiya
    const res2 = analyze_dhaiya(1, 8);
    assertEquals(res2.is_active, true);
    assertEquals(res2.type_name, "Ashtama");

    // Moon in Aries (1)
    // Saturn in Leo (5) -> None
    const res3 = analyze_dhaiya(1, 5);
    assertEquals(res3.is_active, false);
});
