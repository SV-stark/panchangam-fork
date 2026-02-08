
import { assertEquals } from "@std/assert";
import { calculate_kp } from "../lib/panchangam.js";

Deno.test("KP: Sub Lord Calculation", () => {
    // Example: Sun at 0 deg Aries
    // Aswini Nakshatra (Ketu Lord)
    // 0 deg is start of Aswini.
    // Sub Lord should be Ketu (Start of Vimshottari for Ketu Nakshatra).
    // Sub-Sub Lord should be Ketu.

    const res = calculate_kp(0.0);
    console.log("0 deg Aries KP:", res);

    assertEquals(res.sign_id, 1); // Aries
    assertEquals(res.sign_lord, 3); // Mars
    assertEquals(res.star_lord, 9); // Ketu
    assertEquals(res.sub_lord, 9); // Ketu
    assertEquals(res.sub_sub_lord, 9); // Ketu
});

Deno.test("KP: Sub Lord Check (Mid Aswini)", () => {
    // Aswini spans 13deg 20m = 13.333 deg.
    // Ketu Dasha is 7 years out of 120.
    // Ketu Sub in Aswini = (7/120) * 13.333 = ~0.777 deg.
    // So 0.0 to 0.777 is Ketu Sub.
    // Next is Venus Sub (20/120 * 13.333) = ~2.22 deg.
    // Range 0.777 to 3.0 is Venus Sub.

    // Test at 1.0 degree Aries. Should be Venus Sub.
    const res = calculate_kp(1.0);
    console.log("1 deg Aries KP:", res);

    assertEquals(res.star_lord, 9); // Ketu
    assertEquals(res.sub_lord, 6); // Venus
});
