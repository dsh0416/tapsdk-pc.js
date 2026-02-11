//! Node.js bindings for TapTap PC SDK
//!
//! This crate provides NAPI-RS bindings to expose the TapTap PC SDK to Node.js.
//! Events are pushed to JavaScript automatically via a background polling thread.

#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi::threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode};
use napi_derive::napi;
use serde::Serialize;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tapsdk_pc::callback::CloudSaveInfo as RustCloudSaveInfo;
use tapsdk_pc::callback::TapEvent;
use tapsdk_pc::error::SystemState;

#[napi]
pub mod event_id {
    #[napi]
    pub const UNKNOWN: u32 = 0;
    #[napi]
    pub const SYSTEM_STATE_CHANGED: u32 = 1;
    #[napi]
    pub const AUTHORIZE_FINISHED: u32 = 2002;
    #[napi]
    pub const GAME_PLAYABLE_STATUS_CHANGED: u32 = 4001;
    #[napi]
    pub const DLC_PLAYABLE_STATUS_CHANGED: u32 = 4002;
    #[napi]
    pub const CLOUD_SAVE_LIST: u32 = 6001;
    #[napi]
    pub const CLOUD_SAVE_CREATE: u32 = 6002;
    #[napi]
    pub const CLOUD_SAVE_UPDATE: u32 = 6003;
    #[napi]
    pub const CLOUD_SAVE_DELETE: u32 = 6004;
    #[napi]
    pub const CLOUD_SAVE_GET_DATA: u32 = 6005;
    #[napi]
    pub const CLOUD_SAVE_GET_COVER: u32 = 6006;
}

#[napi]
pub mod system_state {
    #[napi]
    pub const UNKNOWN: u32 = 0;
    #[napi]
    pub const PLATFORM_ONLINE: u32 = 1;
    #[napi]
    pub const PLATFORM_OFFLINE: u32 = 2;
    #[napi]
    pub const PLATFORM_SHUTDOWN: u32 = 3;
}

/// Authorization token
#[napi(object)]
#[derive(Serialize)]
pub struct AuthToken {
    pub token_type: String,
    pub kid: String,
    pub mac_key: String,
    pub mac_algorithm: String,
    pub scope: String,
}

/// SDK Error info
#[napi(object)]
#[derive(Serialize)]
pub struct SdkError {
    pub code: i64,
    pub message: String,
}

/// Cloud save information
#[napi(object)]
#[derive(Serialize)]
pub struct CloudSaveInfo {
    pub uuid: String,
    pub file_id: String,
    pub name: String,
    pub save_size: u32,
    pub cover_size: u32,
    pub summary: Option<String>,
    pub extra: Option<String>,
    pub playtime: u32,
    pub created_time: u32,
    pub modified_time: u32,
}

impl From<RustCloudSaveInfo> for CloudSaveInfo {
    fn from(info: RustCloudSaveInfo) -> Self {
        CloudSaveInfo {
            uuid: info.uuid,
            file_id: info.file_id,
            name: info.name,
            save_size: info.save_size,
            cover_size: info.cover_size,
            summary: info.summary,
            extra: info.extra,
            playtime: info.playtime,
            created_time: info.created_time,
            modified_time: info.modified_time,
        }
    }
}

/// Request to create a cloud save
#[napi(object)]
pub struct CreateSaveRequest {
    /// Save name (max 60 bytes, no Chinese characters)
    pub name: String,
    /// Save description (max 500 bytes)
    pub summary: String,
    /// Developer-defined extra data (max 1000 bytes, optional)
    pub extra: Option<String>,
    /// Game playtime in seconds
    pub playtime: u32,
    /// Path to the save data file (max 10MB)
    pub data_file_path: String,
    /// Path to the cover image file (max 512KB, optional)
    pub cover_file_path: Option<String>,
}

/// Request to update a cloud save
#[napi(object)]
pub struct UpdateSaveRequest {
    /// UUID of the cloud save to update
    pub uuid: String,
    /// Save name (max 60 bytes, no Chinese characters)
    pub name: String,
    /// Save description (max 500 bytes)
    pub summary: String,
    /// Developer-defined extra data (max 1000 bytes, optional)
    pub extra: Option<String>,
    /// Game playtime in seconds
    pub playtime: u32,
    /// Path to the save data file (max 10MB)
    pub data_file_path: String,
    /// Path to the cover image file (max 512KB, optional)
    pub cover_file_path: Option<String>,
}

