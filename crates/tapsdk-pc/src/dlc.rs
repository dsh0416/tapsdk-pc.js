//! DLC (Downloadable Content) functionality

use std::ffi::CString;

use crate::error::{Result, TapSdkError};
use crate::sdk::is_initialized;

/// Check if the user owns a specific DLC
///
/// # Arguments
/// * `dlc_id` - The DLC identifier
///
/// # Returns
/// `true` if the user owns the DLC, `false` otherwise
///
/// # Note
/// This will return `false` if the SDK is not initialized.
pub fn is_dlc_owned(dlc_id: &str) -> bool {
    if !is_initialized() {
        return false;
    }

    let dlc_id_c = match CString::new(dlc_id) {
        Ok(s) => s,
        Err(_) => return false,
    };

    unsafe { tapsdk_pc_sys::TapDLC_IsOwned(dlc_id_c.as_ptr()) }
}

/// Show the store page for a specific DLC
///
/// This opens the TapTap store page for the specified DLC,
/// allowing the user to purchase it.
///
/// # Arguments
/// * `dlc_id` - The DLC identifier
///
/// # Returns
/// * `Ok(true)` - Store page opened successfully
/// * `Ok(false)` - Failed to open store page
/// * `Err` - SDK not initialized or invalid argument
pub fn show_dlc_store(dlc_id: &str) -> Result<bool> {
    if !is_initialized() {
        return Err(TapSdkError::NotInitialized);
    }

    let dlc_id_c = CString::new(dlc_id)?;

    let result = unsafe { tapsdk_pc_sys::TapDLC_ShowStore(dlc_id_c.as_ptr()) };

    Ok(result)
}
