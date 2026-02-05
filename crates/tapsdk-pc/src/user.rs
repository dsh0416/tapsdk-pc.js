//! User authentication functionality

use std::ffi::{CStr, CString};

use crate::error::{AuthorizeResult, Result, TapSdkError};
use crate::sdk::is_initialized;

/// Request user authorization
/// 
/// This initiates the authorization flow. The result will be delivered via
/// the `AuthorizeFinished` event when calling `TapSdk::run_callbacks()`.
/// 
/// # Arguments
/// * `scopes` - Permission scopes to request, comma-separated (e.g., "public_profile,user_friends")
/// 
/// # Returns
/// * `Ok(())` - Authorization flow started successfully
/// * `Err` - Failed to start authorization (check the error for details)
/// 
/// # Example
/// ```no_run
/// use tapsdk_pc::{user, TapSdk};
/// 
/// let sdk = TapSdk::init("your_public_key").expect("Failed to init");
/// user::authorize("public_profile").expect("Failed to authorize");
/// 
/// // Poll for events in your game loop
/// for event in sdk.run_callbacks() {
///     // Handle AuthorizeFinished event
/// }
/// ```
pub fn authorize(scopes: &str) -> Result<()> {
    if !is_initialized() {
        return Err(TapSdkError::NotInitialized);
    }

    let scopes_c = CString::new(scopes)?;
    
    let result = unsafe {
        tapsdk_pc_sys::TapUser_AsyncAuthorize(scopes_c.as_ptr())
    };
    
    let auth_result = AuthorizeResult::from(result);
    
    match auth_result {
        AuthorizeResult::Ok => Ok(()),
        _ => Err(TapSdkError::AuthorizeFailed(auth_result)),
    }
}

/// Get the current user's OpenID
/// 
/// The OpenID is a unique identifier for the user within your game.
/// 
/// # Returns
/// The user's OpenID, or `None` if not authorized or not available
pub fn get_open_id() -> Option<String> {
    if !is_initialized() {
        return None;
    }

    let mut buffer: [std::os::raw::c_char; 256] = [0; 256];
    
    let success = unsafe {
        tapsdk_pc_sys::TapUser_GetOpenID(buffer.as_mut_ptr())
    };
    
    if success {
        let open_id = unsafe {
            CStr::from_ptr(buffer.as_ptr())
                .to_string_lossy()
                .into_owned()
        };
        if open_id.is_empty() {
            None
        } else {
            Some(open_id)
        }
    } else {
        None
    }
}
