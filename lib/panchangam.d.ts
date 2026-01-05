// @generated file from wasmbuild -- do not edit
// deno-lint-ignore-file
// deno-fmt-ignore-file

export function calculate_daily_panchang(
  year: number,
  month: number,
  day: number,
  location: Location,
  ayan_mode: number,
): DailyPanchang;
/**
 * Check for Planetary War (Graha Yuddha)
 * Occurs when two Tara Grahas (Mars, Mercury, Jupiter, Venus, Saturn)
 * are within 1 degree of each other.
 */
export function check_graha_yuddha(jd: number, ayan_mode: number): any;
/**
 * Get Ayanamsha value for a given mode and Julian Day
 */
export function get_ayanamsha(mode: AyanamshaMode, jd: number): number;
/**
 * Calculate Karana for a given Julian Day
 * There are 60 Karanas per lunar month (2 per Tithi)
 */
export function calculate_karana(jd: number): KaranaInfo;
/**
 * Get the library version from Swiss Ephemeris
 */
export function get_version(): string;
/**
 * Calculate sunrise time for a given date and location
 * Returns Unix timestamp in milliseconds
 */
export function calculate_sunrise(
  year: number,
  month: number,
  day: number,
  location: Location,
): number;
/**
 * Calculate sunset time for a given date and location
 * Returns Unix timestamp in milliseconds
 */
export function calculate_sunset(
  year: number,
  month: number,
  day: number,
  location: Location,
): number;
/**
 * Calculate Yoga for a given Julian Day
 * Formula: Yoga = floor((Moon_long + Sun_long) / 13.333) + 1
 */
export function calculate_yoga(
  jd: number,
  ayanamsha_mode: AyanamshaMode,
): YogaInfo;
/**
 * Calculate Rahu Kaal for a given day
 * sunrise_ms and sunset_ms are Unix timestamps in milliseconds
 * weekday is 0=Sunday, 6=Saturday
 */
export function calculate_rahu_kaal(
  sunrise_ms: number,
  sunset_ms: number,
  weekday: number,
): TimeInterval;
/**
 * Calculate Yamaganda for a given day
 */
export function calculate_yamaganda(
  sunrise_ms: number,
  sunset_ms: number,
  weekday: number,
): TimeInterval;
/**
 * Calculate Gulika Kaal for a given day
 */
export function calculate_gulika(
  sunrise_ms: number,
  sunset_ms: number,
  weekday: number,
): TimeInterval;
/**
 * Swiss Ephemeris Raw API
 *
 * This module exposes a 1:1 mapping of the C API where possible,
 * adapting pointers to JS Objects/Arrays for ease of use.
 * Function names are prefixed with `js_` internally to avoid linker collisions
 * with the C library, but exported with original names via `js_name`.
 * Calculate planetary position (UT)
 * Returns: { longitude, latitude, distance, speed_long, speed_lat, speed_dist, rc_flags }
 * Throws string error on failure
 */
export function raw_swe_calc_ut(
  tjd_ut: number,
  ipl: number,
  iflag: number,
): any;
/**
 * Calculate fixed star position
 */
export function raw_swe_fixstar_ut(
  star: string,
  tjd_ut: number,
  iflag: number,
): any;
export function raw_swe_sidtime(tjd_ut: number): number;
export function raw_swe_julday(
  year: number,
  month: number,
  day: number,
  hour: number,
  gregflag: number,
): number;
export function raw_swe_revjul(tjd: number, gregflag: number): any;
/**
 * Calculate phenomena (phase, eclipse, etc.)
 * Returns: { phase_angle, phase, elongation, diameter_app, magnitude }
 */
export function raw_swe_pheno_ut(
  tjd_ut: number,
  ipl: number,
  iflag: number,
): any;
export function raw_swe_get_planet_name(ipl: number): string;
export function raw_swe_set_topo(lon: number, lat: number, alt: number): void;
export function raw_swe_set_sid_mode(
  sid_mode: number,
  t0: number,
  ayan_t0: number,
): void;
export function raw_swe_get_ayanamsa_ut(tjd_ut: number): number;
/**
 * Calculate Vara (weekday) for a given Julian Day
 * Note: This returns the astronomical weekday.
 * For Vedic Vara, compare with sunrise time.
 */
