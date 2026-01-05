use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut build = cc::Build::new();
    
    // Source Directories
    let swe_src = "vendor/swisseph";
    let wrapper_src = "src/wrapper";

    // Swiss Ephemeris Source Files (Upstream)
    let c_files = [
        "swecl.c", "swedate.c", "swehel.c", "swehouse.c", "swejpl.c",
        "swemmoon.c", "swemplan.c", "sweph.c", "swephlib.c",
    ];

    for file in &c_files {
        build.file(format!("{}/{}", swe_src, file));
    }
    
    // Include paths
    build.include(swe_src);
    build.include(wrapper_src); // needed for wrapper header

    // Define macros
    build.define("NO_MOSHIER", None); // Use JPL ephemeris only

    // Rename colliding symbols
    let renaming = [
        ("swe_calc_ut", "impl_swe_calc_ut"),
        ("swe_julday", "impl_swe_julday"),
        ("swe_revjul", "impl_swe_revjul"),
        ("swe_fixstar_ut", "impl_swe_fixstar_ut"),
        ("swe_pheno_ut", "impl_swe_pheno_ut"),
        ("swe_set_topo", "impl_swe_set_topo"),
        ("swe_set_sid_mode", "impl_swe_set_sid_mode"),
        ("swe_get_ayanamsa_ut", "impl_swe_get_ayanamsa_ut"),
        ("swe_get_planet_name", "impl_swe_get_planet_name"),
        ("swe_sidtime", "impl_swe_sidtime"),
        ("swe_version", "impl_swe_version"),
    ];
    
    for (orig, renamed) in &renaming {
        build.define(orig, Some(*renamed));
    }

    // Target specific configuration
    if target.contains("wasm32") {
        // Wrapper Code (Custom) - Only for Wasm
        build.file(format!("{}/stub.c", wrapper_src));

        // Use our custom shims
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let wasm_includes = PathBuf::from(manifest_dir).join("wasm-includes");
        build.include(&wasm_includes);
        
        // Flags to suppress warnings and ensure Wasm compatibility
        build.flag("-Wno-implicit-function-declaration")
             .flag("-Wno-int-conversion")
             .flag("-Wno-unused-variable")
             .flag("-Wno-unused-parameter")
             .flag("-Wno-sign-compare")
             .flag("-Wno-missing-braces")
             .flag("-Wno-parentheses")
             .flag("-Wno-misleading-indentation");
    }

    build.compile("swe");
}
