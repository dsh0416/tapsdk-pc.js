//! Node.js bindings for TapTap PC SDK
//!
//! This crate provides NAPI-RS bindings to expose the TapTap PC SDK to Node.js.

#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::Serialize;
use std::path::PathBuf;

use tapsdk_pc::callback::{
    AuthorizeFinishedData, CloudSaveCreateData, CloudSaveDeleteData, CloudSaveGetFileData,
    CloudSaveInfo as RustCloudSaveInfo, CloudSaveListData, DlcPlayableStatusChangedData,
    GamePlayableStatusChangedData, SystemStateChangedData,
};
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

/// TapTap PC SDK wrapper for Node.js
#[napi]
pub struct TapSdk {
    inner: Option<tapsdk_pc::TapSdk>,
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

    /// Initialize the SDK
    #[napi(constructor)]
    pub fn new(pub_key: String) -> Result<Self> {
        let inner = tapsdk_pc::TapSdk::init(&pub_key)
            .map_err(|e| Error::from_reason(e.to_string()))?;
        Ok(TapSdk { inner: Some(inner) })
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

    /// Poll for events from the SDK
    ///
    /// Call this regularly (e.g., in your game loop) to receive events.
    /// Returns an array of event objects with different types based on event_id.
    #[napi(ts_return_type = "Array<SystemStateChangedEvent | AuthorizeFinishedEvent | GamePlayableStatusChangedEvent | DlcPlayableStatusChangedEvent | CloudSaveListEvent | CloudSaveCreateEvent | CloudSaveDeleteEvent | CloudSaveGetFileEvent | UnknownEvent>")]
    pub fn run_callbacks(&self, env: Env) -> Result<Vec<napi::JsUnknown>> {
        let inner = self.inner.as_ref()
            .ok_or_else(|| Error::from_reason("SDK has been shut down"))?;
        
        let events = inner.run_callbacks();
        let mut result = Vec::with_capacity(events.len());

        for event in events {
            let js_event = match event {
                tapsdk_pc::callback::TapEvent::SystemStateChanged(data) => {
                    convert_system_state_event(env, data)?
                }
                tapsdk_pc::callback::TapEvent::AuthorizeFinished(data) => {
                    convert_authorize_event(env, data)?
                }
                tapsdk_pc::callback::TapEvent::GamePlayableStatusChanged(data) => {
                    convert_game_playable_event(env, data)?
                }
                tapsdk_pc::callback::TapEvent::DlcPlayableStatusChanged(data) => {
                    convert_dlc_playable_event(env, data)?
                }
                tapsdk_pc::callback::TapEvent::CloudSaveList(data) => {
                    convert_cloud_save_list_event(env, data)?
                }
                tapsdk_pc::callback::TapEvent::CloudSaveCreate(data) => {
                    convert_cloud_save_create_event(env, event_id::CLOUD_SAVE_CREATE, data)?
                }
                tapsdk_pc::callback::TapEvent::CloudSaveUpdate(data) => {
                    convert_cloud_save_create_event(env, event_id::CLOUD_SAVE_UPDATE, data)?
                }
                tapsdk_pc::callback::TapEvent::CloudSaveDelete(data) => {
                    convert_cloud_save_delete_event(env, data)?
                }
                tapsdk_pc::callback::TapEvent::CloudSaveGetData(data) => {
                    convert_cloud_save_get_file_event(env, event_id::CLOUD_SAVE_GET_DATA, data)?
                }
                tapsdk_pc::callback::TapEvent::CloudSaveGetCover(data) => {
                    convert_cloud_save_get_file_event(env, event_id::CLOUD_SAVE_GET_COVER, data)?
                }
                tapsdk_pc::callback::TapEvent::Unknown { event_id: id } => {
                    env.to_js_value(&UnknownEvent { event_id: id })?
                }
            };
            result.push(js_event);
        }

        Ok(result)
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

    /// Shut down the SDK
    #[napi]
    pub fn shutdown(&mut self) {
        if let Some(inner) = self.inner.take() {
            inner.shutdown();
        }
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
            cover_file_path: request.cover_file_path.map(|p| PathBuf::from(p).into_boxed_path()),
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
            cover_file_path: request.cover_file_path.map(|p| PathBuf::from(p).into_boxed_path()),
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

fn convert_system_state_event(env: Env, data: SystemStateChangedData) -> Result<napi::JsUnknown> {
    env.to_js_value(&SystemStateChangedEvent {
        event_id: event_id::SYSTEM_STATE_CHANGED,
        state: system_state_to_u32(data.state),
    })
}

fn convert_authorize_event(env: Env, data: AuthorizeFinishedData) -> Result<napi::JsUnknown> {
    env.to_js_value(&AuthorizeFinishedEvent {
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
    })
}

fn convert_game_playable_event(
    env: Env,
    data: GamePlayableStatusChangedData,
) -> Result<napi::JsUnknown> {
    env.to_js_value(&GamePlayableStatusChangedEvent {
        event_id: event_id::GAME_PLAYABLE_STATUS_CHANGED,
        is_playable: data.is_playable,
    })
}

fn convert_dlc_playable_event(
    env: Env,
    data: DlcPlayableStatusChangedData,
) -> Result<napi::JsUnknown> {
    env.to_js_value(&DlcPlayableStatusChangedEvent {
        event_id: event_id::DLC_PLAYABLE_STATUS_CHANGED,
        dlc_id: data.dlc_id,
        is_playable: data.is_playable,
    })
}

fn convert_cloud_save_list_event(env: Env, data: CloudSaveListData) -> Result<napi::JsUnknown> {
    env.to_js_value(&CloudSaveListEvent {
        event_id: event_id::CLOUD_SAVE_LIST,
        request_id: data.request_id,
        error: data.error.map(|(code, message)| SdkError { code, message }),
        saves: data.saves.into_iter().map(CloudSaveInfo::from).collect(),
    })
}

fn convert_cloud_save_create_event(
    env: Env,
    event_id: u32,
    data: CloudSaveCreateData,
) -> Result<napi::JsUnknown> {
    env.to_js_value(&CloudSaveCreateEvent {
        event_id,
        request_id: data.request_id,
        error: data.error.map(|(code, message)| SdkError { code, message }),
        save: data.save.map(CloudSaveInfo::from),
    })
}

fn convert_cloud_save_delete_event(
    env: Env,
    data: CloudSaveDeleteData,
) -> Result<napi::JsUnknown> {
    env.to_js_value(&CloudSaveDeleteEvent {
        event_id: event_id::CLOUD_SAVE_DELETE,
        request_id: data.request_id,
        error: data.error.map(|(code, message)| SdkError { code, message }),
        uuid: data.uuid,
    })
}

fn convert_cloud_save_get_file_event(
    env: Env,
    event_id: u32,
    data: CloudSaveGetFileData,
) -> Result<napi::JsUnknown> {
    env.to_js_value(&CloudSaveGetFileEvent {
        event_id,
        request_id: data.request_id,
        error: data.error.map(|(code, message)| SdkError { code, message }),
        data: Buffer::from(data.data),
    })
}