/// System state changed event
#[napi(object)]
#[derive(Serialize)]
pub struct SystemStateChangedEvent {
    pub event_id: u32,
    pub state: u32,
}

/// Authorization finished event
#[napi(object)]
#[derive(Serialize)]
pub struct AuthorizeFinishedEvent {
    pub event_id: u32,
    pub is_cancel: bool,
    pub error: Option<String>,
    pub token: Option<AuthToken>,
}

/// Game playable status changed event
#[napi(object)]
#[derive(Serialize)]
pub struct GamePlayableStatusChangedEvent {
    pub event_id: u32,
    pub is_playable: bool,
}

/// DLC playable status changed event
#[napi(object)]
#[derive(Serialize)]
pub struct DlcPlayableStatusChangedEvent {
    pub event_id: u32,
    pub dlc_id: String,
    pub is_playable: bool,
}

/// Cloud save list event
#[napi(object)]
#[derive(Serialize)]
pub struct CloudSaveListEvent {
    pub event_id: u32,
    pub request_id: i64,
    pub error: Option<SdkError>,
    pub saves: Vec<CloudSaveInfo>,
}

/// Cloud save create/update event
#[napi(object)]
#[derive(Serialize)]
pub struct CloudSaveCreateEvent {
    pub event_id: u32,
    pub request_id: i64,
    pub error: Option<SdkError>,
    pub save: Option<CloudSaveInfo>,
}

/// Cloud save delete event
#[napi(object)]
#[derive(Serialize)]
pub struct CloudSaveDeleteEvent {
    pub event_id: u32,
    pub request_id: i64,
    pub error: Option<SdkError>,
    pub uuid: String,
}

/// Cloud save get file event
#[napi(object)]
#[derive(Serialize)]
pub struct CloudSaveGetFileEvent {
    pub event_id: u32,
    pub request_id: i64,
    pub error: Option<SdkError>,
    #[serde(skip)]
    pub data: Buffer,
}

/// Unknown event
#[napi(object)]
#[derive(Serialize)]
pub struct UnknownEvent {
    pub event_id: u32,
}

/// Convert a TapEvent into a serde_json::Value for passing to JavaScript
fn convert_event_to_json(event: TapEvent) -> serde_json::Result<serde_json::Value> {
    match event {
        TapEvent::SystemStateChanged(data) => serde_json::to_value(SystemStateChangedEvent {
            event_id: event_id::SYSTEM_STATE_CHANGED,
            state: system_state_to_u32(data.state),
        }),
        TapEvent::AuthorizeFinished(data) => serde_json::to_value(AuthorizeFinishedEvent {
            event_id: event_id::AUTHORIZE_FINISHED,
            is_cancel: data.is_cancel,
            error: data.error,
            token: data.token.map(|t| AuthToken {
                token_type: t.token_type,
                kid: t.kid,
                mac_key: t.mac_key,
                mac_algorithm: t.mac_algorithm,
                scope: t.scope,
            }),
        }),
        TapEvent::GamePlayableStatusChanged(data) => {
            serde_json::to_value(GamePlayableStatusChangedEvent {
                event_id: event_id::GAME_PLAYABLE_STATUS_CHANGED,
                is_playable: data.is_playable,
            })
        }
        TapEvent::DlcPlayableStatusChanged(data) => {
            serde_json::to_value(DlcPlayableStatusChangedEvent {
                event_id: event_id::DLC_PLAYABLE_STATUS_CHANGED,
                dlc_id: data.dlc_id,
                is_playable: data.is_playable,
            })
        }
        TapEvent::CloudSaveList(data) => serde_json::to_value(CloudSaveListEvent {
            event_id: event_id::CLOUD_SAVE_LIST,
            request_id: data.request_id,
            error: data.error.map(|(code, message)| SdkError { code, message }),
            saves: data.saves.into_iter().map(CloudSaveInfo::from).collect(),
        }),
        TapEvent::CloudSaveCreate(data) => serde_json::to_value(CloudSaveCreateEvent {
            event_id: event_id::CLOUD_SAVE_CREATE,
            request_id: data.request_id,
            error: data.error.map(|(code, message)| SdkError { code, message }),
            save: data.save.map(CloudSaveInfo::from),
        }),
        TapEvent::CloudSaveUpdate(data) => serde_json::to_value(CloudSaveCreateEvent {
            event_id: event_id::CLOUD_SAVE_UPDATE,
            request_id: data.request_id,
            error: data.error.map(|(code, message)| SdkError { code, message }),
            save: data.save.map(CloudSaveInfo::from),
        }),
        TapEvent::CloudSaveDelete(data) => serde_json::to_value(CloudSaveDeleteEvent {
            event_id: event_id::CLOUD_SAVE_DELETE,
            request_id: data.request_id,
            error: data.error.map(|(code, message)| SdkError { code, message }),
            uuid: data.uuid,
        }),
        TapEvent::CloudSaveGetData(data) => serde_json::to_value(CloudSaveGetFileEvent {
            event_id: event_id::CLOUD_SAVE_GET_DATA,
            request_id: data.request_id,
            error: data.error.map(|(code, message)| SdkError { code, message }),
            data: Buffer::from(data.data),
        }),
        TapEvent::CloudSaveGetCover(data) => serde_json::to_value(CloudSaveGetFileEvent {
            event_id: event_id::CLOUD_SAVE_GET_COVER,
            request_id: data.request_id,
            error: data.error.map(|(code, message)| SdkError { code, message }),
            data: Buffer::from(data.data),
        }),
        TapEvent::Unknown { event_id: id } => serde_json::to_value(UnknownEvent { event_id: id }),
    }
}

