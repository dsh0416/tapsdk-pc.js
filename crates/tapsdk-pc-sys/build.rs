use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // Only generate real bindings and link DLL on Windows
    if target_os == "windows" {
        build_windows();
    } else {
        // Generate stub bindings for non-Windows platforms
        build_stub();
    }
}

#[cfg(target_os = "windows")]
fn build_windows() {
    // Path to the SDK directory containing headers and lib (bundled with crate)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let sdk_dir = PathBuf::from(&manifest_dir).join("sdk");
    let sdk_dir = sdk_dir
        .canonicalize()
        .expect("Failed to find sdk directory");

    // Tell cargo to link against taptap_api.lib
    println!("cargo:rustc-link-search=native={}", sdk_dir.display());
    println!("cargo:rustc-link-lib=dylib=taptap_api");

    // Tell cargo to rerun if the headers change
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed={}/taptap_api.h", sdk_dir.display());
    println!(
        "cargo:rerun-if-changed={}/taptap_cloudsave.h",
        sdk_dir.display()
    );

    // Generate bindings using bindgen
    let bindings = bindgen::Builder::default()
        // Input header
        .header("wrapper.h")
        // Add include path for the reference headers
        .clang_arg(format!("-I{}", sdk_dir.display()))
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
    let dll_src = sdk_dir.join("taptap_api.dll");
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

#[cfg(not(target_os = "windows"))]
fn build_windows() {
    build_stub();
}

fn build_stub() {
    // Generate stub bindings for non-Windows platforms
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let stub_bindings = r#"
// Stub bindings for non-Windows platforms
// TapTap PC SDK only supports Windows

use std::os::raw::{c_char, c_void};

// Stub types - these are never instantiated, all functions panic
pub type TapCloudSaveHandle = *mut c_void;
pub type ITapCloudSave = c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapSDK_Error {
    pub code: i64,
    pub message: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapSystemStateNotification {
    pub state: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizeFinishedResponse {
    pub is_cancel: bool,
    pub error: [c_char; 256],
    pub token_type: [c_char; 64],
    pub kid: [c_char; 256],
    pub mac_key: [c_char; 256],
    pub mac_algorithm: [c_char; 64],
    pub scope: [c_char; 256],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GamePlayableStatusChangedResponse {
    pub is_playable: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DLCPlayableStatusChangedResponse {
    pub dlc_id: [c_char; 256],
    pub is_playable: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveInfo {
    pub uuid: *const c_char,
    pub file_id: *const c_char,
    pub name: *const c_char,
    pub save_size: u32,
    pub cover_size: u32,
    pub summary: *const c_char,
    pub extra: *const c_char,
    pub playtime: u32,
    pub created_time: u32,
    pub modified_time: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveListResponse {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub saves: *const TapCloudSaveInfo,
    pub save_count: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveCreateResponse {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub save: *const TapCloudSaveInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveDeleteResponse {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub uuid: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveGetFileResponse {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub data: *const c_void,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveCreateRequest {
    pub name: *const c_char,
    pub summary: *const c_char,
    pub extra: *const c_char,
    pub playtime: u32,
    pub data_file_path: *const c_char,
    pub cover_file_path: *const c_char,
    pub __bindgen_padding_0: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveUpdateRequest {
    pub uuid: *const c_char,
    pub name: *const c_char,
    pub summary: *const c_char,
    pub extra: *const c_char,
    pub playtime: u32,
    pub data_file_path: *const c_char,
    pub cover_file_path: *const c_char,
    pub __bindgen_padding_0: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveGetFileRequest {
    pub uuid: *const c_char,
    pub file_id: *const c_char,
}

// Callback type
pub type TapCallback = Option<unsafe extern "C" fn(event_id: u32, data: *mut c_void)>;

// Stub functions that panic on non-Windows
#[inline(always)]
fn unsupported() -> ! {
    panic!("TapTap PC SDK is only supported on Windows. This platform (macOS/Linux) is not supported.")
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_RestartAppIfNecessary(_client_id: *const c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_Init(_err_msg: *mut c_char, _pub_key: *const c_char) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_Shutdown() {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_RunCallbacks() {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_GetClientID(_buffer: *mut c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_RegisterCallback(_event_id: u32, _cb: TapCallback) {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_UnregisterCallback(_event_id: u32, _cb: TapCallback) {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapUser_AsyncAuthorize(_scopes: *const c_char) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapUser_GetOpenID(_buffer: *mut c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapApps_IsOwned() -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapDLC_IsOwned(_dlc_id: *const c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapDLC_ShowStore(_dlc_id: *const c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave() -> *mut ITapCloudSave {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncList(_handle: *mut ITapCloudSave, _request_id: i64) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncCreate(
    _handle: *mut ITapCloudSave,
    _request_id: i64,
    _request: *const TapCloudSaveCreateRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncUpdate(
    _handle: *mut ITapCloudSave,
    _request_id: i64,
    _request: *const TapCloudSaveUpdateRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncDelete(
    _handle: *mut ITapCloudSave,
    _request_id: i64,
    _uuid: *const c_char,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncGetData(
    _handle: *mut ITapCloudSave,
    _request_id: i64,
    _request: *const TapCloudSaveGetFileRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncGetCover(
    _handle: *mut ITapCloudSave,
    _request_id: i64,
    _request: *const TapCloudSaveGetFileRequest,
) -> u32 {
    unsupported()
}
"#;

    std::fs::write(out_path.join("bindings.rs"), stub_bindings)
        .expect("Failed to write stub bindings");
}