export function calculate_vara(jd: number): VaraInfo;
/**
 * Calculate Raahu, Yamaganda, and Gulika for a given day
 *
 * # Arguments
 * * `sunrise_ms` - Unix timestamp of sunrise in ms
 * * `sunset_ms` - Unix timestamp of sunset in ms
 * * `weekday` - 0=Sunday, 1=Monday, ..., 6=Saturday
 */
export function calculate_muhurats(
  sunrise_ms: number,
  sunset_ms: number,
  weekday: number,
): DayMuhurats;
/**
 * Calculate Nakshatra for a given Julian Day
 * Each Nakshatra spans 13°20' (13.333... degrees)
 */
export function calculate_nakshatra(
  jd: number,
  ayanamsha_mode: AyanamshaMode,
): NakshatraInfo;
/**
 * Calculate Tithi for a given Julian Day
 * Returns TithiInfo with index, name, paksha, and completion percentage
 */
export function calculate_tithi(jd: number): TithiInfo;
/**
 * Ayanamsha modes
 */
export enum AyanamshaMode {
  /**
   * Lahiri (Chitrapaksha)
   */
  Lahiri = 0,
  /**
   * Raman
   */
  Raman = 1,
  /**
   * Krishnamurti (KP)
   */
  Krishnamurti = 2,
  /**
   * True Chitrapaksha
   */
  TrueCitra = 27,
}
/**
 * Paksha (lunar fortnight)
 */
export enum Paksha {
  Shukla = 0,
  Krishna = 1,
}
export class DailyPanchang {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  sunrise: number;
  sunset: number;
  tithi_index: number;
  tithi_name: string;
  get tithi_end_time(): number | undefined;
  set tithi_end_time(value: number | null | undefined);
  nakshatra_index: number;
  nakshatra_name: string;
  get nakshatra_end_time(): number | undefined;
  set nakshatra_end_time(value: number | null | undefined);
  yoga_index: number;
  yoga_name: string;
  get yoga_end_time(): number | undefined;
  set yoga_end_time(value: number | null | undefined);
  vara_name: string;
  ayanamsha_value: number;
  muhurats: DayMuhurats;
}
export class DayMuhurats {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  rahu_kalam: Muhurat;
  yamaganda: Muhurat;
  gulika: Muhurat;
}
/**
 * Karana information
 */
export class KaranaInfo {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Karana index (1-60 per lunar month)
   */
  index: number;
  /**
   * Karana name
   */
  name: string;
  /**
   * Whether first or second half of Tithi
   */
  half: number;
}
/**
 * Location struct for geo-spatial calculations
 */
export class Location {
  free(): void;
  [Symbol.dispose](): void;
  constructor(latitude: number, longitude: number, altitude: number);
  latitude: number;
  longitude: number;
  altitude: number;
}
export class Muhurat {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  name: string;
  start: number;
  end: number;
}
/**
 * Nakshatra information
 */
export class NakshatraInfo {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Nakshatra index (1-27)
   */
  index: number;
  /**
   * Nakshatra name
   */
  name: string;
  /**
   * Planetary ruler
   */
  ruler: string;
  /**
   * Quality/nature
   */
  quality: string;
  /**
   * Pada (quarter, 1-4)
   */
  pada: number;
}
/**
 * Time interval
 */
export class TimeInterval {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Start time as Unix timestamp in ms
   */
  start_ms: number;
  /**
   * End time as Unix timestamp in ms
   */
  end_ms: number;
  /**
   * Duration in minutes
   */
  duration_minutes: number;
}
/**
 * Tithi information
 */
export class TithiInfo {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Tithi index (1-30)
   */
  index: number;
  /**
   * Tithi name
   */
  name: string;
  /**
   * Lunar fortnight
   */
  paksha: Paksha;
  /**
   * Completion percentage (0.0 to 1.0)
   */
  completion: number;
  readonly paksha_name: string;
}
/**
 * Vara (weekday) information
 */
export class VaraInfo {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Day index (0=Sunday, 6=Saturday)
   */
  index: number;
  /**
   * Sanskrit weekday name
   */
  name: string;
  /**
   * Planetary lord
   */
  lord: string;
}
export class WarDetails {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  planet1_id: number;
  planet1_name: string;
  planet1_long: number;
  planet1_mag: number;
  planet2_id: number;
  planet2_name: string;
  planet2_long: number;
  planet2_mag: number;
  longitude_diff: number;
  winner_id: number;
}
/**
 * Yoga information
 */
export class YogaInfo {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Yoga index (1-27)
   */
  index: number;
  /**
   * Yoga name
   */
  name: string;
}
