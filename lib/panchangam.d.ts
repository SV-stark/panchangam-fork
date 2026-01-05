// @generated file from wasmbuild -- do not edit
// deno-lint-ignore-file
// deno-fmt-ignore-file

/**
 * Calculate Nakshatra for a given Julian Day
 * Each Nakshatra spans 13°20' (13.333... degrees)
 */
export function calculate_nakshatra(
  jd: number,
  ayanamsha_mode: AyanamshaMode,
): NakshatraInfo;
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
 * Get Ayanamsha value for a given mode and Julian Day
 */
export function get_ayanamsha(mode: AyanamshaMode, jd: number): number;
/**
 * Calculate Tithi for a given Julian Day
 * Returns TithiInfo with index, name, paksha, and completion percentage
 */
export function calculate_tithi(jd: number): TithiInfo;
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
 * Calculate Vara (weekday) for a given Julian Day
 * Note: This returns the astronomical weekday.
 * For Vedic Vara, compare with sunrise time.
 */
export function calculate_vara(jd: number): VaraInfo;
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
  TrueChitrapaksha = 3,
}
/**
 * Paksha (lunar fortnight)
 */
export enum Paksha {
  Shukla = 0,
  Krishna = 1,
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
