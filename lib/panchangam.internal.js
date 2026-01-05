// @generated file from wasmbuild -- do not edit
// @ts-nocheck: generated
// deno-lint-ignore-file
// deno-fmt-ignore-file

let wasm;
export function __wbg_set_wasm(val) {
  wasm = val;
}

function debugString(val) {
  // primitive types
  const type = typeof val;
  if (type == "number" || type == "boolean" || val == null) {
    return `${val}`;
  }
  if (type == "string") {
    return `"${val}"`;
  }
  if (type == "symbol") {
    const description = val.description;
    if (description == null) {
      return "Symbol";
    } else {
      return `Symbol(${description})`;
    }
  }
  if (type == "function") {
    const name = val.name;
    if (typeof name == "string" && name.length > 0) {
      return `Function(${name})`;
    } else {
      return "Function";
    }
  }
  // objects
  if (Array.isArray(val)) {
    const length = val.length;
    let debug = "[";
    if (length > 0) {
      debug += debugString(val[0]);
    }
    for (let i = 1; i < length; i++) {
      debug += ", " + debugString(val[i]);
    }
    debug += "]";
    return debug;
  }
  // Test for built-in
  const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
  let className;
  if (builtInMatches && builtInMatches.length > 1) {
    className = builtInMatches[1];
  } else {
    // Failed to match the standard '[object ClassName]'
    return toString.call(val);
  }
  if (className == "Object") {
    // we're a user defined class or Object
    // JSON.stringify avoids problems with cycles, and is generally much
    // easier than looping through ownProperties of `val`.
    try {
      return "Object(" + JSON.stringify(val) + ")";
    } catch (_) {
      return "Object";
    }
  }
  // errors
  if (val instanceof Error) {
    return `${val.name}: ${val.message}\n${val.stack}`;
  }
  // TODO we could test for more things here, like `Set`s and `Map`s.
  return className;
}

let WASM_VECTOR_LEN = 0;

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
  if (
    cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0
  ) {
    cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8ArrayMemory0;
}

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

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
  if (
    cachedDataViewMemory0 === null ||
    cachedDataViewMemory0.buffer.detached === true ||
    (cachedDataViewMemory0.buffer.detached === undefined &&
      cachedDataViewMemory0.buffer !== wasm.memory.buffer)
  ) {
    cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
  }
  return cachedDataViewMemory0;
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

function addToExternrefTable0(obj) {
  const idx = wasm.__externref_table_alloc();
  wasm.__wbindgen_externrefs.set(idx, obj);
  return idx;
}

function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    const idx = addToExternrefTable0(e);
    wasm.__wbindgen_exn_store(idx);
  }
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

function _assertClass(instance, klass) {
  if (!(instance instanceof klass)) {
    throw new Error(`expected instance of ${klass.name}`);
  }
}
/**
 * Calculate Raahu, Yamaganda, and Gulika for a given day
 *
 * # Arguments
 * * `sunrise_ms` - Unix timestamp of sunrise in ms
 * * `sunset_ms` - Unix timestamp of sunset in ms
 * * `weekday` - 0=Sunday, 1=Monday, ..., 6=Saturday
 * @param {number} sunrise_ms
 * @param {number} sunset_ms
 * @param {number} weekday
 * @returns {DayMuhurats}
 */
