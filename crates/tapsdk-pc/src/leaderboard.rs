//! Leaderboard functionality

use std::ffi::CString;

use crate::error::{Result, SdkResult, TapSdkError};
use crate::sdk::is_initialized;

/// Leaderboard collection type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeaderboardCollection {
    Public,
    Friends,
}

impl From<LeaderboardCollection> for u32 {
    fn from(collection: LeaderboardCollection) -> Self {
        match collection {
            LeaderboardCollection::Public => {
                tapsdk_pc_sys::TapLeaderboardCollection_Public.0 as u32
            }
            LeaderboardCollection::Friends => {
                tapsdk_pc_sys::TapLeaderboardCollection_Friends.0 as u32
            }
        }
    }
}

/// Leaderboard API handle.
///
/// Get an instance via `Leaderboard::get()` after initializing the SDK.
pub struct Leaderboard {
    handle: *mut tapsdk_pc_sys::ITapLeaderboard,
}

// The SDK exposes leaderboards through a process-wide singleton.
unsafe impl Send for Leaderboard {}
unsafe impl Sync for Leaderboard {}

impl Leaderboard {
    /// Get the leaderboard singleton instance.
    pub fn get() -> Option<Self> {
        if !is_initialized() {
            return None;
        }

        let handle = unsafe { tapsdk_pc_sys::TapLeaderboard() };

        if handle.is_null() {
            None
        } else {
            Some(Leaderboard { handle })
        }
    }

    /// Submit scores to up to five leaderboards.
    ///
    /// The result will be delivered via the `LeaderboardSubmitScores` event.
    pub fn submit_scores(&self, request_id: i64, items: &[LeaderboardScoreItem]) -> Result<()> {
        let leaderboard_ids = items
            .iter()
            .map(|item| CString::new(item.leaderboard_id.as_str()))
            .collect::<std::result::Result<Vec<_>, _>>()?;

        let raw_items = items
            .iter()
            .zip(leaderboard_ids.iter())
            .map(
                |(item, leaderboard_id)| tapsdk_pc_sys::TapLeaderboardScoreItem {
                    leaderboard_id: leaderboard_id.as_ptr(),
                    score: item.score,
                },
            )
            .collect::<Vec<_>>();

        let mut request: tapsdk_pc_sys::TapLeaderboardSubmitScoresRequest =
            unsafe { std::mem::zeroed() };
        request.item_count = raw_items.len() as u32;
        request.items = raw_items.as_ptr();

        let result = unsafe {
            tapsdk_pc_sys::TapLeaderboard_AsyncSubmitScores(self.handle, request_id, &request)
        };

        check_sdk_result(result)
    }

    /// Load leaderboard scores.
    ///
    /// The result will be delivered via the `LeaderboardLoadScores` event.
    pub fn load_scores(&self, request_id: i64, request: &LoadScoresRequest) -> Result<()> {
        let leaderboard_id = CString::new(request.leaderboard_id.as_str())?;
        let continuation_token = request
            .continuation_token
            .as_ref()
            .map(|token| CString::new(token.as_str()))
            .transpose()?;
        let period_token = request
            .period_token
            .as_ref()
            .map(|token| CString::new(token.as_str()))
            .transpose()?;

        let mut raw_request: tapsdk_pc_sys::TapLeaderboardLoadScoresRequest =
            unsafe { std::mem::zeroed() };
        raw_request.leaderboard_id = leaderboard_id.as_ptr();
        raw_request.collection = request.collection.into();
        raw_request.continuation_token = continuation_token
            .as_ref()
            .map(|token| token.as_ptr())
            .unwrap_or(std::ptr::null());
        raw_request.period_token = period_token
            .as_ref()
            .map(|token| token.as_ptr())
            .unwrap_or(std::ptr::null());

        let result = unsafe {
            tapsdk_pc_sys::TapLeaderboard_AsyncLoadScores(self.handle, request_id, &raw_request)
        };

        check_sdk_result(result)
    }