/// TapTap PC SDK wrapper for Node.js
///
/// Events are automatically pushed to the provided callback via a background
/// polling thread. There is no need to call `runCallbacks()` manually.
#[napi]
pub struct TapSdk {
    inner: Option<tapsdk_pc::TapSdk>,
    running: Arc<AtomicBool>,
    handle: Option<std::thread::JoinHandle<()>>,
}

#[napi]
impl TapSdk {
    /// Check if the app needs to restart (call before init)
    ///
    /// If this returns true, TapTap will relaunch the game - exit immediately.
    #[napi]
    pub fn restart_app_if_necessary(client_id: String) -> Result<bool> {
        tapsdk_pc::restart_app_if_necessary(&client_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Initialize the SDK and start the background event loop.
    ///
    /// The provided callback will be called with each event as it arrives.
    #[napi(
        constructor,
        ts_args_type = "pubKey: string, callback: (event: any) => void"
    )]
    pub fn new(pub_key: String, callback: Function<'_, serde_json::Value, ()>) -> Result<Self> {
        let inner =
            tapsdk_pc::TapSdk::init(&pub_key).map_err(|e| Error::from_reason(e.to_string()))?;

        // Create a threadsafe function from the JS callback so we can call it
        // from the background thread.
        let tsfn: ThreadsafeFunction<serde_json::Value, ()> = callback
            .build_threadsafe_function()
            .callee_handled::<true>()
            .build()?;

        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        // Spawn a background thread with a tokio runtime that periodically
        // polls the C SDK for events and pushes them to JavaScript.
        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .build()
                .expect("Failed to create tokio runtime for event loop");

            rt.block_on(async {
                let mut interval = tokio::time::interval(std::time::Duration::from_millis(50));

                while running_clone.load(Ordering::Relaxed) {
                    interval.tick().await;
                    let events = tapsdk_pc::callback::poll_events();
                    for event in events {
                        if let Ok(js_event) = convert_event_to_json(event) {
                            tsfn.call(Ok(js_event), ThreadsafeFunctionCallMode::NonBlocking);
                        }
                    }
                }
            });
        });

        Ok(TapSdk {
            inner: Some(inner),
            running,
            handle: Some(handle),
        })
    }

    /// Get the client ID
    #[napi]
    pub fn get_client_id(&self) -> Option<String> {
        self.inner.as_ref()?.get_client_id()
    }

    /// Check if the SDK is initialized
    #[napi]
    pub fn is_initialized() -> bool {
        tapsdk_pc::is_initialized()
    }

    /// Request user authorization
    #[napi]
    pub fn authorize(&self, scopes: String) -> Result<()> {
        tapsdk_pc::user::authorize(&scopes).map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Get the current user's OpenID
    #[napi]
    pub fn get_open_id(&self) -> Option<String> {
        tapsdk_pc::user::get_open_id()
    }

    /// Check if the user owns the current game
    #[napi]
    pub fn is_game_owned(&self) -> bool {
        tapsdk_pc::ownership::is_game_owned()
    }

    /// Check if the user owns a specific DLC
    #[napi]
    pub fn is_dlc_owned(&self, dlc_id: String) -> bool {
        tapsdk_pc::dlc::is_dlc_owned(&dlc_id)
    }

    /// Show the store page for a specific DLC
    #[napi]
    pub fn show_dlc_store(&self, dlc_id: String) -> Result<bool> {
        tapsdk_pc::dlc::show_dlc_store(&dlc_id).map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Shut down the SDK and stop the background event loop.
    #[napi]
    pub fn shutdown(&mut self) {
        // Signal the background thread to stop
        self.running.store(false, Ordering::Relaxed);

        // Wait for the background thread to finish
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }

        // Shut down the underlying SDK
        if let Some(inner) = self.inner.take() {
            inner.shutdown();
        }
    }
}

