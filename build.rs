use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut build = cc::Build::new();
    
    // Swiss Ephemeris Source Files - only the core calculation files
    let swe_src = "src/libswe";
    let c_files = [
        "swecl.c", "swedate.c", "swehel.c", "swehouse.c", "swejpl.c",
        "swemmoon.c", "swemplan.c", "sweph.c", "swephlib.c",
        "stub.c",
    ];

    for file in &c_files {
        build.file(format!("{}/{}", swe_src, file));
    }

    // Include paths
    build.include(swe_src);

    // Target specific configuration
    if target.contains("wasm32") {
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
             
        // Define macros
        build.define("NO_MOSHIER", None); // Use JPL ephemeris only
    }

    build.compile("swe");

    // Generate Bindings
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let wasm_includes = PathBuf::from(&manifest_dir).join("wasm-includes");
    
    println!("cargo:warning=Generating bindings from {}/swe_wrapper.h", swe_src);
    println!("cargo:warning=Include paths: {} and {}", swe_src, wasm_includes.display());
    
    // Bindgen only needs to parse headers, not compile for Wasm
    // Use host target (aarch64-apple-darwin) for parsing
    let bindings = bindgen::Builder::default()
        .header(format!("{}/swe_wrapper.h", swe_src))
        .clang_arg(format!("-I{}", swe_src))
        .clang_arg(format!("-I{}", wasm_includes.display()))
        // Temporarily remove allowlists to see what gets generated
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .ctypes_prefix("libc")
        .generate_comments(false)
        .layout_tests(false)
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");
    
    println!("cargo:warning=Bindings written to {:?}", bindings_path);
}
