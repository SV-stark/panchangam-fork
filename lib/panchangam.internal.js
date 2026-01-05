// @generated file from wasmbuild -- do not edit
// @ts-nocheck: generated
// deno-lint-ignore-file
// deno-fmt-ignore-file

let wasm;
export function __wbg_set_wasm(val) {
  wasm = val;
}

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
  if (
    cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0
  ) {
    cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder("utf-8", {
  ignoreBOM: true,
  fatal: true,
});

cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
  numBytesDecoded += len;
  if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
    cachedTextDecoder = new TextDecoder("utf-8", {
      ignoreBOM: true,
      fatal: true,
    });
    cachedTextDecoder.decode();
    numBytesDecoded = len;
  }
  return cachedTextDecoder.decode(
    getUint8ArrayMemory0().subarray(ptr, ptr + len),
  );
}

function getStringFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return decodeText(ptr, len);
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder();

if (!("encodeInto" in cachedTextEncoder)) {
  cachedTextEncoder.encodeInto = function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
      read: arg.length,
      written: buf.length,
    };
  };
}

function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr = malloc(buf.length, 1) >>> 0;
    getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
  }

  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0;

  const mem = getUint8ArrayMemory0();

  let offset = 0;

  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7F) break;
    mem[ptr + offset] = code;
  }

  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
    const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
    const ret = cachedTextEncoder.encodeInto(arg, view);

    offset += ret.written;
    ptr = realloc(ptr, len, offset, 1) >>> 0;
  }

  WASM_VECTOR_LEN = offset;
  return ptr;
}
/**
 * Calculate Nakshatra for a given Julian Day
 * Each Nakshatra spans 13°20' (13.333... degrees)
 * @param {number} jd
 * @param {AyanamshaMode} ayanamsha_mode
 * @returns {NakshatraInfo}
 */
export function calculate_nakshatra(jd, ayanamsha_mode) {
  const ret = wasm.calculate_nakshatra(jd, ayanamsha_mode);
  return NakshatraInfo.__wrap(ret);
}

/**
 * Calculate Rahu Kaal for a given day
 * sunrise_ms and sunset_ms are Unix timestamps in milliseconds
 * weekday is 0=Sunday, 6=Saturday
 * @param {number} sunrise_ms
 * @param {number} sunset_ms
 * @param {number} weekday
 * @returns {TimeInterval}
 */
export function calculate_rahu_kaal(sunrise_ms, sunset_ms, weekday) {
  const ret = wasm.calculate_rahu_kaal(sunrise_ms, sunset_ms, weekday);
  return TimeInterval.__wrap(ret);
}

/**
 * Calculate Yamaganda for a given day
 * @param {number} sunrise_ms
 * @param {number} sunset_ms
 * @param {number} weekday
 * @returns {TimeInterval}
 */
export function calculate_yamaganda(sunrise_ms, sunset_ms, weekday) {
  const ret = wasm.calculate_yamaganda(sunrise_ms, sunset_ms, weekday);
  return TimeInterval.__wrap(ret);
}

/**
 * Calculate Gulika Kaal for a given day
 * @param {number} sunrise_ms
 * @param {number} sunset_ms
 * @param {number} weekday
 * @returns {TimeInterval}
 */
export function calculate_gulika(sunrise_ms, sunset_ms, weekday) {
  const ret = wasm.calculate_gulika(sunrise_ms, sunset_ms, weekday);
  return TimeInterval.__wrap(ret);
}

/**
 * Get Ayanamsha value for a given mode and Julian Day
 * @param {AyanamshaMode} mode
 * @param {number} jd
 * @returns {number}
 */
export function get_ayanamsha(mode, jd) {
  const ret = wasm.get_ayanamsha(mode, jd);
  return ret;
}

/**
 * Calculate Tithi for a given Julian Day
 * Returns TithiInfo with index, name, paksha, and completion percentage
 * @param {number} jd
 * @returns {TithiInfo}
 */
export function calculate_tithi(jd) {
  const ret = wasm.calculate_tithi(jd);
  return TithiInfo.__wrap(ret);
}

/**
 * Calculate Karana for a given Julian Day
 * There are 60 Karanas per lunar month (2 per Tithi)
 * @param {number} jd
 * @returns {KaranaInfo}
 */
export function calculate_karana(jd) {
  const ret = wasm.calculate_karana(jd);
  return KaranaInfo.__wrap(ret);
}

/**
 * Get the library version from Swiss Ephemeris
 * @returns {string}
 */
export function get_version() {
  let deferred1_0;
  let deferred1_1;
  try {
    const ret = wasm.get_version();
    deferred1_0 = ret[0];
    deferred1_1 = ret[1];
    return getStringFromWasm0(ret[0], ret[1]);
  } finally {
    wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
  }
}