impl Drop for TapSdk {
    fn drop(&mut self) {
        // Ensure the background thread is stopped if shutdown() wasn't called
        self.running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
        // inner's Drop will handle TapSDK_Shutdown() if not already taken
    }
}

/// Cloud save API
#[napi]
pub struct CloudSave {
    inner: tapsdk_pc::CloudSave,
}

#[napi]
impl CloudSave {
    /// Get the cloud save singleton
    #[napi(factory)]
    pub fn get() -> Result<Self> {
        let inner = tapsdk_pc::CloudSave::get()
            .ok_or_else(|| Error::from_reason("SDK not initialized or CloudSave unavailable"))?;
        Ok(CloudSave { inner })
    }

    /// Request the list of cloud saves
    #[napi]
    pub fn list(&self, request_id: i64) -> Result<()> {
        self.inner
            .list(request_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Create a new cloud save
    #[napi]
    pub fn create(&self, request_id: i64, request: CreateSaveRequest) -> Result<()> {
        let rust_request = tapsdk_pc::cloudsave::CreateSaveRequest {
            name: request.name,
            summary: request.summary,
            extra: request.extra,
            playtime: request.playtime,
            data_file_path: PathBuf::from(request.data_file_path).into_boxed_path(),
            cover_file_path: request
                .cover_file_path
                .map(|p| PathBuf::from(p).into_boxed_path()),
        };

        self.inner
            .create(request_id, &rust_request)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Update an existing cloud save
    #[napi]
    pub fn update(&self, request_id: i64, request: UpdateSaveRequest) -> Result<()> {
        let rust_request = tapsdk_pc::cloudsave::UpdateSaveRequest {
            uuid: request.uuid,
            name: request.name,
            summary: request.summary,
            extra: request.extra,
            playtime: request.playtime,
            data_file_path: PathBuf::from(request.data_file_path).into_boxed_path(),
            cover_file_path: request
                .cover_file_path
                .map(|p| PathBuf::from(p).into_boxed_path()),
        };

        self.inner
            .update(request_id, &rust_request)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Delete a cloud save
    #[napi]
    pub fn delete(&self, request_id: i64, uuid: String) -> Result<()> {
        self.inner
            .delete(request_id, &uuid)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Get the data file for a cloud save
    #[napi]
    pub fn get_data(&self, request_id: i64, uuid: String, file_id: String) -> Result<()> {
        self.inner
            .get_data(request_id, &uuid, &file_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Get the cover image for a cloud save
    #[napi]
    pub fn get_cover(&self, request_id: i64, uuid: String, file_id: String) -> Result<()> {
        self.inner
            .get_cover(request_id, &uuid, &file_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }
}

fn system_state_to_u32(state: SystemState) -> u32 {
    match state {
        SystemState::Unknown => system_state::UNKNOWN,
        SystemState::PlatformOnline => system_state::PLATFORM_ONLINE,
        SystemState::PlatformOffline => system_state::PLATFORM_OFFLINE,
        SystemState::PlatformShutdown => system_state::PLATFORM_SHUTDOWN,
    }
}
