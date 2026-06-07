//! Achievement functionality

use std::ffi::CString;

use crate::error::{Result, SdkResult, TapSdkError};
use crate::sdk::is_initialized;

/// Achievement API handle.
///
/// Get an instance via `Achievement::get()` after initializing the SDK.
pub struct Achievement {
    handle: *mut tapsdk_pc_sys::ITapAchievement,
}

// The SDK exposes achievements through a process-wide singleton.
unsafe impl Send for Achievement {}
unsafe impl Sync for Achievement {}

impl Achievement {
    /// Get the achievement singleton instance.
    pub fn get() -> Option<Self> {
        if !is_initialized() {
            return None;
        }

        let handle = unsafe { tapsdk_pc_sys::TapAchievement() };

        if handle.is_null() {
            None
        } else {
            Some(Achievement { handle })
        }
    }

    /// Unlock an achievement.
    ///
    /// The result will be delivered via the `AchievementUnlock` event when
    /// calling `TapSdk::run_callbacks()`.
    pub fn unlock(&self, request_id: i64, achievement_id: &str) -> Result<()> {
        let achievement_id_c = CString::new(achievement_id)?;
        let request = tapsdk_pc_sys::TapAchievementUnlockRequest {
            achievement_id: achievement_id_c.as_ptr(),
        };

        let result =
            unsafe { tapsdk_pc_sys::TapAchievement_AsyncUnlock(self.handle, request_id, &request) };

        check_sdk_result(result)
    }

    /// Increment progress for a step-based achievement.
    ///
    /// The result will be delivered via the `AchievementIncrement` event when
    /// calling `TapSdk::run_callbacks()`.
    pub fn increment(&self, request_id: i64, achievement_id: &str, steps: u64) -> Result<()> {
        let achievement_id_c = CString::new(achievement_id)?;
        let request = tapsdk_pc_sys::TapAchievementIncrementRequest {
            achievement_id: achievement_id_c.as_ptr(),
            steps,
        };

        let result = unsafe {
            tapsdk_pc_sys::TapAchievement_AsyncIncrement(self.handle, request_id, &request)
        };

        check_sdk_result(result)
    }

    /// Open the TapTap achievements page.
    pub fn show_achievements(&self) -> Result<()> {
        let result = unsafe { tapsdk_pc_sys::TapAchievement_ShowAchievements(self.handle) };

        check_sdk_result(result)
    }
}

fn check_sdk_result(result: u32) -> Result<()> {
    match SdkResult::from(result) {
        SdkResult::Ok => Ok(()),
        result => Err(TapSdkError::SdkRequestFailed(result)),
    }
}
