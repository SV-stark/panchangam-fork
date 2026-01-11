import { assert } from "@std/assert";
import {
  calculate_daily_panchang,
  get_swisseph_version,
  Location,
} from "../lib/panchangam.js";

Deno.test("Deno Integration - Default Import", () => {
  // Test 1: Location Object
  const loc = new Location(12.97, 77.59, 920);
  assert(loc, "Location should be instantiable");

  // Test 2: Calculation
  // 2026-01-05
  const result = calculate_daily_panchang(2026, 1, 5, loc, 1);
  assert(result, "Panchang calculation should return result");
  assert(result.tithi_index > 0, "Tithi Index should be valid");
  assert(result.tithi_name.length > 0, "Tithi Name should be present");

  // Test 3: Version check (Optional, depending on export)
  const ver = get_swisseph_version();
  console.log("Swisseph Version:", ver);
  assert(ver.length > 0, "Swisseph version should be returned");
});
