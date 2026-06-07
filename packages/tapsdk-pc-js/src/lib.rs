//! Node.js bindings for TapTap PC SDK
//!
//! This crate provides NAPI-RS bindings to expose the TapTap PC SDK to Node.js.
//! Events are pushed to JavaScript automatically via a background polling thread.

#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;
use napi_derive::napi;
use serde::Serialize;
use serde_json::json;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tapsdk_pc::callback::AchievementInfo as RustAchievementInfo;
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
    #[napi]
    pub const ACHIEVEMENT_UNLOCK: u32 = 7001;
    #[napi]
    pub const ACHIEVEMENT_INCREMENT: u32 = 7002;
    #[napi]
    pub const COMPLIANCE_ENSURE_REAL_NAME: u32 = 8001;
    #[napi]
    pub const COMPLIANCE_ACTIONS_EVENT: u32 = 8002;
    #[napi]
    pub const ONLINE_GAME_EVENT: u32 = 10001;
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct SdkError {
    pub code: i64,
    pub message: String,
}

/// Cloud save information
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
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

/// Achievement information
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementInfo {
    pub id: String,
    pub name: String,
    pub current_steps: i64,
    pub newly_unlocked: bool,
}

impl From<RustAchievementInfo> for AchievementInfo {
    fn from(info: RustAchievementInfo) -> Self {
        AchievementInfo {
            id: info.id,
            name: info.name,
            current_steps: i64::try_from(info.current_steps).unwrap_or(i64::MAX),
            newly_unlocked: info.newly_unlocked,
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

/// Payment limit check response.
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentLimitResponse {
    pub allow: bool,
    pub title: String,
    pub description: String,
}

/// Online game room match parameter.
#[napi(object)]
pub struct OnlineGameMatchParam {
    pub key: String,
    pub value: String,
}

/// Online game room configuration.
#[napi(object)]
pub struct OnlineGameRoomConfig {
    pub max_player_count: u32,
    pub room_type: String,
    pub match_params: Option<Vec<OnlineGameMatchParam>>,
    pub name: Option<String>,
    pub custom_properties: Option<String>,
}

/// Online game player configuration.
#[napi(object)]
pub struct OnlineGamePlayerConfig {
    pub custom_status: i32,
    pub custom_properties: Option<String>,
}

/// Create or match room request.
#[napi(object)]
pub struct OnlineGameRoomRequest {
    pub room: OnlineGameRoomConfig,
    pub player: OnlineGamePlayerConfig,
}

/// Get room list request.
#[napi(object)]
pub struct OnlineGameGetRoomListRequest {
    pub room_type: Option<String>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,
}

/// Join room request.
#[napi(object)]
pub struct OnlineGameJoinRoomRequest {
    pub room_id: String,
    pub player: OnlineGamePlayerConfig,
}

/// Update room properties request.
#[napi(object)]
pub struct OnlineGameUpdateRoomPropertiesRequest {
    pub name: Option<String>,
    pub custom_properties: Option<String>,
}

/// Send custom message request.
#[napi(object)]
pub struct OnlineGameSendCustomMessageRequest {
    pub msg: String,
    pub receiver_type: u32,
    pub receivers: Option<Vec<String>>,
}

/// System state changed event
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStateChangedEvent {
    pub event_id: u32,
    pub state: u32,
}

/// Authorization finished event
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeFinishedEvent {
    pub event_id: u32,
    pub is_cancel: bool,
    pub error: Option<String>,
    pub token: Option<AuthToken>,
}

/// Game playable status changed event
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePlayableStatusChangedEvent {
    pub event_id: u32,
    pub is_playable: bool,
}

/// DLC playable status changed event
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DlcPlayableStatusChangedEvent {
    pub event_id: u32,
    pub dlc_id: String,
    pub is_playable: bool,
}

/// Cloud save list event
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudSaveListEvent {
    pub event_id: u32,
    pub request_id: i64,
    pub error: Option<SdkError>,
    pub saves: Vec<CloudSaveInfo>,
}

/// Cloud save create/update event
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudSaveCreateEvent {
    pub event_id: u32,
    pub request_id: i64,
    pub error: Option<SdkError>,
    pub save: Option<CloudSaveInfo>,
}

/// Cloud save delete event
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudSaveDeleteEvent {
    pub event_id: u32,
    pub request_id: i64,
    pub error: Option<SdkError>,
    pub uuid: String,
}

/// Cloud save get file event
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudSaveGetFileEvent {
    pub event_id: u32,
    pub request_id: i64,
    pub error: Option<SdkError>,
    #[serde(skip)]
    pub data: Buffer,
}

/// Achievement unlock/increment event
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementEvent {
    pub event_id: u32,
    pub request_id: i64,
    pub error: Option<SdkError>,
    pub achievement: Option<AchievementInfo>,
    pub platinum_achievement: Option<AchievementInfo>,
}

/// Unknown event
#[napi(object)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
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
        TapEvent::AchievementUnlock(data) => serde_json::to_value(AchievementEvent {
            event_id: event_id::ACHIEVEMENT_UNLOCK,
            request_id: data.request_id,
            error: data.error.map(|(code, message)| SdkError { code, message }),
            achievement: data.achievement.map(AchievementInfo::from),
            platinum_achievement: data.platinum_achievement.map(AchievementInfo::from),
        }),
        TapEvent::AchievementIncrement(data) => serde_json::to_value(AchievementEvent {
            event_id: event_id::ACHIEVEMENT_INCREMENT,
            request_id: data.request_id,
            error: data.error.map(|(code, message)| SdkError { code, message }),
            achievement: data.achievement.map(AchievementInfo::from),
            platinum_achievement: data.platinum_achievement.map(AchievementInfo::from),
        }),
        TapEvent::ComplianceEnsureRealName(data) => Ok(json!({
            "eventId": event_id::COMPLIANCE_ENSURE_REAL_NAME,
            "requestId": data.request_id,
            "error": data.error.map(|(code, message)| SdkError { code, message }),
            "status": data.status,
        })),
        TapEvent::ComplianceActionsEvent(data) => Ok(json!({
            "eventId": event_id::COMPLIANCE_ACTIONS_EVENT,
            "actions": data.actions.into_iter().map(|action| {
                json!({
                    "actionType": action.action_type,
                    "title": action.title,
                    "description": action.description,
                    "displayDurationSeconds": action.display_duration_seconds,
                })
            }).collect::<Vec<_>>(),
        })),
        TapEvent::OnlineGame(data) => Ok(json!({
            "eventId": event_id::ONLINE_GAME_EVENT,
            "requestId": data.request_id,
            "error": data.error.map(|(code, message)| SdkError { code, message }),
            "onlineGameEventId": data.online_game_event_id,
            "data": online_game_payload_to_json(data.data),
        })),
        TapEvent::Unknown { event_id: id } => serde_json::to_value(UnknownEvent { event_id: id }),
    }
}

fn online_game_payload_to_json(
    payload: tapsdk_pc::callback::OnlineGameEventPayload,
) -> serde_json::Value {
    use tapsdk_pc::callback::OnlineGameEventPayload;

    match payload {
        OnlineGameEventPayload::Empty => json!(null),
        OnlineGameEventPayload::Connect { player_id } => json!({ "playerId": player_id }),
        OnlineGameEventPayload::Room { room_info } => {
            json!({ "roomInfo": online_game_room_info_to_json(room_info) })
        }
        OnlineGameEventPayload::RoomList {
            rooms,
            offset,
            has_more,
        } => json!({
            "rooms": rooms.into_iter().map(online_game_room_basic_info_to_json).collect::<Vec<_>>(),
            "offset": offset,
            "hasMore": has_more,
        }),
        OnlineGameEventPayload::PlayerOffline {
            room_id,
            room_owner_id,
            player_id,
        } => json!({
            "roomId": room_id,
            "roomOwnerId": room_owner_id,
            "playerId": player_id,
        }),
        OnlineGameEventPayload::PlayerEnterRoom {
            room_id,
            player_info,
        } => json!({
            "roomId": room_id,
            "playerInfo": online_game_player_info_to_json(player_info),
        }),
        OnlineGameEventPayload::PlayerLeaveRoom {
            room_id,
            room_owner_id,
            player_id,
        } => json!({
            "roomId": room_id,
            "roomOwnerId": room_owner_id,
            "playerId": player_id,
        }),
        OnlineGameEventPayload::PlayerCustomStatus { player_id, status } => json!({
            "playerId": player_id,
            "status": status,
        }),
        OnlineGameEventPayload::PlayerCustomProperties {
            player_id,
            properties,
        } => json!({
            "playerId": player_id,
            "properties": properties,
        }),
        OnlineGameEventPayload::RoomProperties {
            id,
            name,
            custom_properties,
        } => json!({
            "id": id,
            "name": name,
            "customProperties": custom_properties,
        }),
        OnlineGameEventPayload::CustomMessage { player_id, msg } => json!({
            "playerId": player_id,
            "msg": msg,
        }),
        OnlineGameEventPayload::RoomPlayerKicked { room_id, player_id } => json!({
            "roomId": room_id,
            "playerId": player_id,
        }),
        OnlineGameEventPayload::FrameSyncStart {
            room_info,
            frame_sync_id,
            seed,
            server_tms,
        } => json!({
            "roomInfo": online_game_room_info_to_json(room_info),
            "frameSyncId": frame_sync_id,
            "seed": seed,
            "serverTms": server_tms,
        }),
        OnlineGameEventPayload::Frame { id, inputs } => json!({
            "id": id,
            "inputs": inputs.into_iter().map(|input| json!({
                "playerId": input.player_id,
                "data": input.data,
                "serverTms": input.server_tms,
            })).collect::<Vec<_>>(),
        }),
        OnlineGameEventPayload::FrameSyncStop {
            room_id,
            frame_sync_id,
            reason,
        } => json!({
            "roomId": room_id,
            "frameSyncId": frame_sync_id,
            "reason": reason,
        }),
        OnlineGameEventPayload::Unknown => json!({}),
    }
}

fn online_game_room_info_to_json(
    room: tapsdk_pc::callback::OnlineGameRoomInfo,
) -> serde_json::Value {
    json!({
        "id": room.id,
        "name": room.name,
        "roomType": room.room_type,
        "ownerId": room.owner_id,
        "status": room.status,
        "customProperties": room.custom_properties,
        "maxPlayerCount": room.max_player_count,
        "playerCount": room.player_count,
        "players": room.players.into_iter().map(online_game_player_info_to_json).collect::<Vec<_>>(),
        "createTime": room.create_time,
    })
}

fn online_game_room_basic_info_to_json(
    room: tapsdk_pc::callback::OnlineGameRoomBasicInfo,
) -> serde_json::Value {
    json!({
        "id": room.id,
        "name": room.name,
        "roomType": room.room_type,
        "status": room.status,
        "customProperties": room.custom_properties,
        "maxPlayerCount": room.max_player_count,
        "playerCount": room.player_count,
        "createTime": room.create_time,
    })
}

fn online_game_player_info_to_json(
    player: tapsdk_pc::callback::OnlineGamePlayerInfo,
) -> serde_json::Value {
    json!({
        "id": player.id,
        "status": player.status,
        "customStatus": player.custom_status,
        "customProperties": player.custom_properties,
    })
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
        let tsfn = callback.build_threadsafe_function().build()?;

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
                            tsfn.call(js_event, ThreadsafeFunctionCallMode::NonBlocking);
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

/// Achievement API
#[napi]
pub struct Achievement {
    inner: tapsdk_pc::Achievement,
}

#[napi]
impl Achievement {
    /// Get the achievement singleton
    #[napi(factory)]
    pub fn get() -> Result<Self> {
        let inner = tapsdk_pc::Achievement::get()
            .ok_or_else(|| Error::from_reason("SDK not initialized or Achievement unavailable"))?;
        Ok(Achievement { inner })
    }

    /// Unlock an achievement
    #[napi]
    pub fn unlock(&self, request_id: i64, achievement_id: String) -> Result<()> {
        self.inner
            .unlock(request_id, &achievement_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Increment progress for a step-based achievement
    #[napi]
    pub fn increment(&self, request_id: i64, achievement_id: String, steps: i64) -> Result<()> {
        let steps = u64::try_from(steps)
            .map_err(|_| Error::from_reason("steps must be a non-negative integer"))?;

        self.inner
            .increment(request_id, &achievement_id, steps)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Open the TapTap achievements page
    #[napi(js_name = "showAchievements")]
    pub fn show_achievements(&self) -> Result<()> {
        self.inner
            .show_achievements()
            .map_err(|e| Error::from_reason(e.to_string()))
    }
}

/// Compliance API
#[napi]
pub struct Compliance {
    inner: tapsdk_pc::Compliance,
}

#[napi]
impl Compliance {
    /// Get the compliance singleton
    #[napi(factory)]
    pub fn get() -> Result<Self> {
        let inner = tapsdk_pc::Compliance::get()
            .ok_or_else(|| Error::from_reason("SDK not initialized or Compliance unavailable"))?;
        Ok(Compliance { inner })
    }

    /// Ensure the current user has completed real-name verification
    #[napi(js_name = "ensureRealName")]
    pub fn ensure_real_name(&self, request_id: i64) -> Result<()> {
        self.inner
            .ensure_real_name(request_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Enable anti-addiction checks
    #[napi(js_name = "enableAntiAddiction")]
    pub fn enable_anti_addiction(&self) -> Result<()> {
        self.inner
            .enable_anti_addiction()
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Check whether a payment amount is allowed
    #[napi(js_name = "checkPaymentLimit")]
    pub fn check_payment_limit(&self, amount: u32) -> Result<PaymentLimitResponse> {
        self.inner
            .check_payment_limit(amount)
            .map(|response| PaymentLimitResponse {
                allow: response.allow,
                title: response.title,
                description: response.description,
            })
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Submit a successful payment amount
    #[napi(js_name = "submitPayment")]
    pub fn submit_payment(&self, amount: u32) -> Result<()> {
        self.inner
            .submit_payment(amount)
            .map_err(|e| Error::from_reason(e.to_string()))
    }
}

/// Online game API
#[napi]
pub struct OnlineGame {
    inner: tapsdk_pc::OnlineGame,
}

#[napi]
impl OnlineGame {
    /// Get the online game singleton
    #[napi(factory)]
    pub fn get() -> Result<Self> {
        let inner = tapsdk_pc::OnlineGame::get()
            .ok_or_else(|| Error::from_reason("SDK not initialized or OnlineGame unavailable"))?;
        Ok(OnlineGame { inner })
    }

    /// Connect to the online game service
    #[napi]
    pub fn connect(&self, request_id: i64) -> Result<()> {
        self.inner
            .connect(request_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Disconnect from the online game service
    #[napi]
    pub fn disconnect(&self, request_id: i64) -> Result<()> {
        self.inner
            .disconnect(request_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Create a room
    #[napi(js_name = "createRoom")]
    pub fn create_room(&self, request_id: i64, request: OnlineGameRoomRequest) -> Result<()> {
        let request = room_request_to_create(request)?;
        self.inner
            .create_room(request_id, &request)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Match or create a room
    #[napi(js_name = "matchRoom")]
    pub fn match_room(&self, request_id: i64, request: OnlineGameRoomRequest) -> Result<()> {
        let request = room_request_to_match(request)?;
        self.inner
            .match_room(request_id, &request)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Get joinable room list
    #[napi(js_name = "getRoomList")]
    pub fn get_room_list(
        &self,
        request_id: i64,
        request: OnlineGameGetRoomListRequest,
    ) -> Result<()> {
        let request = tapsdk_pc::onlinegame::GetRoomListRequest {
            room_type: request.room_type,
            offset: request.offset.unwrap_or(0),
            limit: request.limit.unwrap_or(20),
        };

        self.inner
            .get_room_list(request_id, &request)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Join a room
    #[napi(js_name = "joinRoom")]
    pub fn join_room(&self, request_id: i64, request: OnlineGameJoinRoomRequest) -> Result<()> {
        let request = tapsdk_pc::onlinegame::JoinRoomRequest {
            room_id: request.room_id,
            player: player_config_to_rust(request.player),
        };

        self.inner
            .join_room(request_id, &request)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Leave the current room
    #[napi(js_name = "leaveRoom")]
    pub fn leave_room(&self, request_id: i64) -> Result<()> {
        self.inner
            .leave_room(request_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Update the current player's custom status
    #[napi(js_name = "updatePlayerCustomStatus")]
    pub fn update_player_custom_status(&self, request_id: i64, status: i32) -> Result<()> {
        self.inner
            .update_player_custom_status(request_id, status)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Update the current player's custom properties
    #[napi(js_name = "updatePlayerCustomProperties")]
    pub fn update_player_custom_properties(
        &self,
        request_id: i64,
        properties: String,
    ) -> Result<()> {
        self.inner
            .update_player_custom_properties(request_id, &properties)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Update the current room's properties
    #[napi(js_name = "updateRoomProperties")]
    pub fn update_room_properties(
        &self,
        request_id: i64,
        request: OnlineGameUpdateRoomPropertiesRequest,
    ) -> Result<()> {
        let request = tapsdk_pc::onlinegame::UpdateRoomPropertiesRequest {
            name: request.name,
            custom_properties: request.custom_properties,
        };

        self.inner
            .update_room_properties(request_id, &request)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Send a custom message
    #[napi(js_name = "sendCustomMessage")]
    pub fn send_custom_message(
        &self,
        request_id: i64,
        request: OnlineGameSendCustomMessageRequest,
    ) -> Result<()> {
        let request = tapsdk_pc::onlinegame::SendCustomMessageRequest {
            msg: request.msg,
            receiver_type: message_receiver_type_from_u32(request.receiver_type)?,
            receivers: request.receivers.unwrap_or_default(),
        };

        self.inner
            .send_custom_message(request_id, &request)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Kick a player from the current room
    #[napi(js_name = "kickRoomPlayer")]
    pub fn kick_room_player(&self, request_id: i64, player_id: String) -> Result<()> {
        self.inner
            .kick_room_player(request_id, &player_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Start frame synchronization
    #[napi(js_name = "startFrameSync")]
    pub fn start_frame_sync(&self, request_id: i64) -> Result<()> {
        self.inner
            .start_frame_sync(request_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Send frame input data
    #[napi(js_name = "sendFrameInput")]
    pub fn send_frame_input(&self, request_id: i64, data: String) -> Result<()> {
        self.inner
            .send_frame_input(request_id, &data)
            .map_err(|e| Error::from_reason(e.to_string()))
    }

    /// Stop frame synchronization
    #[napi(js_name = "stopFrameSync")]
    pub fn stop_frame_sync(&self, request_id: i64) -> Result<()> {
        self.inner
            .stop_frame_sync(request_id)
            .map_err(|e| Error::from_reason(e.to_string()))
    }
}

fn message_receiver_type_from_u32(
    receiver_type: u32,
) -> Result<tapsdk_pc::onlinegame::MessageReceiverType> {
    match receiver_type {
        0 => Ok(tapsdk_pc::onlinegame::MessageReceiverType::Room),
        1 => Ok(tapsdk_pc::onlinegame::MessageReceiverType::Players),
        _ => Err(Error::from_reason(
            "receiverType must be 0 (room) or 1 (players)",
        )),
    }
}

fn room_request_to_create(
    request: OnlineGameRoomRequest,
) -> Result<tapsdk_pc::onlinegame::CreateRoomRequest> {
    Ok(tapsdk_pc::onlinegame::CreateRoomRequest {
        room: room_config_to_rust(request.room),
        player: player_config_to_rust(request.player),
    })
}

fn room_request_to_match(
    request: OnlineGameRoomRequest,
) -> Result<tapsdk_pc::onlinegame::MatchRoomRequest> {
    Ok(tapsdk_pc::onlinegame::MatchRoomRequest {
        room: room_config_to_rust(request.room),
        player: player_config_to_rust(request.player),
    })
}

fn room_config_to_rust(config: OnlineGameRoomConfig) -> tapsdk_pc::onlinegame::RoomConfig {
    tapsdk_pc::onlinegame::RoomConfig {
        max_player_count: config.max_player_count,
        room_type: config.room_type,
        match_params: config
            .match_params
            .unwrap_or_default()
            .into_iter()
            .map(|param| tapsdk_pc::onlinegame::MatchParam {
                key: param.key,
                value: param.value,
            })
            .collect(),
        name: config.name,
        custom_properties: config.custom_properties,
    }
}

fn player_config_to_rust(config: OnlineGamePlayerConfig) -> tapsdk_pc::onlinegame::PlayerConfig {
    tapsdk_pc::onlinegame::PlayerConfig {
        custom_status: config.custom_status,
        custom_properties: config.custom_properties,
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