export function calculate_muhurats(sunrise_ms, sunset_ms, weekday) {
  const ret = wasm.calculate_muhurats(sunrise_ms, sunset_ms, weekday);
  return DayMuhurats.__wrap(ret);
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

function isLikeNone(x) {
  return x === undefined || x === null;
}

function takeFromExternrefTable0(idx) {
  const value = wasm.__wbindgen_externrefs.get(idx);
  wasm.__externref_table_dealloc(idx);
  return value;
}
/**
 * @param {number} year
 * @param {number} month
 * @param {number} day
 * @param {Location} location
 * @param {number} ayan_mode
 * @returns {DailyPanchang}
 */
export function calculate_daily_panchang(
  year,
  month,
  day,
  location,
  ayan_mode,
) {
  _assertClass(location, Location);
  const ret = wasm.calculate_daily_panchang(
    year,
    month,
    day,
    location.__wbg_ptr,
    ayan_mode,
  );
  if (ret[2]) {
    throw takeFromExternrefTable0(ret[1]);
  }
  return DailyPanchang.__wrap(ret[0]);
}

/**
 * Check for Planetary War (Graha Yuddha)
 * Occurs when two Tara Grahas (Mars, Mercury, Jupiter, Venus, Saturn)
 * are within 1 degree of each other.
 * @param {number} jd
 * @param {number} ayan_mode
 * @returns {any}
 */
export function check_graha_yuddha(jd, ayan_mode) {
  const ret = wasm.check_graha_yuddha(jd, ayan_mode);
  return ret;
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
 * @param {number} tjd_ut
 * @param {number} ipl
 * @param {number} iflag
 * @returns {any}
 */
export function swe_calc_ut(tjd_ut, ipl, iflag) {
  const ret = wasm.swe_calc_ut(tjd_ut, ipl, iflag);
  if (ret[2]) {
    throw takeFromExternrefTable0(ret[1]);
  }
  return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {number} year
 * @param {number} month
 * @param {number} day
 * @param {number} hour
 * @param {number} gregflag
 * @returns {number}
 */
export function swe_julday(year, month, day, hour, gregflag) {
  const ret = wasm.swe_julday(year, month, day, hour, gregflag);
  return ret;
}

/**
 * @param {number} tjd
 * @param {number} gregflag
 * @returns {any}
 */
export function swe_revjul(tjd, gregflag) {
  const ret = wasm.swe_revjul(tjd, gregflag);
  return ret;
}

/**
 * @param {string} star
 * @param {number} tjd_ut
 * @param {number} iflag
 * @returns {any}
 */
export function swe_fixstar_ut(star, tjd_ut, iflag) {
  const ptr0 = passStringToWasm0(
    star,
    wasm.__wbindgen_malloc,
    wasm.__wbindgen_realloc,
  );
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.swe_fixstar_ut(ptr0, len0, tjd_ut, iflag);
  if (ret[2]) {
    throw takeFromExternrefTable0(ret[1]);
  }
  return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {number} tjd_ut
 * @param {number} ipl
 * @param {number} iflag
 * @returns {any}
 */
export function swe_pheno_ut(tjd_ut, ipl, iflag) {
  const ret = wasm.swe_pheno_ut(tjd_ut, ipl, iflag);
  if (ret[2]) {
    throw takeFromExternrefTable0(ret[1]);
  }
  return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {number} geolon
 * @param {number} geolat
 * @param {number} geoalt
 */
export function swe_set_topo(geolon, geolat, geoalt) {
  wasm.swe_set_topo(geolon, geolat, geoalt);
}

/**
 * @param {number} sid_mode
 * @param {number} t0
 * @param {number} ayan_t0
 */
export function swe_set_sid_mode(sid_mode, t0, ayan_t0) {
  wasm.swe_set_sid_mode(sid_mode, t0, ayan_t0);
}

/**
 * @param {number} tjd_ut
 * @returns {number}
 */
export function swe_get_ayanamsa_ut(tjd_ut) {
  const ret = wasm.swe_get_ayanamsa_ut(tjd_ut);
  return ret;
}

/**
 * @param {number} ipl
 * @returns {string}
 */
export function swe_get_planet_name(ipl) {
  let deferred1_0;
  let deferred1_1;
  try {
    const ret = wasm.swe_get_planet_name(ipl);
    deferred1_0 = ret[0];
    deferred1_1 = ret[1];
    return getStringFromWasm0(ret[0], ret[1]);
  } finally {
    wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
  }
}

/**
 * @param {number} tjd_ut
 * @returns {number}
 */
export function swe_sidtime(tjd_ut) {
  const ret = wasm.swe_sidtime(tjd_ut);
  return ret;
}

/**
 * Ayanamsha modes
 * @enum {0 | 1 | 2 | 27}
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
  TrueCitra: 27,
  "27": "TrueCitra",
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

const DailyPanchangFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) =>
    wasm.__wbg_dailypanchang_free(ptr >>> 0, 1)
  );

export class DailyPanchang {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(DailyPanchang.prototype);
    obj.__wbg_ptr = ptr;
    DailyPanchangFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }

  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    DailyPanchangFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_dailypanchang_free(ptr, 0);
  }
  /**
   * @returns {number}
   */
  get sunrise() {
    const ret = wasm.__wbg_get_dailypanchang_sunrise(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set sunrise(arg0) {
    wasm.__wbg_set_dailypanchang_sunrise(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {number}
   */
  get sunset() {
    const ret = wasm.__wbg_get_dailypanchang_sunset(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set sunset(arg0) {
    wasm.__wbg_set_dailypanchang_sunset(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {number}
   */
  get tithi_index() {
    const ret = wasm.__wbg_get_dailypanchang_tithi_index(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set tithi_index(arg0) {
    wasm.__wbg_set_dailypanchang_tithi_index(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {string}
   */
  get tithi_name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_dailypanchang_tithi_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * @param {string} arg0
   */
  set tithi_name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_dailypanchang_tithi_name(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * @returns {number | undefined}
   */
  get tithi_end_time() {
    const ret = wasm.__wbg_get_dailypanchang_tithi_end_time(this.__wbg_ptr);
    return ret[0] === 0 ? undefined : ret[1];
  }
  /**
   * @param {number | null} [arg0]
   */
  set tithi_end_time(arg0) {
    wasm.__wbg_set_dailypanchang_tithi_end_time(
      this.__wbg_ptr,
      !isLikeNone(arg0),
      isLikeNone(arg0) ? 0 : arg0,
    );
  }
  /**
   * @returns {number}
   */
  get nakshatra_index() {
    const ret = wasm.__wbg_get_dailypanchang_nakshatra_index(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set nakshatra_index(arg0) {
    wasm.__wbg_set_dailypanchang_nakshatra_index(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {string}
   */
  get nakshatra_name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_dailypanchang_nakshatra_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * @param {string} arg0
   */
  set nakshatra_name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_dailypanchang_nakshatra_name(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * @returns {number | undefined}
   */
  get nakshatra_end_time() {
    const ret = wasm.__wbg_get_dailypanchang_nakshatra_end_time(this.__wbg_ptr);
    return ret[0] === 0 ? undefined : ret[1];
  }
  /**
   * @param {number | null} [arg0]
   */
  set nakshatra_end_time(arg0) {
    wasm.__wbg_set_dailypanchang_nakshatra_end_time(
      this.__wbg_ptr,
      !isLikeNone(arg0),
      isLikeNone(arg0) ? 0 : arg0,
    );
  }
  /**
   * @returns {number}
   */
  get yoga_index() {
    const ret = wasm.__wbg_get_dailypanchang_yoga_index(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set yoga_index(arg0) {
    wasm.__wbg_set_dailypanchang_yoga_index(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {string}
   */
  get yoga_name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_dailypanchang_yoga_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * @param {string} arg0
   */
  set yoga_name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_dailypanchang_yoga_name(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * @returns {number | undefined}
   */
  get yoga_end_time() {
    const ret = wasm.__wbg_get_dailypanchang_yoga_end_time(this.__wbg_ptr);
    return ret[0] === 0 ? undefined : ret[1];
  }
  /**
   * @param {number | null} [arg0]
   */
  set yoga_end_time(arg0) {
    wasm.__wbg_set_dailypanchang_yoga_end_time(
      this.__wbg_ptr,
      !isLikeNone(arg0),
      isLikeNone(arg0) ? 0 : arg0,
    );
  }
  /**
   * @returns {string}
   */
  get vara_name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_dailypanchang_vara_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * @param {string} arg0
   */
  set vara_name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_dailypanchang_vara_name(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * @returns {number}
   */
  get ayanamsha_value() {
    const ret = wasm.__wbg_get_dailypanchang_ayanamsha_value(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set ayanamsha_value(arg0) {
    wasm.__wbg_set_dailypanchang_ayanamsha_value(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {DayMuhurats}
   */
  get muhurats() {
    const ret = wasm.__wbg_get_dailypanchang_muhurats(this.__wbg_ptr);
    return DayMuhurats.__wrap(ret);
  }
  /**
   * @param {DayMuhurats} arg0
   */
  set muhurats(arg0) {
    _assertClass(arg0, DayMuhurats);
    var ptr0 = arg0.__destroy_into_raw();
    wasm.__wbg_set_dailypanchang_muhurats(this.__wbg_ptr, ptr0);
  }
}
if (Symbol.dispose) {
  DailyPanchang.prototype[Symbol.dispose] = DailyPanchang.prototype.free;
}

const DayMuhuratsFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) =>
    wasm.__wbg_daymuhurats_free(ptr >>> 0, 1)
  );

export class DayMuhurats {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(DayMuhurats.prototype);
    obj.__wbg_ptr = ptr;
    DayMuhuratsFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }

  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    DayMuhuratsFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_daymuhurats_free(ptr, 0);
  }
  /**
   * @returns {Muhurat}
   */
  get rahu_kalam() {
    const ret = wasm.__wbg_get_daymuhurats_rahu_kalam(this.__wbg_ptr);
    return Muhurat.__wrap(ret);
  }
  /**
   * @param {Muhurat} arg0
   */
  set rahu_kalam(arg0) {
    _assertClass(arg0, Muhurat);
    var ptr0 = arg0.__destroy_into_raw();
    wasm.__wbg_set_daymuhurats_rahu_kalam(this.__wbg_ptr, ptr0);
  }
  /**
   * @returns {Muhurat}
   */
  get yamaganda() {
    const ret = wasm.__wbg_get_daymuhurats_yamaganda(this.__wbg_ptr);
    return Muhurat.__wrap(ret);
  }
  /**
   * @param {Muhurat} arg0
   */
  set yamaganda(arg0) {
    _assertClass(arg0, Muhurat);
    var ptr0 = arg0.__destroy_into_raw();
    wasm.__wbg_set_daymuhurats_yamaganda(this.__wbg_ptr, ptr0);
  }
  /**
   * @returns {Muhurat}
   */
  get gulika() {
    const ret = wasm.__wbg_get_daymuhurats_gulika(this.__wbg_ptr);
    return Muhurat.__wrap(ret);
  }
  /**
   * @param {Muhurat} arg0
   */
  set gulika(arg0) {
    _assertClass(arg0, Muhurat);
    var ptr0 = arg0.__destroy_into_raw();
    wasm.__wbg_set_daymuhurats_gulika(this.__wbg_ptr, ptr0);
  }
}
if (Symbol.dispose) {
  DayMuhurats.prototype[Symbol.dispose] = DayMuhurats.prototype.free;
}

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

const MuhuratFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) => wasm.__wbg_muhurat_free(ptr >>> 0, 1));

export class Muhurat {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(Muhurat.prototype);
    obj.__wbg_ptr = ptr;
    MuhuratFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }

  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    MuhuratFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_muhurat_free(ptr, 0);
  }
  /**
   * @returns {string}
   */
  get name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_muhurat_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * @param {string} arg0
   */
  set name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_muhurat_name(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * @returns {number}
   */
  get start() {
    const ret = wasm.__wbg_get_muhurat_start(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set start(arg0) {
    wasm.__wbg_set_muhurat_start(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {number}
   */
  get end() {
    const ret = wasm.__wbg_get_muhurat_end(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set end(arg0) {
    wasm.__wbg_set_muhurat_end(this.__wbg_ptr, arg0);
  }
}
if (Symbol.dispose) Muhurat.prototype[Symbol.dispose] = Muhurat.prototype.free;

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

const WarDetailsFinalization = (typeof FinalizationRegistry === "undefined")
  ? { register: () => {}, unregister: () => {} }
  : new FinalizationRegistry((ptr) => wasm.__wbg_wardetails_free(ptr >>> 0, 1));

export class WarDetails {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    WarDetailsFinalization.unregister(this);
    return ptr;
  }

  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_wardetails_free(ptr, 0);
  }
  /**
   * @returns {number}
   */
  get planet1_id() {
    const ret = wasm.__wbg_get_wardetails_planet1_id(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set planet1_id(arg0) {
    wasm.__wbg_set_wardetails_planet1_id(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {string}
   */
  get planet1_name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_wardetails_planet1_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * @param {string} arg0
   */
  set planet1_name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_wardetails_planet1_name(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * @returns {number}
   */
  get planet1_long() {
    const ret = wasm.__wbg_get_wardetails_planet1_long(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set planet1_long(arg0) {
    wasm.__wbg_set_wardetails_planet1_long(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {number}
   */
  get planet1_mag() {
    const ret = wasm.__wbg_get_wardetails_planet1_mag(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set planet1_mag(arg0) {
    wasm.__wbg_set_wardetails_planet1_mag(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {number}
   */
  get planet2_id() {
    const ret = wasm.__wbg_get_wardetails_planet2_id(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set planet2_id(arg0) {
    wasm.__wbg_set_wardetails_planet2_id(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {string}
   */
  get planet2_name() {
    let deferred1_0;
    let deferred1_1;
    try {
      const ret = wasm.__wbg_get_wardetails_planet2_name(this.__wbg_ptr);
      deferred1_0 = ret[0];
      deferred1_1 = ret[1];
      return getStringFromWasm0(ret[0], ret[1]);
    } finally {
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  /**
   * @param {string} arg0
   */
  set planet2_name(arg0) {
    const ptr0 = passStringToWasm0(
      arg0,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.__wbg_set_wardetails_planet2_name(this.__wbg_ptr, ptr0, len0);
  }
  /**
   * @returns {number}
   */
  get planet2_long() {
    const ret = wasm.__wbg_get_wardetails_planet2_long(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set planet2_long(arg0) {
    wasm.__wbg_set_wardetails_planet2_long(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {number}
   */
  get planet2_mag() {
    const ret = wasm.__wbg_get_wardetails_planet2_mag(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set planet2_mag(arg0) {
    wasm.__wbg_set_wardetails_planet2_mag(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {number}
   */
  get longitude_diff() {
    const ret = wasm.__wbg_get_wardetails_longitude_diff(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set longitude_diff(arg0) {
    wasm.__wbg_set_wardetails_longitude_diff(this.__wbg_ptr, arg0);
  }
  /**
   * @returns {number}
   */
  get winner_id() {
    const ret = wasm.__wbg_get_wardetails_winner_id(this.__wbg_ptr);
    return ret;
  }
  /**
   * @param {number} arg0
   */
  set winner_id(arg0) {
    wasm.__wbg_set_wardetails_winner_id(this.__wbg_ptr, arg0);
  }
}
if (Symbol.dispose) {
  WarDetails.prototype[Symbol.dispose] = WarDetails.prototype.free;
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
    wasm.__wbg_set_yogainfo_name(this.__wbg_ptr, ptr0, len0);
  }
}
if (Symbol.dispose) {
  YogaInfo.prototype[Symbol.dispose] = YogaInfo.prototype.free;
}

export function __wbg___wbindgen_debug_string_df47ffb5e35e6763(arg0, arg1) {
  const ret = debugString(arg1);
  const ptr1 = passStringToWasm0(
    ret,
    wasm.__wbindgen_malloc,
    wasm.__wbindgen_realloc,
  );
  const len1 = WASM_VECTOR_LEN;
  getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
  getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}

export function __wbg___wbindgen_throw_b855445ff6a94295(arg0, arg1) {
  throw new Error(getStringFromWasm0(arg0, arg1));
}

export function __wbg_new_1acc0b6eea89d040() {
  const ret = new Object();
  return ret;
}

export function __wbg_new_e17d9f43105b08be() {
  const ret = new Array();
  return ret;
}

export function __wbg_set_3f1d0b984ed272ed(arg0, arg1, arg2) {
  arg0[arg1] = arg2;
}

export function __wbg_set_c213c871859d6500(arg0, arg1, arg2) {
  arg0[arg1 >>> 0] = arg2;
}

export function __wbg_set_c2abbebe8b9ebee1() {
  return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(arg0, arg1, arg2);
    return ret;
  }, arguments);
}

export function __wbindgen_cast_2241b6af4c4b2941(arg0, arg1) {
  // Cast intrinsic for `Ref(String) -> Externref`.
  const ret = getStringFromWasm0(arg0, arg1);
  return ret;
}

export function __wbindgen_cast_d6cd19b81560fd6e(arg0) {
  // Cast intrinsic for `F64 -> Externref`.
  const ret = arg0;
  return ret;
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
