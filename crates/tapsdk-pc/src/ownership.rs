//! Game ownership functionality

use crate::sdk::is_initialized;

/// Check if the user owns the current game
/// 
/// # Returns
/// `true` if the user owns the game, `false` otherwise
/// 
/// # Note
/// This will return `false` if the SDK is not initialized.
pub fn is_game_owned() -> bool {
    if !is_initialized() {
        return false;
    }
    
    unsafe { tapsdk_pc_sys::TapApps_IsOwned() }
}
