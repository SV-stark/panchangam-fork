import { parse } from "@std/toml";
import { assertEquals } from "@std/assert";

const textDecoder = new TextDecoder("utf-8");

async function checkVersion() {
  const cargoTomlRaw = await Deno.readFile("Cargo.toml");
  const denoJsonRaw = await Deno.readFile("deno.json");

  const cargoToml = parse(textDecoder.decode(cargoTomlRaw));
  const denoJson = JSON.parse(textDecoder.decode(denoJsonRaw));

  const cargoVersion =
    (cargoToml as { package: { version: string } }).package.version;
  const denoVersion = denoJson.version;

  console.log(`Cargo version: ${cargoVersion}`);
  console.log(`Deno version: ${denoVersion}`);

  assertEquals(cargoVersion, denoVersion, "Versions do not match!");

  // Check against tag if provided as argument
  if (Deno.args.length > 0) {
    let tag = Deno.args[0];
    // Remove 'v' prefix if present
    if (tag.startsWith("v")) {
      tag = tag.substring(1);
    }
    console.log(`Tag version: ${tag}`);
    assertEquals(tag, cargoVersion, "Tag does not match Cargo version!");
    assertEquals(tag, denoVersion, "Tag does not match Deno version!");
  }

  console.log("Versions match (Cargo <-> Deno).");

  // Check npm/package.json if it exists
  try {
    const npmPackageRaw = await Deno.readFile("npm/package.json");
    const npmPackage = JSON.parse(textDecoder.decode(npmPackageRaw));
    const npmVersion = npmPackage.version;
    console.log(`NPM version: ${npmVersion}`);
    assertEquals(
      denoVersion,
      npmVersion,
      "NPM package version does not match Deno version!",
    );
  } catch (e) {
    if (e instanceof Deno.errors.NotFound) {
      console.log("Skipping NPM check (npm/package.json not found).");
    } else {
      throw e;
    }
  }
}

if (import.meta.main) {
  await checkVersion();
}
