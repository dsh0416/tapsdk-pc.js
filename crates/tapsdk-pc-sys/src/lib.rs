//! Raw FFI bindings to TapTap PC SDK
//!
//! This crate provides unsafe, low-level bindings to the TapTap PC SDK.
//! For a safe, high-level API, use the `tapsdk-pc` crate instead.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]

// Include the generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Re-export commonly used constants for convenience

/// SDK initialization result codes
pub mod init_result {
    /// Initialization successful
    pub const OK: u32 = 0;
    /// Generic failure
    pub const FAILED_GENERIC: u32 = 1;
    /// TapTap platform not found
    pub const NO_PLATFORM: u32 = 2;
    /// Not launched by TapTap platform
    pub const NOT_LAUNCHED_BY_PLATFORM: u32 = 3;
    /// Platform version mismatch
    pub const PLATFORM_VERSION_MISMATCH: u32 = 4;
}

/// Authorization result codes
pub mod authorize_result {
    /// Unknown error
    pub const UNKNOWN: u32 = 0;
    /// Authorization flow started successfully
    pub const OK: u32 = 1;
    /// Failed to start authorization
    pub const FAILED: u32 = 2;
    /// Authorization already in progress
    pub const IN_FLIGHT: u32 = 3;
}

/// Event ID constants
pub mod event_id {
    /// Unknown event
    pub const UNKNOWN: u32 = 0;
    /// System state changed
    pub const SYSTEM_STATE_CHANGED: u32 = 1;
    /// Authorization finished
    pub const AUTHORIZE_FINISHED: u32 = 2002;
    /// Game playable status changed
    pub const GAME_PLAYABLE_STATUS_CHANGED: u32 = 4001;
    /// DLC playable status changed
    pub const DLC_PLAYABLE_STATUS_CHANGED: u32 = 4002;
    /// Cloud save list response
    pub const CLOUD_SAVE_LIST: u32 = 6001;
    /// Cloud save create response
    pub const CLOUD_SAVE_CREATE: u32 = 6002;
    /// Cloud save update response
    pub const CLOUD_SAVE_UPDATE: u32 = 6003;
    /// Cloud save delete response
    pub const CLOUD_SAVE_DELETE: u32 = 6004;
    /// Cloud save get data response
    pub const CLOUD_SAVE_GET_DATA: u32 = 6005;
    /// Cloud save get cover response
    pub const CLOUD_SAVE_GET_COVER: u32 = 6006;
}

/// System state constants
pub mod system_state {
    /// Unknown state
    pub const UNKNOWN: u32 = 0;
    /// Platform is online
    pub const PLATFORM_ONLINE: u32 = 1;
    /// Platform is offline
    pub const PLATFORM_OFFLINE: u32 = 2;
    /// Platform is shutting down
    pub const PLATFORM_SHUTDOWN: u32 = 3;
}

/// Error codes
pub mod error_code {
    /// Success
    pub const SUCCESS: i64 = 0;
    /// Unknown error
    pub const UNKNOWN: i64 = 1;
    /// Unauthorized
    pub const UNAUTHORIZED: i64 = 2;
    /// Method not allowed
    pub const METHOD_NOT_ALLOWED: i64 = 3;
    /// Unimplemented
    pub const UNIMPLEMENTED: i64 = 4;
    /// Invalid arguments
    pub const INVALID_ARGUMENTS: i64 = 5;
    /// Forbidden
    pub const FORBIDDEN: i64 = 6;
    /// User is deactivated
    pub const USER_IS_DEACTIVATED: i64 = 7;
    /// Internal server error
    pub const INTERNAL_SERVER_ERROR: i64 = 8;
    /// Internal SDK error
    pub const INTERNAL_SDK_ERROR: i64 = 9;
    /// Network error
    pub const NETWORK_ERROR: i64 = 10;

    // Cloud save specific errors (400000-499999)
    /// Invalid file size
    pub const CLOUD_SAVE_INVALID_FILE_SIZE: i64 = 400000;
    /// Upload rate limit exceeded
    pub const CLOUD_SAVE_UPLOAD_RATE_LIMIT: i64 = 400001;
    /// File not found
    pub const CLOUD_SAVE_FILE_NOT_FOUND: i64 = 400002;
    /// File count limit per client exceeded
    pub const CLOUD_SAVE_FILE_COUNT_LIMIT_PER_CLIENT: i64 = 400003;
    /// Storage size limit per client exceeded
    pub const CLOUD_SAVE_STORAGE_SIZE_LIMIT_PER_CLIENT: i64 = 400004;
    /// Total storage size limit exceeded
    pub const CLOUD_SAVE_TOTAL_STORAGE_SIZE_LIMIT: i64 = 400005;
    /// Timeout
    pub const CLOUD_SAVE_TIMEOUT: i64 = 400006;
    /// Concurrent call disallowed
    pub const CLOUD_SAVE_CONCURRENT_CALL_DISALLOWED: i64 = 400007;
    /// Storage server error
    pub const CLOUD_SAVE_STORAGE_SERVER_ERROR: i64 = 400008;
    /// Invalid name
    pub const CLOUD_SAVE_INVALID_NAME: i64 = 400009;
}

