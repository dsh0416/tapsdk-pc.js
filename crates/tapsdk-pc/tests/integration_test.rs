//! Integration tests for tapsdk-pc
//!
//! These tests verify the high-level Rust API works correctly.
//! Note: Full SDK functionality requires the TapTap client to be running.

use tapsdk_pc::error::{InitResult, TapSdkError};
use tapsdk_pc::{dlc, ownership, user, TapSdk};

#[test]
fn test_sdk_not_initialized_initially() {
    // SDK should not be initialized at start
    assert!(
        !tapsdk_pc::is_initialized(),
        "SDK should not be initialized initially"
    );
}

#[test]
fn test_restart_app_if_necessary() {
    // This should return false when not running from TapTap
    let result = tapsdk_pc::restart_app_if_necessary("test_client_id");
    assert!(result.is_ok(), "restart_app_if_necessary should not error");
    assert!(
        !result.unwrap(),
        "Should return false when not running from TapTap"
    );
}

#[test]
fn test_sdk_init_fails_without_taptap() {
    // SDK initialization should fail gracefully without TapTap client
    let result = TapSdk::init("test_public_key");

    assert!(result.is_err(), "SDK init should fail without TapTap");

    if let Err(TapSdkError::InitFailed { result, message: _ }) = result {
        assert!(
            result == InitResult::NoPlatform || result == InitResult::NotLaunchedByPlatform,
            "Should fail with NoPlatform or NotLaunchedByPlatform, got: {:?}",
            result
        );
    } else {
        panic!("Expected InitFailed error, got: {:?}", result);
    }
}

#[test]
fn test_ownership_without_init() {
    // Ownership check should return false when SDK not initialized
    assert!(
        !ownership::is_game_owned(),
        "is_game_owned should return false when not initialized"
    );
}

#[test]
fn test_dlc_ownership_without_init() {
    // DLC ownership check should return false when SDK not initialized
    assert!(
        !dlc::is_dlc_owned("test_dlc"),
        "is_dlc_owned should return false when not initialized"
    );
}

#[test]
fn test_dlc_store_without_init() {
    // DLC store should return error or false when SDK not initialized
    let result = dlc::show_dlc_store("test_dlc");
    match result {
        Err(_) => (),    // Expected - SDK not initialized
        Ok(false) => (), // Also acceptable - operation failed
        Ok(true) => panic!("show_dlc_store should not succeed when not initialized"),
    }
}

#[test]
fn test_user_authorize_without_init() {
    // Authorize should fail when SDK not initialized
    let result = user::authorize("public_profile");
    assert!(
        result.is_err(),
        "authorize should fail when not initialized"
    );
}

#[test]
fn test_user_open_id_without_init() {
    // OpenID should be None when SDK not initialized
    assert!(
        user::get_open_id().is_none(),
        "get_open_id should return None when not initialized"
    );
}

#[test]
fn test_cloudsave_without_init() {
    // CloudSave::get() should return None when SDK not initialized
    let cloudsave = tapsdk_pc::CloudSave::get();
    assert!(
        cloudsave.is_none(),
        "CloudSave::get() should return None when not initialized"
    );
}

#[test]
fn test_error_types() {
    // Verify error type conversions work correctly
    use tapsdk_pc::error::{AuthorizeResult, CloudSaveResult, SystemState};

    // Test InitResult conversions
    assert_eq!(InitResult::from(0), InitResult::Ok);
    assert_eq!(InitResult::from(2), InitResult::NoPlatform);
    assert_eq!(InitResult::from(99), InitResult::Unknown(99));

    // Test AuthorizeResult conversions
    assert_eq!(AuthorizeResult::from(0), AuthorizeResult::Unknown);
    assert_eq!(AuthorizeResult::from(1), AuthorizeResult::Ok);
    assert_eq!(AuthorizeResult::from(3), AuthorizeResult::InFlight);

    // Test CloudSaveResult conversions
    assert_eq!(CloudSaveResult::from(0), CloudSaveResult::Ok);
    assert_eq!(CloudSaveResult::from(1), CloudSaveResult::Uninitialized);
    assert_eq!(CloudSaveResult::from(7), CloudSaveResult::SaveFileTooLarge);

    // Test SystemState conversions
    assert_eq!(SystemState::from(0), SystemState::Unknown);
    assert_eq!(SystemState::from(1), SystemState::PlatformOnline);
    assert_eq!(SystemState::from(3), SystemState::PlatformShutdown);
}

#[test]
fn test_callback_event_ids() {
    // Verify event ID constants are correct
    use tapsdk_pc::callback::event_id;

    assert_eq!(event_id::SYSTEM_STATE_CHANGED, 1);
    assert_eq!(event_id::AUTHORIZE_FINISHED, 2002);
    assert_eq!(event_id::GAME_PLAYABLE_STATUS_CHANGED, 4001);
    assert_eq!(event_id::CLOUD_SAVE_LIST, 6001);
}
