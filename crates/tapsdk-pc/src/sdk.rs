//! Core SDK functionality

use std::ffi::{CStr, CString};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::callback::{self, TapEvent};
use crate::error::{InitResult, Result, TapSdkError};

/// Global flag to track if SDK is initialized
static SDK_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Check if the SDK has been initialized
pub fn is_initialized() -> bool {
    SDK_INITIALIZED.load(Ordering::SeqCst)
}

/// Check if the app needs to restart (should be called before init)
/// 
/// This function should be called before `TapSdk::init()` to check if the game
/// was launched directly instead of through TapTap. If it returns `true`, 
/// TapTap will relaunch the game and you should exit immediately.
/// 
/// # Arguments
/// * `client_id` - The client ID from TapTap developer center
/// 
/// # Returns
/// `true` if the app needs to restart (exit immediately), `false` otherwise
pub fn restart_app_if_necessary(client_id: &str) -> Result<bool> {
    let client_id_c = CString::new(client_id)?;
    let result = unsafe { tapsdk_pc_sys::TapSDK_RestartAppIfNecessary(client_id_c.as_ptr()) };
    Ok(result)
}

/// Main TapTap PC SDK wrapper
/// 
/// This struct represents an initialized SDK instance. Only one instance
/// can exist at a time. When dropped, it will shut down the SDK.
#[derive(Debug)]
pub struct TapSdk {
    _private: (), // Prevent direct construction
}

impl TapSdk {
    /// Initialize the TapTap PC SDK
    /// 
    /// # Arguments
    /// * `pub_key` - The public key from TapTap developer center
    /// 
    /// # Returns
    /// A `TapSdk` instance on success, or an error if initialization failed
    /// 
    /// # Example
    /// ```no_run
    /// use tapsdk_pc::TapSdk;
    /// 
    /// let sdk = TapSdk::init("your_public_key_here").expect("Failed to init SDK");
    /// ```
    pub fn init(pub_key: &str) -> Result<Self> {
        if SDK_INITIALIZED.swap(true, Ordering::SeqCst) {
            return Err(TapSdkError::InvalidArgument(
                "SDK already initialized".to_string(),
            ));
        }

        let pub_key_c = CString::new(pub_key)?;
        let mut err_msg: [std::os::raw::c_char; 1024] = [0; 1024];

        let result = unsafe {
            tapsdk_pc_sys::TapSDK_Init(err_msg.as_mut_ptr() as *mut _, pub_key_c.as_ptr())
        };

        let init_result = InitResult::from(result);

        if init_result != InitResult::Ok {
            SDK_INITIALIZED.store(false, Ordering::SeqCst);
            
            let error_message = unsafe {
                CStr::from_ptr(err_msg.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            };
            
            return Err(TapSdkError::InitFailed {
                result: init_result,
                message: error_message,
            });
        }

        // Register our callback handlers
        callback::register_callbacks();

        Ok(TapSdk { _private: () })
    }

    /// Get the client ID
    /// 
    /// # Returns
    /// The client ID string, or `None` if not available
    pub fn get_client_id(&self) -> Option<String> {
        let mut buffer: [std::os::raw::c_char; 256] = [0; 256];
        
        let success = unsafe {
            tapsdk_pc_sys::TapSDK_GetClientID(buffer.as_mut_ptr())
        };
        
        if success {
            let client_id = unsafe {
                CStr::from_ptr(buffer.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            };
            if client_id.is_empty() {
                None
            } else {
                Some(client_id)
            }
        } else {
            None
        }
    }

    /// Poll for events from the SDK
    /// 
    /// This should be called regularly (e.g., in your game loop) to process
    /// pending callbacks and receive events.
    /// 
    /// # Returns
    /// A vector of events that have occurred since the last poll
    pub fn run_callbacks(&self) -> Vec<TapEvent> {
        callback::poll_events()
    }

    /// Shut down the SDK
    /// 
    /// This is called automatically when the `TapSdk` instance is dropped,
    /// but can be called explicitly if needed.
    pub fn shutdown(self) {
        // The Drop implementation will handle cleanup
        drop(self);
    }
}

impl Drop for TapSdk {
    fn drop(&mut self) {
        // Unregister callbacks first
        callback::unregister_callbacks();
        
        // Shut down the SDK
        unsafe {
            tapsdk_pc_sys::TapSDK_Shutdown();
        }
        
        // Mark SDK as not initialized
        SDK_INITIALIZED.store(false, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_initialized() {
        assert!(!is_initialized());
    }
}
