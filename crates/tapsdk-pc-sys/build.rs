use std::env;
use std::path::PathBuf;

fn main() {
    // Path to the reference directory containing headers and lib
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let reference_dir = PathBuf::from(&manifest_dir).join("../../reference");
    let reference_dir = reference_dir.canonicalize().expect("Failed to find reference directory");

    // Tell cargo to link against taptap_api.lib
    println!("cargo:rustc-link-search=native={}", reference_dir.display());
    println!("cargo:rustc-link-lib=dylib=taptap_api");

    // Tell cargo to rerun if the headers change
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed={}/taptap_api.h", reference_dir.display());
    println!("cargo:rerun-if-changed={}/taptap_cloudsave.h", reference_dir.display());

    // Generate bindings using bindgen
    let bindings = bindgen::Builder::default()
        // Input header
        .header("wrapper.h")
        // Add include path for the reference headers
        .clang_arg(format!("-I{}", reference_dir.display()))
        // Force C mode to avoid C++ enum class issues
        // The header uses #ifdef __cplusplus to provide C-compatible typedefs
        .clang_arg("-xc")
        // Ensure __cplusplus is not defined
        .clang_arg("-U__cplusplus")
        // Generate newtype enums for C enums (constants are exposed)
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        // Derive common traits
        .derive_debug(true)
        .derive_default(true)
        .derive_copy(true)
        // Handle packed structs
        .explicit_padding(true)
        // Allow all functions
        .allowlist_function("TapSDK_.*")
        .allowlist_function("TapUser_.*")
        .allowlist_function("TapApps_.*")
        .allowlist_function("TapDLC_.*")
        .allowlist_function("TapCloudSave.*")
        // Allow all types
        .allowlist_type(".*")
        // Allow all vars (constants)
        .allowlist_var(".*")
        // Parse callbacks
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Generate
        .generate()
        .expect("Failed to generate bindings");

    // Write bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");

    // Copy DLL to output directory for runtime
    let target_dir = env::var("OUT_DIR").unwrap();
    let target_path = PathBuf::from(&target_dir);
    
    // Go up from OUT_DIR to find the target directory
    // OUT_DIR is typically target/<profile>/build/<crate>/out
    let dll_src = reference_dir.join("taptap_api.dll");
    if dll_src.exists() {
        // Copy to multiple locations to ensure it's found at runtime
        if let Some(deps_dir) = target_path.ancestors().nth(3) {
            let dll_dest = deps_dir.join("taptap_api.dll");
            if let Err(e) = std::fs::copy(&dll_src, &dll_dest) {
                println!("cargo:warning=Failed to copy DLL to deps: {}", e);
            }
        }
    }
}
