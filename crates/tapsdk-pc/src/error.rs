//! Error types for TapTap PC SDK

use thiserror::Error;

/// Result type alias for TapSDK operations
pub type Result<T> = std::result::Result<T, TapSdkError>;

/// SDK initialization result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitResult {
    /// Initialization successful
    Ok,
    /// Generic failure
    FailedGeneric,
    /// TapTap platform not found
    NoPlatform,
    /// Not launched by TapTap platform
    NotLaunchedByPlatform,
    /// Platform version mismatch
    PlatformVersionMismatch,
    /// Unknown result code
    Unknown(u32),
}

impl From<u32> for InitResult {
    fn from(code: u32) -> Self {
        match code {
            0 => InitResult::Ok,
            1 => InitResult::FailedGeneric,
            2 => InitResult::NoPlatform,
            3 => InitResult::NotLaunchedByPlatform,
            4 => InitResult::PlatformVersionMismatch,
            _ => InitResult::Unknown(code),
        }
    }
}

/// Authorization request result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizeResult {
    /// Unknown error
    Unknown,
    /// Authorization flow started successfully
    Ok,
    /// Failed to start authorization
    Failed,
    /// Authorization already in progress
    InFlight,
}

impl From<u32> for AuthorizeResult {
    fn from(code: u32) -> Self {
        match code {
            0 => AuthorizeResult::Unknown,
            1 => AuthorizeResult::Ok,
            2 => AuthorizeResult::Failed,
            3 => AuthorizeResult::InFlight,
            _ => AuthorizeResult::Unknown,
        }
    }
}

/// Cloud save operation result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloudSaveResult {
    /// Request initiated successfully
    Ok,
    /// SDK not initialized
    Uninitialized,
    /// TapTap client not running
    NoTapTapClient,
    /// TapTap client outdated
    TapTapClientOutdated,
    /// Invalid argument
    InvalidArgument,
    /// SDK internal failure
    SdkFailed,
    /// Failed to read save file
    FailedToReadSaveFile,
    /// Save file too large (>10MB)
    SaveFileTooLarge,
    /// Failed to read cover file
    FailedToReadCoverFile,
    /// Cover file too large (>512KB)
    CoverFileTooLarge,
    /// Unknown result code
    Unknown(u32),
}

impl From<u32> for CloudSaveResult {
    fn from(code: u32) -> Self {
        match code {
            0 => CloudSaveResult::Ok,
            1 => CloudSaveResult::Uninitialized,
            2 => CloudSaveResult::NoTapTapClient,
            3 => CloudSaveResult::TapTapClientOutdated,
            4 => CloudSaveResult::InvalidArgument,
            5 => CloudSaveResult::SdkFailed,
            6 => CloudSaveResult::FailedToReadSaveFile,
            7 => CloudSaveResult::SaveFileTooLarge,
            8 => CloudSaveResult::FailedToReadCoverFile,
            9 => CloudSaveResult::CoverFileTooLarge,
            _ => CloudSaveResult::Unknown(code),
        }
    }
}

/// System state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemState {
    /// Unknown state
    Unknown,
    /// Platform is online
    PlatformOnline,
    /// Platform is offline
    PlatformOffline,
    /// Platform is shutting down
    PlatformShutdown,
}

impl From<u32> for SystemState {
    fn from(code: u32) -> Self {
        match code {
            0 => SystemState::Unknown,
            1 => SystemState::PlatformOnline,
            2 => SystemState::PlatformOffline,
            3 => SystemState::PlatformShutdown,
            _ => SystemState::Unknown,
        }
    }
}

/// Main error type for TapSDK operations
#[derive(Debug, Error)]
pub enum TapSdkError {
    /// SDK initialization failed
    #[error("SDK initialization failed: {result:?} - {message}")]
    InitFailed {
        result: InitResult,
        message: String,
    },

    /// SDK not initialized
    #[error("SDK not initialized")]
    NotInitialized,

    /// Authorization failed
    #[error("Authorization failed: {0:?}")]
    AuthorizeFailed(AuthorizeResult),

    /// Cloud save operation failed to start
    #[error("Cloud save request failed: {0:?}")]
    CloudSaveRequestFailed(CloudSaveResult),

    /// API error returned from the SDK
    #[error("API error ({code}): {message}")]
    ApiError {
        code: i64,
        message: String,
    },

    /// Invalid argument provided
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Null pointer returned
    #[error("Null pointer returned from SDK")]
    NullPointer,

    /// UTF-8 conversion error
    #[error("UTF-8 conversion error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    /// String contains null byte
    #[error("String contains null byte: {0}")]
    NulError(#[from] std::ffi::NulError),
}

impl TapSdkError {
    /// Create an API error from SDK error code and message
    pub fn from_api_error(code: i64, message: impl Into<String>) -> Self {
        TapSdkError::ApiError {
            code,
            message: message.into(),
        }
    }

    /// Create an API error from raw SDK error pointer
    /// 
    /// # Safety
    /// The error pointer must be valid and point to a valid TapSDK_Error struct
    pub unsafe fn from_raw_error(error: *const tapsdk_pc_sys::TapSDK_Error) -> Option<Self> {
        if error.is_null() {
            return None;
        }
        
        let err = &*error;
        let message = if err.message.is_null() {
            String::new()
        } else {
            std::ffi::CStr::from_ptr(err.message)
                .to_string_lossy()
                .into_owned()
        };
        
        Some(TapSdkError::ApiError {
            code: err.code,
            message,
        })
    }
}

/// Error code constants matching the C SDK
pub mod error_code {
    pub const SUCCESS: i64 = 0;
    pub const UNKNOWN: i64 = 1;
    pub const UNAUTHORIZED: i64 = 2;
    pub const METHOD_NOT_ALLOWED: i64 = 3;
    pub const UNIMPLEMENTED: i64 = 4;
    pub const INVALID_ARGUMENTS: i64 = 5;
    pub const FORBIDDEN: i64 = 6;
    pub const USER_IS_DEACTIVATED: i64 = 7;
    pub const INTERNAL_SERVER_ERROR: i64 = 8;
    pub const INTERNAL_SDK_ERROR: i64 = 9;
    pub const NETWORK_ERROR: i64 = 10;

    // Cloud save specific errors (400000-499999)
    pub const CLOUD_SAVE_INVALID_FILE_SIZE: i64 = 400000;
    pub const CLOUD_SAVE_UPLOAD_RATE_LIMIT: i64 = 400001;
    pub const CLOUD_SAVE_FILE_NOT_FOUND: i64 = 400002;
    pub const CLOUD_SAVE_FILE_COUNT_LIMIT_PER_CLIENT: i64 = 400003;
    pub const CLOUD_SAVE_STORAGE_SIZE_LIMIT_PER_CLIENT: i64 = 400004;
    pub const CLOUD_SAVE_TOTAL_STORAGE_SIZE_LIMIT: i64 = 400005;
    pub const CLOUD_SAVE_TIMEOUT: i64 = 400006;
    pub const CLOUD_SAVE_CONCURRENT_CALL_DISALLOWED: i64 = 400007;
    pub const CLOUD_SAVE_STORAGE_SERVER_ERROR: i64 = 400008;
    pub const CLOUD_SAVE_INVALID_NAME: i64 = 400009;
}