/// Cloud save result codes
pub mod cloudsave_result {
    /// Request initiated successfully
    pub const OK: u32 = 0;
    /// SDK not initialized
    pub const UNINITIALIZED: u32 = 1;
    /// TapTap client not running
    pub const NO_TAPTAP_CLIENT: u32 = 2;
    /// TapTap client outdated
    pub const TAPTAP_CLIENT_OUTDATED: u32 = 3;
    /// Invalid argument
    pub const INVALID_ARGUMENT: u32 = 4;
    /// SDK internal failure
    pub const SDK_FAILED: u32 = 5;
    /// Failed to read save file
    pub const FAILED_TO_READ_SAVE_FILE: u32 = 6;
    /// Save file too large (>10MB)
    pub const SAVE_FILE_TOO_LARGE: u32 = 7;
    /// Failed to read cover file
    pub const FAILED_TO_READ_COVER_FILE: u32 = 8;
    /// Cover file too large (>512KB)
    pub const COVER_FILE_TOO_LARGE: u32 = 9;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_constants() {
        // Verify our constant definitions match expected values
        assert_eq!(init_result::OK, 0);
        assert_eq!(init_result::NO_PLATFORM, 2);
        assert_eq!(event_id::AUTHORIZE_FINISHED, 2002);
        assert_eq!(error_code::SUCCESS, 0);
        assert_eq!(cloudsave_result::OK, 0);
    }

    #[test]
    fn test_dll_loads_and_functions_exist() {
        // This test verifies the DLL is properly linked and functions are callable
        // The functions should be resolvable even if they fail at runtime
        
        // Test TapSDK_RestartAppIfNecessary - should return false since we're not in TapTap
        let client_id = CString::new("test_client_id").unwrap();
        let result = unsafe { TapSDK_RestartAppIfNecessary(client_id.as_ptr()) };
        // Should return false since we're not launched from TapTap
        assert!(!result, "RestartAppIfNecessary should return false when not in TapTap");
    }

    #[test]
    fn test_sdk_init_without_taptap() {
        // Test that SDK init fails gracefully without TapTap client
        let pub_key = CString::new("test_public_key").unwrap();
        let mut err_msg: [std::os::raw::c_char; 1024] = [0; 1024];

        let result = unsafe { TapSDK_Init(err_msg.as_mut_ptr() as *mut _, pub_key.as_ptr()) };

        // Without TapTap client, should return NoPlatform (2) or NotLaunchedByPlatform (3)
        assert!(
            result == init_result::NO_PLATFORM || result == init_result::NOT_LAUNCHED_BY_PLATFORM,
            "SDK init should fail with NoPlatform or NotLaunchedByPlatform, got: {}",
            result
        );
    }

    #[test]
    fn test_ownership_check_without_init() {
        // Ownership check should safely return false when SDK not initialized
        let owned = unsafe { TapApps_IsOwned() };
        assert!(!owned, "IsOwned should return false when SDK not initialized");
    }

    #[test]
    fn test_get_client_id_without_init() {
        // GetClientID should safely return false when SDK not initialized
        let mut buffer: [std::os::raw::c_char; 256] = [0; 256];
        let result = unsafe { TapSDK_GetClientID(buffer.as_mut_ptr()) };
        assert!(!result, "GetClientID should return false when SDK not initialized");
    }

    #[test]
    fn test_get_open_id_without_init() {
        // GetOpenID should safely return false when SDK not initialized
        let mut buffer: [std::os::raw::c_char; 256] = [0; 256];
        let result = unsafe { TapUser_GetOpenID(buffer.as_mut_ptr()) };
        assert!(!result, "GetOpenID should return false when SDK not initialized");
    }

    #[test]
    fn test_dlc_check_without_init() {
        // DLC ownership check should safely return false when SDK not initialized
        let dlc_id = CString::new("test_dlc").unwrap();
        let owned = unsafe { TapDLC_IsOwned(dlc_id.as_ptr()) };
        assert!(!owned, "IsDlcOwned should return false when SDK not initialized");
    }

    #[test]
    fn test_run_callbacks_without_init() {
        // RunCallbacks should be safe to call even without initialization
        // This just verifies it doesn't crash
        unsafe { TapSDK_RunCallbacks() };
    }

    #[test]
    fn test_struct_sizes() {
        // Verify struct sizes are reasonable (helps catch alignment issues)
        assert!(std::mem::size_of::<TapSDK_Error>() >= 16, "TapSDK_Error should be at least 16 bytes");
        assert!(std::mem::size_of::<AuthorizeFinishedResponse>() > 0, "AuthorizeFinishedResponse should have size");
        assert!(std::mem::size_of::<TapCloudSaveInfo>() > 0, "TapCloudSaveInfo should have size");
    }

    #[test]
    fn test_cloudsave_without_init() {
        // CloudSave functions should return appropriate error when SDK not initialized
        let handle = unsafe { TapCloudSave() };
        
        if !handle.is_null() {
            let result = unsafe { TapCloudSave_AsyncList(handle, 1) };
            // Should return Uninitialized or SdkFailed
            assert!(
                result == cloudsave_result::UNINITIALIZED || 
                result == cloudsave_result::SDK_FAILED ||
                result == cloudsave_result::NO_TAPTAP_CLIENT,
                "CloudSave list should fail when not initialized, got: {}",
                result
            );
        }
    }
}