    /// Load the current user's score.
    ///
    /// The result will be delivered via the `LeaderboardLoadMyScores` event.
    pub fn load_my_scores(&self, request_id: i64, request: &LoadMyScoresRequest) -> Result<()> {
        let leaderboard_id = CString::new(request.leaderboard_id.as_str())?;
        let period_token = request
            .period_token
            .as_ref()
            .map(|token| CString::new(token.as_str()))
            .transpose()?;

        let mut raw_request: tapsdk_pc_sys::TapLeaderboardLoadMyScoresRequest =
            unsafe { std::mem::zeroed() };
        raw_request.leaderboard_id = leaderboard_id.as_ptr();
        raw_request.collection = request.collection.into();
        raw_request.period_token = period_token
            .as_ref()
            .map(|token| token.as_ptr())
            .unwrap_or(std::ptr::null());

        let result = unsafe {
            tapsdk_pc_sys::TapLeaderboard_AsyncLoadMyScores(self.handle, request_id, &raw_request)
        };

        check_sdk_result(result)
    }

    /// Load scores near the current user.
    ///
    /// The result will be delivered via the `LeaderboardLoadMyCenteredScores` event.
    pub fn load_my_centered_scores(
        &self,
        request_id: i64,
        request: &LoadMyCenteredScoresRequest,
    ) -> Result<()> {
        let leaderboard_id = CString::new(request.leaderboard_id.as_str())?;

        let raw_request = tapsdk_pc_sys::TapLeaderboardLoadMyCenteredScoresRequest {
            leaderboard_id: leaderboard_id.as_ptr(),
            collection: request.collection.into(),
            max_results: request.max_results,
        };

        let result = unsafe {
            tapsdk_pc_sys::TapLeaderboard_AsyncLoadMyCenteredScores(
                self.handle,
                request_id,
                &raw_request,
            )
        };

        check_sdk_result(result)
    }

    /// Open the TapTap leaderboards page.
    pub fn show_leaderboards(&self, request: &ShowLeaderboardRequest) -> Result<()> {
        let leaderboard_id = CString::new(request.leaderboard_id.as_str())?;

        let mut raw_request: tapsdk_pc_sys::TapLeaderboardShowRequest =
            unsafe { std::mem::zeroed() };
        raw_request.leaderboard_id = leaderboard_id.as_ptr();
        raw_request.collection = request.collection.into();

        let result =
            unsafe { tapsdk_pc_sys::TapLeaderboard_ShowLeaderboards(self.handle, &raw_request) };

        check_sdk_result(result)
    }
}

/// Score item submitted to a leaderboard.
#[derive(Debug, Clone)]
pub struct LeaderboardScoreItem {
    pub leaderboard_id: String,
    pub score: i64,
}

/// Request parameters for loading leaderboard scores.
#[derive(Debug, Clone)]
pub struct LoadScoresRequest {
    pub leaderboard_id: String,
    pub collection: LeaderboardCollection,
    pub continuation_token: Option<String>,
    pub period_token: Option<String>,
}

/// Request parameters for loading the current user's score.
#[derive(Debug, Clone)]
pub struct LoadMyScoresRequest {
    pub leaderboard_id: String,
    pub collection: LeaderboardCollection,
    pub period_token: Option<String>,
}

/// Request parameters for loading scores near the current user.
#[derive(Debug, Clone)]
pub struct LoadMyCenteredScoresRequest {
    pub leaderboard_id: String,
    pub collection: LeaderboardCollection,
    pub max_results: u32,
}

/// Request parameters for opening the leaderboards page.
#[derive(Debug, Clone)]
pub struct ShowLeaderboardRequest {
    pub leaderboard_id: String,
    pub collection: LeaderboardCollection,
}

fn check_sdk_result(result: u32) -> Result<()> {
    match SdkResult::from(result) {
        SdkResult::Ok => Ok(()),
        result => Err(TapSdkError::SdkRequestFailed(result)),
    }
}