function _assertClass(instance, klass) {
  if (!(instance instanceof klass)) {
    throw new Error(`expected instance of ${klass.name}`);
  }
}
/**
 * Calculate sunrise time for a given date and location
 * Returns Unix timestamp in milliseconds
 * @param {number} year
 * @param {number} month
 * @param {number} day
 * @param {Location} location
 * @returns {number}
 */
export function calculate_sunrise(year, month, day, location) {
  _assertClass(location, Location);
  const ret = wasm.calculate_sunrise(year, month, day, location.__wbg_ptr);
  return ret;
}

/**
 * Calculate sunset time for a given date and location
 * Returns Unix timestamp in milliseconds
 * @param {number} year
 * @param {number} month
 * @param {number} day
 * @param {Location} location
 * @returns {number}
 */
export function calculate_sunset(year, month, day, location) {
  _assertClass(location, Location);
  const ret = wasm.calculate_sunset(year, month, day, location.__wbg_ptr);
  return ret;
}

/**
 * Calculate Yoga for a given Julian Day
 * Formula: Yoga = floor((Moon_long + Sun_long) / 13.333) + 1
 * @param {number} jd
 * @param {AyanamshaMode} ayanamsha_mode
 * @returns {YogaInfo}
 */
export function calculate_yoga(jd, ayanamsha_mode) {
  const ret = wasm.calculate_yoga(jd, ayanamsha_mode);
  return YogaInfo.__wrap(ret);
}

/**
 * Calculate Vara (weekday) for a given Julian Day
 * Note: This returns the astronomical weekday.
 * For Vedic Vara, compare with sunrise time.
 * @param {number} jd
 * @returns {VaraInfo}
 */
export function calculate_vara(jd) {
  const ret = wasm.calculate_vara(jd);
  return VaraInfo.__wrap(ret);
}

/**
 * Ayanamsha modes
 * @enum {0 | 1 | 2 | 3}
 */
export const AyanamshaMode = Object.freeze({
  /**
   * Lahiri (Chitrapaksha)
   */
  Lahiri: 0,
  "0": "Lahiri",
  /**
   * Raman
   */
  Raman: 1,
  "1": "Raman",
  /**
   * Krishnamurti (KP)
   */
  Krishnamurti: 2,
  "2": "Krishnamurti",
  /**
   * True Chitrapaksha
   */
  TrueChitrapaksha: 3,
  "3": "TrueChitrapaksha",
});
/**
 * Paksha (lunar fortnight)
 * @enum {0 | 1}
 */
export const Paksha = Object.freeze({
  Shukla: 0,
  "0": "Shukla",
  Krishna: 1,
  "1": "Krishna",
});

const KaranaInfoFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) => wasm.__wbg_karanainfo_free(ptr >>> 0, 1));
/**
 * Karana information
 */
export class KaranaInfo {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(KaranaInfo.prototype);
    obj.__wbg_ptr = ptr;
    KaranaInfoFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }

  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    KaranaInfoFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_karanainfo_free(ptr, 0);
  }
  /**
   * Karana index (1-60 per lunar month)
   * @returns {number}
   */
  get index() {
    const ret = wasm.__wbg_get_karanainfo_index(this.__wbg_ptr);
    return ret;
  }
  /**
   * Karana index (1-60 per lunar month)
   * @param {number} arg0
   */
  set index(arg0) {
    wasm.__wbg_set_karanainfo_index(this.__wbg_ptr, arg0);
  }
  /**
   * Karana name
   * @returns {string}
   */
  get name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_karanainfo_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * Karana name
   * @param {string} arg0
   */
  set name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_karanainfo_name(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * Whether first or second half of Tithi
   * @returns {number}
   */
  get half() {
    const ret = wasm.__wbg_get_karanainfo_half(this.__wbg_ptr);
    return ret;
  }
  /**
   * Whether first or second half of Tithi
   * @param {number} arg0
   */
  set half(arg0) {
    wasm.__wbg_set_karanainfo_half(this.__wbg_ptr, arg0);
  }
}
if (Symbol.dispose) {
  KaranaInfo.prototype[Symbol.dispose] = KaranaInfo.prototype.free;
}

const LocationFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) => wasm.__wbg_location_free(ptr >>> 0, 1));
/**
 * Location struct for geo-spatial calculations
 */
