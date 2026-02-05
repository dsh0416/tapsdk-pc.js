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
    println!(
        "cargo:rerun-if-changed={}/taptap_api.h",
        sdk_dir.display()
    );
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

use std::os::raw::{c_char, c_int, c_void};

// Stub types
pub type TapCloudSaveHandle = *mut c_void;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TapSDK_Error {
    pub code: i64,
    pub message: [c_char; 256],
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AuthToken {
    pub token_type: [c_char; 64],
    pub kid: [c_char; 256],
    pub mac_key: [c_char; 256],
    pub mac_algorithm: [c_char; 64],
    pub scope: [c_char; 256],
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SystemStateChangedData {
    pub state: u32,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AuthorizeFinishedResponse {
    pub is_cancel: bool,
    pub error: *const c_char,
    pub token: *const AuthToken,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct GamePlayableStatusChangedData {
    pub is_playable: bool,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct DlcPlayableStatusChangedData {
    pub dlc_id: [c_char; 256],
    pub is_playable: bool,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TapCloudSaveInfo {
    pub uuid: [c_char; 64],
    pub file_id: [c_char; 64],
    pub name: [c_char; 64],
    pub save_size: u64,
    pub cover_size: u64,
    pub summary: [c_char; 512],
    pub extra: [c_char; 1024],
    pub playtime: u64,
    pub created_time: i64,
    pub modified_time: i64,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CloudSaveListData {
    pub request_id: u64,
    pub error: *const TapSDK_Error,
    pub saves: *const TapCloudSaveInfo,
    pub count: u64,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CloudSaveCreateData {
    pub request_id: u64,
    pub error: *const TapSDK_Error,
    pub save: *const TapCloudSaveInfo,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CloudSaveDeleteData {
    pub request_id: u64,
    pub error: *const TapSDK_Error,
    pub uuid: [c_char; 64],
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct CloudSaveGetFileData {
    pub request_id: u64,
    pub error: *const TapSDK_Error,
    pub data: *const u8,
    pub size: u64,
}

// Callback type definitions
pub type TapSDK_SystemStateChangedCallback = Option<extern "C" fn(*const SystemStateChangedData)>;
pub type TapSDK_AuthorizeFinishedCallback = Option<extern "C" fn(*const AuthorizeFinishedResponse)>;
pub type TapSDK_GamePlayableStatusChangedCallback = Option<extern "C" fn(*const GamePlayableStatusChangedData)>;
pub type TapSDK_DlcPlayableStatusChangedCallback = Option<extern "C" fn(*const DlcPlayableStatusChangedData)>;
pub type TapCloudSave_ListCallback = Option<extern "C" fn(*const CloudSaveListData)>;
pub type TapCloudSave_CreateCallback = Option<extern "C" fn(*const CloudSaveCreateData)>;
pub type TapCloudSave_DeleteCallback = Option<extern "C" fn(*const CloudSaveDeleteData)>;
pub type TapCloudSave_GetFileCallback = Option<extern "C" fn(*const CloudSaveGetFileData)>;

// Stub functions that panic on non-Windows
#[inline(always)]
fn unsupported() -> ! {
    panic!("TapTap PC SDK is only supported on Windows. This platform (macOS/Linux) is not supported.")
}

#[no_mangle]
pub extern "C" fn TapSDK_RestartAppIfNecessary(_client_id: *const c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapSDK_Init(_err_msg: *mut c_char, _pub_key: *const c_char) -> u32 {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapSDK_Shutdown() {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapSDK_RunCallbacks() {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapSDK_GetClientID(_buffer: *mut c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapSDK_IsInitialized() -> bool {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapSDK_SetSystemStateChangedCallback(_cb: TapSDK_SystemStateChangedCallback) {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapUser_Authorize(_scopes: *const c_char) -> u32 {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapUser_GetOpenID(_buffer: *mut c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapUser_SetAuthorizeFinishedCallback(_cb: TapSDK_AuthorizeFinishedCallback) {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapApps_IsOwned() -> bool {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapApps_SetPlayableStatusChangedCallback(_cb: TapSDK_GamePlayableStatusChangedCallback) {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapDLC_IsOwned(_dlc_id: *const c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapDLC_ShowStore(_dlc_id: *const c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapDLC_SetPlayableStatusChangedCallback(_cb: TapSDK_DlcPlayableStatusChangedCallback) {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave() -> TapCloudSaveHandle {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_AsyncList(_handle: TapCloudSaveHandle, _request_id: u64) -> u32 {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_AsyncCreate(
    _handle: TapCloudSaveHandle,
    _request_id: u64,
    _name: *const c_char,
    _summary: *const c_char,
    _extra: *const c_char,
    _playtime: u64,
    _data_file_path: *const c_char,
    _cover_file_path: *const c_char,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_AsyncUpdate(
    _handle: TapCloudSaveHandle,
    _request_id: u64,
    _uuid: *const c_char,
    _name: *const c_char,
    _summary: *const c_char,
    _extra: *const c_char,
    _playtime: u64,
    _data_file_path: *const c_char,
    _cover_file_path: *const c_char,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_AsyncDelete(_handle: TapCloudSaveHandle, _request_id: u64, _uuid: *const c_char) -> u32 {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_AsyncGetData(
    _handle: TapCloudSaveHandle,
    _request_id: u64,
    _uuid: *const c_char,
    _file_id: *const c_char,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_AsyncGetCover(
    _handle: TapCloudSaveHandle,
    _request_id: u64,
    _uuid: *const c_char,
    _file_id: *const c_char,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_SetListCallback(_handle: TapCloudSaveHandle, _cb: TapCloudSave_ListCallback) {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_SetCreateCallback(_handle: TapCloudSaveHandle, _cb: TapCloudSave_CreateCallback) {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_SetUpdateCallback(_handle: TapCloudSaveHandle, _cb: TapCloudSave_CreateCallback) {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_SetDeleteCallback(_handle: TapCloudSaveHandle, _cb: TapCloudSave_DeleteCallback) {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_SetGetDataCallback(_handle: TapCloudSaveHandle, _cb: TapCloudSave_GetFileCallback) {
    unsupported()
}

#[no_mangle]
pub extern "C" fn TapCloudSave_SetGetCoverCallback(_handle: TapCloudSaveHandle, _cb: TapCloudSave_GetFileCallback) {
    unsupported()
}
"#;

    std::fs::write(out_path.join("bindings.rs"), stub_bindings)
        .expect("Failed to write stub bindings");
}
