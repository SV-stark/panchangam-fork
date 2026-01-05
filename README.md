# Panchangam (Wasm)

A high-precision, high-performance Vedic Astrology and Calendar library for the
web and server-side environments (Deno, Node.js, Cloudflare Workers).

It is built with **Rust** and compiled to **WebAssembly**, integrating the
gold-standard **Swiss Ephemeris** (C library) for astronomical accuracy
(`Drik Ganita`).

## 🚀 Features

- **High Precision**: Uses Swiss Ephemeris (vsop87) for planetary positions.
- **Zero External Dependencies**: All logic and ephemerides are bundled into the
  Wasm binary.
- **Vedic Calendar (Panchang)**:
  - **Tithi**: Lunar day with exact end times (Root Finding).
  - **Nakshatra**: Lunar mansion with exact end times.
  - **Yoga**: Luni-solar combination with exact end times.
  - **Vara**: Weekday based on Sunrise.
  - **Udaya Logic**: Correctly identifies properties active at sunrise.
- **Advanced Astronomy**:
  - **Graha Yuddha**: Planetary War detection for Tara Grahas.
  - **Ayanamsha**: Supports Lahiri, Raman, KP, True Chitrapaksha.
  - **Planetary Positions**: Sidereal and Tropical longitudes.
- **Muhurat Matrix**: dynamic calculation of Rahu Kalam, Yamaganda, Gulika based
  on actual day duration.

## 📦 Usage

### Installation

Currently set up as a local crate. Build it first:

```bash
deno task build
```

This produces `lib/panchangam.js` and `lib/panchangam.wasm`.

### Basic Example

```typescript
import { calculate_daily_panchang, Location } from "./lib/panchangam.js";

// Location: Bangalore (12.97 N, 77.59 E)
const loc = new Location(12.9716, 77.5946, 920.0);

// Calculate for Jan 5, 2026, using Lahiri Ayanamsha (mode 1)
const result = calculate_daily_panchang(2026, 1, 5, loc, 1);

console.log("Tithi:", result.tithi_name);
// Output: "Dwitiya"

console.log(
  "Nakshatra Ends:",
  new Date(result.nakshatra_end_time).toISOString(),
);
// Output: "2026-01-05T07:54:53.000Z"
```

### Muhurats (Time Qualities)

```typescript
const m = result.muhurats;
console.log(`Rahu Kalam: ${new Date(m.rahu_kalam.start).toLocaleTimeString()}`);
```

### Planetary War

```typescript
import { check_graha_yuddha, raw_swe_julday } from "./lib/panchangam.js";

const jd = raw_swe_julday(2024, 2, 22, 12.0, 1);
const wars = check_graha_yuddha(jd, 1); // 1 = Lahiri

if (wars.length > 0) {
  console.log(`${wars[0].planet1_name} fights ${wars[0].planet2_name}!`);
  console.log(
    `Winner: ${
      wars[0].winner_id === wars[0].planet1_id
        ? wars[0].planet1_name
        : wars[0].planet2_name
    }`,
  );
}
```

## 🛠️ Build

Requirements:

- **Rust** (stable)
- **Deno**
- **Clang/LLVM** (for compiling Swiss Ephemeris C code)

```bash
deno task build
```

This command:

1. Compiles the Swiss Ephemeris C source inside `src/libswe`.
2. Compiles the Rust crate and links the C library.
3. Generates the Wasm binary and JS bindings in `lib/`.

## 📂 Project Structure

- `src/lib.rs`: Wasm entry point.
- `src/vedic/`: Core Vedic logic (Tithi, Yoga, Muhurat, Graha Yuddha).
- `src/astronomy/`: Astronomy wrappers (Planets, Solver, Ayanamsha).
- `src/libswe/`: Vendored Swiss Ephemeris C source.
- `examples/`: TypeScript verification scripts.

## License

MIT