export class Location {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    LocationFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_location_free(ptr, 0);
  }
  /**
   * @returns {number}
   */
  get latitude() {
    const ret = wasm.__wbg_get_location_latitude(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set latitude(arg0) {
    wasm.__wbg_set_location_latitude(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {number}
   */
  get longitude() {
    const ret = wasm.__wbg_get_location_longitude(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set longitude(arg0) {
    wasm.__wbg_set_location_longitude(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {number}
   */
  get altitude() {
    const ret = wasm.__wbg_get_location_altitude(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set altitude(arg0) {
    wasm.__wbg_set_location_altitude(this.__wbg_ptr, arg0);
  }
  /**
   * @param {number} latitude
   * @param {number} longitude
   * @param {number} altitude
   */
  constructor(latitude, longitude, altitude) {
    const ret = wasm.location_new(latitude, longitude, altitude);
    this.__wbg_ptr = ret >>> 0;
    LocationFinalization.register(this, this.__wbg_ptr, this);
    return this;
  }
}
if (Symbol.dispose) {
  Location.prototype[Symbol.dispose] = Location.prototype.free;
}

const NakshatraInfoFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) =>
    wasm.__wbg_nakshatrainfo_free(ptr >>> 0, 1)
  );
/**
 * Nakshatra information
 */
export class NakshatraInfo {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(NakshatraInfo.prototype);
    obj.__wbg_ptr = ptr;
    NakshatraInfoFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }

  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    NakshatraInfoFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_nakshatrainfo_free(ptr, 0);
  }
  /**
   * Nakshatra index (1-27)
   * @returns {number}
   */
  get index() {
    const ret = wasm.__wbg_get_nakshatrainfo_index(this.__wbg_ptr);
    return ret;
  }
  /**
   * Nakshatra index (1-27)
   * @param {number} arg0
   */
  set index(arg0) {
    wasm.__wbg_set_nakshatrainfo_index(this.__wbg_ptr, arg0);
  }
  /**
   * Nakshatra name
   * @returns {string}
   */
  get name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_nakshatrainfo_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * Nakshatra name
   * @param {string} arg0
   */
  set name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_nakshatrainfo_name(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * Planetary ruler
   * @returns {string}
   */
  get ruler() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_nakshatrainfo_ruler(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * Planetary ruler
   * @param {string} arg0
   */
  set ruler(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_nakshatrainfo_ruler(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * Quality/nature
   * @returns {string}
   */
  get quality() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_nakshatrainfo_quality(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * Quality/nature
   * @param {string} arg0
   */
  set quality(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_nakshatrainfo_quality(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * Pada (quarter, 1-4)
   * @returns {number}
   */
  get pada() {
    const ret = wasm.__wbg_get_nakshatrainfo_pada(this.__wbg_ptr);
    return ret;
  }
  /**
   * Pada (quarter, 1-4)
   * @param {number} arg0
   */
  set pada(arg0) {
    wasm.__wbg_set_nakshatrainfo_pada(this.__wbg_ptr, arg0);
  }
}
if (Symbol.dispose) {
  NakshatraInfo.prototype[Symbol.dispose] = NakshatraInfo.prototype.free;
}

const TimeIntervalFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) =>
    wasm.__wbg_timeinterval_free(ptr >>> 0, 1)
  );
/**
 * Time interval
 */
export class TimeInterval {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(TimeInterval.prototype);
    obj.__wbg_ptr = ptr;
    TimeIntervalFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }

  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TimeIntervalFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_timeinterval_free(ptr, 0);
  }
  /**
   * Start time as Unix timestamp in ms
   * @returns {number}
   */
  get start_ms() {
    const ret = wasm.__wbg_get_timeinterval_start_ms(this.__wbg_ptr);
    return ret;
  }
  /**
   * Start time as Unix timestamp in ms
   * @param {number} arg0
   */
  set start_ms(arg0) {
    wasm.__wbg_set_timeinterval_start_ms(this.__wbg_ptr, arg0);
  }
  /**
   * End time as Unix timestamp in ms
   * @returns {number}
   */
  get end_ms() {
    const ret = wasm.__wbg_get_timeinterval_end_ms(this.__wbg_ptr);
    return ret;
  }
  /**
   * End time as Unix timestamp in ms
   * @param {number} arg0
   */
  set end_ms(arg0) {
    wasm.__wbg_set_timeinterval_end_ms(this.__wbg_ptr, arg0);
  }
  /**
   * Duration in minutes
   * @returns {number}
   */
  get duration_minutes() {
    const ret = wasm.__wbg_get_timeinterval_duration_minutes(this.__wbg_ptr);
    return ret;
  }
  /**
   * Duration in minutes
   * @param {number} arg0
   */
  set duration_minutes(arg0) {
    wasm.__wbg_set_timeinterval_duration_minutes(this.__wbg_ptr, arg0);
  }
}
if (Symbol.dispose) {
  TimeInterval.prototype[Symbol.dispose] = TimeInterval.prototype.free;
}

const TithiInfoFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) => wasm.__wbg_tithiinfo_free(ptr >>> 0, 1));
/**
 * Tithi information
 */
export class TithiInfo {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(TithiInfo.prototype);
    obj.__wbg_ptr = ptr;
    TithiInfoFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }

  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TithiInfoFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tithiinfo_free(ptr, 0);
  }
  /**
   * Tithi index (1-30)
   * @returns {number}
   */
  get index() {
    const ret = wasm.__wbg_get_tithiinfo_index(this.__wbg_ptr);
    return ret;
  }
  /**
   * Tithi index (1-30)
   * @param {number} arg0
   */
  set index(arg0) {
    wasm.__wbg_set_tithiinfo_index(this.__wbg_ptr, arg0);
  }
  /**
   * Tithi name
   * @returns {string}
   */
  get name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_tithiinfo_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * Tithi name
   * @param {string} arg0
   */
  set name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_tithiinfo_name(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * Lunar fortnight
   * @returns {Paksha}
   */
  get paksha() {
    const ret = wasm.__wbg_get_tithiinfo_paksha(this.__wbg_ptr);
    return ret;
  }
  /**
   * Lunar fortnight
   * @param {Paksha} arg0
   */
  set paksha(arg0) {
    wasm.__wbg_set_tithiinfo_paksha(this.__wbg_ptr, arg0);
  }
  /**
   * Completion percentage (0.0 to 1.0)
   * @returns {number}
   */
  get completion() {
    const ret = wasm.__wbg_get_tithiinfo_completion(this.__wbg_ptr);
    return ret;
  }
  /**
   * Completion percentage (0.0 to 1.0)
   * @param {number} arg0
   */
  set completion(arg0) {
    wasm.__wbg_set_tithiinfo_completion(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {string}
   */
  get paksha_name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.tithiinfo_paksha_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
}
if (Symbol.dispose) {
  TithiInfo.prototype[Symbol.dispose] = TithiInfo.prototype.free;
}

const VaraInfoFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) => wasm.__wbg_varainfo_free(ptr >>> 0, 1));
/**
 * Vara (weekday) information
 */
export class VaraInfo {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(VaraInfo.prototype);
    obj.__wbg_ptr = ptr;
    VaraInfoFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }

  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    VaraInfoFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_varainfo_free(ptr, 0);
  }
  /**
   * Day index (0=Sunday, 6=Saturday)
   * @returns {number}
   */
  get index() {
    const ret = wasm.__wbg_get_varainfo_index(this.__wbg_ptr);
    return ret;
  }
  /**
   * Day index (0=Sunday, 6=Saturday)
   * @param {number} arg0
   */
  set index(arg0) {
    wasm.__wbg_set_varainfo_index(this.__wbg_ptr, arg0);
  }
  /**
   * Sanskrit weekday name
   * @returns {string}
   */
  get name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_varainfo_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * Sanskrit weekday name
   * @param {string} arg0
   */
  set name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_varainfo_name(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * Planetary lord
   * @returns {string}
   */
  get lord() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_varainfo_lord(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * Planetary lord
   * @param {string} arg0
   */
  set lord(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_varainfo_lord(this.__wbg_ptr, ptr0, len0);
  }
}
if (Symbol.dispose) {
  VaraInfo.prototype[Symbol.dispose] = VaraInfo.prototype.free;
}

const YogaInfoFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) => wasm.__wbg_yogainfo_free(ptr >>> 0, 1));
/**
 * Yoga information
 */
export class YogaInfo {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(YogaInfo.prototype);
    obj.__wbg_ptr = ptr;
    YogaInfoFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }

  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    YogaInfoFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_yogainfo_free(ptr, 0);
  }
  /**
   * Yoga index (1-27)
   * @returns {number}
   */
  get index() {
    const ret = wasm.__wbg_get_yogainfo_index(this.__wbg_ptr);
    return ret;
  }
  /**
   * Yoga index (1-27)
   * @param {number} arg0
   */
  set index(arg0) {
    wasm.__wbg_set_yogainfo_index(this.__wbg_ptr, arg0);
  }
  /**
   * Yoga name
   * @returns {string}
   */
  get name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_yogainfo_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * Yoga name
   * @param {string} arg0
   */
  set name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_varainfo_name(this.__wbg_ptr, ptr0, len0);
  }
}
if (Symbol.dispose) {
  YogaInfo.prototype[Symbol.dispose] = YogaInfo.prototype.free;
}

export function __wbg___wbindgen_throw_b855445ff6a94295(arg0, arg1) {
  throw new Error(getStringFromWasm0(arg0, arg1));
}

export function __wbindgen_init_externref_table() {
  const table = wasm.__wbindgen_externrefs;
  const offset = table.grow(4);
  table.set(0, undefined);
  table.set(offset + 0, undefined);
  table.set(offset + 1, null);
  table.set(offset + 2, true);
  table.set(offset + 3, false);
}
