import { join } from "@std/path";

const CWD = Deno.cwd();

async function bumpDeno(version: string) {
  const path = join(CWD, "deno.json");
  const content = await Deno.readTextFile(path);
  const json = JSON.parse(content);
  json.version = version;
  await Deno.writeTextFile(path, JSON.stringify(json, null, 2) + "\n");
  console.log(`Updated deno.json to ${version}`);
}

async function bumpCargo(version: string) {
  const path = join(CWD, "Cargo.toml");
  let content = await Deno.readTextFile(path);
  // Replace version = "x.y.z" at the top level (package)
  content = content.replace(/^version\s*=\s*".*?"/m, `version = "${version}"`);
  await Deno.writeTextFile(path, content);
  console.log(`Updated Cargo.toml to ${version}`);

  // Update Cargo.lock
  console.log("Updating Cargo.lock...");
  const command = new Deno.Command("cargo", {
    args: ["check"],
    stdout: "piped",
    stderr: "piped",
  });
  const { code } = await command.output();
  if (code !== 0) {
    console.warn(
      "⚠️ Warning: 'cargo check' failed. Cargo.lock might not be updated.",
    );
  } else {
    console.log("Updated Cargo.lock");
  }
}

async function main() {
  const newVersion = Deno.args[0];
  if (!newVersion) {
    console.error("Usage: deno task bump-version <new_version>");
    Deno.exit(1);
  }

  // Basic semver validation (loose)
  if (!/^\d+\.\d+\.\d+/.test(newVersion)) {
    console.error("Error: Version must be in format x.y.z");
    Deno.exit(1);
  }

  console.log(`Bumping version to: ${newVersion}`);

  try {
    await bumpDeno(newVersion);
    await bumpCargo(newVersion);
    // scripts/build_npm.ts reads from deno.json dynamically, so no update needed there.

    console.log("\n✅ Version bump complete.");
    console.log("Run 'deno task build' to verify everything works.");
  } catch (error) {
    console.error("\n❌ Error bumping version:", error);
    Deno.exit(1);
  }
}

if (import.meta.main) {
  main();
}
