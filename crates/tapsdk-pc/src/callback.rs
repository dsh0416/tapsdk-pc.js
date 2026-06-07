//! Callback registry and event handling for TapTap PC SDK

use std::collections::VecDeque;
use std::ffi::CStr;
use std::sync::Mutex;

use crate::error::SystemState;

/// Event IDs matching the C SDK
pub mod event_id {
    pub const UNKNOWN: u32 = 0;
    pub const SYSTEM_STATE_CHANGED: u32 = 1;
    pub const AUTHORIZE_FINISHED: u32 = 2002;
    pub const GAME_PLAYABLE_STATUS_CHANGED: u32 = 4001;
    pub const DLC_PLAYABLE_STATUS_CHANGED: u32 = 4002;
    pub const CLOUD_SAVE_LIST: u32 = 6001;
    pub const CLOUD_SAVE_CREATE: u32 = 6002;
    pub const CLOUD_SAVE_UPDATE: u32 = 6003;
    pub const CLOUD_SAVE_DELETE: u32 = 6004;
    pub const CLOUD_SAVE_GET_DATA: u32 = 6005;
    pub const CLOUD_SAVE_GET_COVER: u32 = 6006;
    pub const ACHIEVEMENT_UNLOCK: u32 = 7001;
    pub const ACHIEVEMENT_INCREMENT: u32 = 7002;
    pub const COMPLIANCE_ENSURE_REAL_NAME: u32 = 8001;
    pub const COMPLIANCE_ACTIONS_EVENT: u32 = 8002;
    pub const ONLINE_GAME_EVENT: u32 = 10001;
}

/// Authorization token returned after successful authorization
#[derive(Debug, Clone, Default)]
pub struct AuthToken {
    pub token_type: String,
    pub kid: String,
    pub mac_key: String,
    pub mac_algorithm: String,
    pub scope: String,
}

/// Authorization finished event data
#[derive(Debug, Clone)]
pub struct AuthorizeFinishedData {
    pub is_cancel: bool,
    pub error: Option<String>,
    pub token: Option<AuthToken>,
}

/// System state changed event data
#[derive(Debug, Clone)]
pub struct SystemStateChangedData {
    pub state: SystemState,
}

/// Game playable status changed event data
#[derive(Debug, Clone)]
pub struct GamePlayableStatusChangedData {
    pub is_playable: bool,
}

/// DLC playable status changed event data
#[derive(Debug, Clone)]
pub struct DlcPlayableStatusChangedData {
    pub dlc_id: String,
    pub is_playable: bool,
}

/// Cloud save info
#[derive(Debug, Clone)]
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

/// Cloud save list response
#[derive(Debug, Clone)]
pub struct CloudSaveListData {
    pub request_id: i64,
    pub error: Option<(i64, String)>,
    pub saves: Vec<CloudSaveInfo>,
}

/// Cloud save create/update response
#[derive(Debug, Clone)]
pub struct CloudSaveCreateData {
    pub request_id: i64,
    pub error: Option<(i64, String)>,
    pub save: Option<CloudSaveInfo>,
}

/// Cloud save delete response
#[derive(Debug, Clone)]
pub struct CloudSaveDeleteData {
    pub request_id: i64,
    pub error: Option<(i64, String)>,
    pub uuid: String,
}

/// Cloud save get file response
#[derive(Debug, Clone)]
pub struct CloudSaveGetFileData {
    pub request_id: i64,
    pub error: Option<(i64, String)>,
    pub data: Vec<u8>,
}

/// Achievement information
#[derive(Debug, Clone)]
pub struct AchievementInfo {
    pub id: String,
    pub name: String,
    pub current_steps: u64,
    pub newly_unlocked: bool,
}

/// Achievement unlock/increment response
#[derive(Debug, Clone)]
pub struct AchievementData {
    pub request_id: i64,
    pub error: Option<(i64, String)>,
    pub achievement: Option<AchievementInfo>,
    pub platinum_achievement: Option<AchievementInfo>,
}

/// Compliance action pushed by anti-addiction checks.
#[derive(Debug, Clone)]
pub struct ComplianceAction {
    pub action_type: u32,
    pub title: String,
    pub description: String,
    pub display_duration_seconds: u32,
}

/// Compliance real-name response.
#[derive(Debug, Clone)]
pub struct ComplianceEnsureRealNameData {
    pub request_id: i64,
    pub error: Option<(i64, String)>,
    pub status: u32,
}

/// Compliance actions notification.
#[derive(Debug, Clone)]
pub struct ComplianceActionsData {
    pub actions: Vec<ComplianceAction>,
}

/// Online game event response or notification.
#[derive(Debug, Clone)]
pub struct OnlineGameEventData {
    pub request_id: i64,
    pub error: Option<(i64, String)>,
    pub online_game_event_id: u32,
    pub data: OnlineGameEventPayload,
}

/// Online game event payload.
#[derive(Debug, Clone)]
pub enum OnlineGameEventPayload {
    Empty,
    Connect {
        player_id: String,
    },
    Room {
        room_info: OnlineGameRoomInfo,
    },
    RoomList {
        rooms: Vec<OnlineGameRoomBasicInfo>,
        offset: u32,
        has_more: bool,
    },
    PlayerOffline {
        room_id: String,
        room_owner_id: String,
        player_id: String,
    },
    PlayerEnterRoom {
        room_id: String,
        player_info: OnlineGamePlayerInfo,
    },
    PlayerLeaveRoom {
        room_id: String,
        room_owner_id: String,
        player_id: String,
    },
    PlayerCustomStatus {
        player_id: String,
        status: i32,
    },
    PlayerCustomProperties {
        player_id: String,
        properties: String,
    },
    RoomProperties {
        id: String,
        name: String,
        custom_properties: String,
    },
    CustomMessage {
        player_id: String,
        msg: String,
    },
    RoomPlayerKicked {
        room_id: String,
        player_id: String,
    },
    FrameSyncStart {
        room_info: OnlineGameRoomInfo,
        frame_sync_id: i32,
        seed: i32,
        server_tms: i64,
    },
    Frame {
        id: u32,
        inputs: Vec<OnlineGameFrameInput>,
    },
    FrameSyncStop {
        room_id: String,
        frame_sync_id: i32,
        reason: i32,
    },
    Unknown,
}

/// Online game player info.
#[derive(Debug, Clone)]
pub struct OnlineGamePlayerInfo {
    pub id: String,
    pub status: i32,
    pub custom_status: i32,
    pub custom_properties: String,
}

/// Online game room info.
#[derive(Debug, Clone)]
pub struct OnlineGameRoomInfo {
    pub id: String,
    pub name: String,
    pub room_type: String,
    pub owner_id: String,
    pub status: i32,
    pub custom_properties: String,
    pub max_player_count: u32,
    pub player_count: u32,
    pub players: Vec<OnlineGamePlayerInfo>,
    pub create_time: i64,
}

/// Online game room basic info.
#[derive(Debug, Clone)]
pub struct OnlineGameRoomBasicInfo {
    pub id: String,
    pub name: String,
    pub room_type: String,
    pub status: i32,
    pub custom_properties: String,
    pub max_player_count: u32,
    pub player_count: u32,
    pub create_time: i64,
}

/// Online game frame input.
#[derive(Debug, Clone)]
pub struct OnlineGameFrameInput {
    pub player_id: String,
    pub data: String,
    pub server_tms: i64,
}

/// Events that can be received from the SDK
#[derive(Debug, Clone)]
pub enum TapEvent {
    /// System state changed
    SystemStateChanged(SystemStateChangedData),
    /// Authorization finished
    AuthorizeFinished(AuthorizeFinishedData),
    /// Game playable status changed
    GamePlayableStatusChanged(GamePlayableStatusChangedData),
    /// DLC playable status changed
    DlcPlayableStatusChanged(DlcPlayableStatusChangedData),
    /// Cloud save list response
    CloudSaveList(CloudSaveListData),
    /// Cloud save create response
    CloudSaveCreate(CloudSaveCreateData),
    /// Cloud save update response
    CloudSaveUpdate(CloudSaveCreateData),
    /// Cloud save delete response
    CloudSaveDelete(CloudSaveDeleteData),
    /// Cloud save get data response
    CloudSaveGetData(CloudSaveGetFileData),
    /// Cloud save get cover response
    CloudSaveGetCover(CloudSaveGetFileData),
    /// Achievement unlock response
    AchievementUnlock(AchievementData),
    /// Achievement increment response
    AchievementIncrement(AchievementData),
    /// Compliance real-name response
    ComplianceEnsureRealName(ComplianceEnsureRealNameData),
    /// Compliance anti-addiction actions notification
    ComplianceActionsEvent(ComplianceActionsData),
    /// Online game event
    OnlineGame(OnlineGameEventData),
    /// Unknown event
    Unknown { event_id: u32 },
}

/// Global event queue
static EVENT_QUEUE: Mutex<VecDeque<TapEvent>> = Mutex::new(VecDeque::new());

/// Register the global callback handler with the SDK
pub fn register_callbacks() {
    unsafe {
        // Register for all event types we care about
        tapsdk_pc_sys::TapSDK_RegisterCallback(
            event_id::SYSTEM_STATE_CHANGED,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_RegisterCallback(event_id::AUTHORIZE_FINISHED, Some(global_callback));
        tapsdk_pc_sys::TapSDK_RegisterCallback(
            event_id::GAME_PLAYABLE_STATUS_CHANGED,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_RegisterCallback(
            event_id::DLC_PLAYABLE_STATUS_CHANGED,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_RegisterCallback(event_id::CLOUD_SAVE_LIST, Some(global_callback));
        tapsdk_pc_sys::TapSDK_RegisterCallback(event_id::CLOUD_SAVE_CREATE, Some(global_callback));
        tapsdk_pc_sys::TapSDK_RegisterCallback(event_id::CLOUD_SAVE_UPDATE, Some(global_callback));
        tapsdk_pc_sys::TapSDK_RegisterCallback(event_id::CLOUD_SAVE_DELETE, Some(global_callback));
        tapsdk_pc_sys::TapSDK_RegisterCallback(
            event_id::CLOUD_SAVE_GET_DATA,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_RegisterCallback(
            event_id::CLOUD_SAVE_GET_COVER,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_RegisterCallback(event_id::ACHIEVEMENT_UNLOCK, Some(global_callback));
        tapsdk_pc_sys::TapSDK_RegisterCallback(
            event_id::ACHIEVEMENT_INCREMENT,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_RegisterCallback(
            event_id::COMPLIANCE_ENSURE_REAL_NAME,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_RegisterCallback(
            event_id::COMPLIANCE_ACTIONS_EVENT,
            Some(global_callback),
        );
    }
}

/// Unregister the global callback handler
pub fn unregister_callbacks() {
    unsafe {
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::SYSTEM_STATE_CHANGED,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::AUTHORIZE_FINISHED,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::GAME_PLAYABLE_STATUS_CHANGED,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::DLC_PLAYABLE_STATUS_CHANGED,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(event_id::CLOUD_SAVE_LIST, Some(global_callback));
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::CLOUD_SAVE_CREATE,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::CLOUD_SAVE_UPDATE,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::CLOUD_SAVE_DELETE,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::CLOUD_SAVE_GET_DATA,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::CLOUD_SAVE_GET_COVER,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::ACHIEVEMENT_UNLOCK,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::ACHIEVEMENT_INCREMENT,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::COMPLIANCE_ENSURE_REAL_NAME,
            Some(global_callback),
        );
        tapsdk_pc_sys::TapSDK_UnregisterCallback(
            event_id::COMPLIANCE_ACTIONS_EVENT,
            Some(global_callback),
        );
    }
}

/// Poll for events from the SDK
///
/// This calls `TapSDK_RunCallbacks()` to process pending callbacks,
/// then returns all events that were queued.
pub fn poll_events() -> Vec<TapEvent> {
    // First, run the SDK callbacks to trigger our callback handler
    unsafe {
        tapsdk_pc_sys::TapSDK_RunCallbacks();
        poll_online_game_events();
    }

    // Then drain the event queue
    let mut queue = EVENT_QUEUE.lock().unwrap();
    queue.drain(..).collect()
}

unsafe fn poll_online_game_events() {
    if !crate::sdk::is_initialized() {
        return;
    }

    let handle = tapsdk_pc_sys::TapOnlineGame();
    if handle.is_null() {
        return;
    }

    let mut left_events = 0u32;
    loop {
        tapsdk_pc_sys::TapOnlineGame_RunCallbacks(
            handle,
            Some(global_online_game_callback),
            10,
            &mut left_events,
        );

        if left_events == 0 {
            break;
        }
    }
}

/// Global callback handler called by the SDK
///
/// # Safety
/// This function is called from C code with raw pointers
unsafe extern "C" fn global_callback(event_id: u32, data: *mut std::ffi::c_void) {
    let event = parse_event(event_id, data);

    if let Ok(mut queue) = EVENT_QUEUE.lock() {
        queue.push_back(event);
    }
}

/// Global callback handler called by the online game SDK
///
/// # Safety
/// This function is called from C code with raw pointers.
unsafe extern "C" fn global_online_game_callback(event: *const tapsdk_pc_sys::TapOnlineGameEvent) {
    let event = parse_online_game_event(event);

    if let Ok(mut queue) = EVENT_QUEUE.lock() {
        queue.push_back(event);
    }
}

/// Parse an event from raw SDK data
unsafe fn parse_event(event_id: u32, data: *mut std::ffi::c_void) -> TapEvent {
    match event_id {
        event_id::SYSTEM_STATE_CHANGED => {
            if data.is_null() {
                return TapEvent::Unknown { event_id };
            }
            let notification = &*(data as *const tapsdk_pc_sys::TapSystemStateNotification);
            TapEvent::SystemStateChanged(SystemStateChangedData {
                state: SystemState::from(notification.state),
            })
        }

        event_id::AUTHORIZE_FINISHED => {
            if data.is_null() {
                return TapEvent::Unknown { event_id };
            }
            let response = &*(data as *const tapsdk_pc_sys::AuthorizeFinishedResponse);

            let error = {
                let error_str = CStr::from_ptr(response.error.as_ptr())
                    .to_string_lossy()
                    .into_owned();
                if error_str.is_empty() {
                    None
                } else {
                    Some(error_str)
                }
            };

            let token = if !response.is_cancel && error.is_none() {
                Some(AuthToken {
                    token_type: CStr::from_ptr(response.token_type.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                    kid: CStr::from_ptr(response.kid.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                    mac_key: CStr::from_ptr(response.mac_key.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                    mac_algorithm: CStr::from_ptr(response.mac_algorithm.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                    scope: CStr::from_ptr(response.scope.as_ptr())
                        .to_string_lossy()
                        .into_owned(),
                })
            } else {
                None
            };

            TapEvent::AuthorizeFinished(AuthorizeFinishedData {
                is_cancel: response.is_cancel,
                error,
                token,
            })
        }

        event_id::GAME_PLAYABLE_STATUS_CHANGED => {
            if data.is_null() {
                return TapEvent::Unknown { event_id };
            }
            let response = &*(data as *const tapsdk_pc_sys::GamePlayableStatusChangedResponse);
            TapEvent::GamePlayableStatusChanged(GamePlayableStatusChangedData {
                is_playable: response.is_playable,
            })
        }

        event_id::DLC_PLAYABLE_STATUS_CHANGED => {
            if data.is_null() {
                return TapEvent::Unknown { event_id };
            }
            let response = &*(data as *const tapsdk_pc_sys::DLCPlayableStatusChangedResponse);
            TapEvent::DlcPlayableStatusChanged(DlcPlayableStatusChangedData {
                dlc_id: CStr::from_ptr(response.dlc_id.as_ptr())
                    .to_string_lossy()
                    .into_owned(),
                is_playable: response.is_playable,
            })
        }

        event_id::CLOUD_SAVE_LIST => {
            if data.is_null() {
                return TapEvent::Unknown { event_id };
            }
            let response = &*(data as *const tapsdk_pc_sys::TapCloudSaveListResponse);

            let error = parse_sdk_error(response.error);

            let saves = if response.saves.is_null() || response.save_count <= 0 {
                Vec::new()
            } else {
                let slice =
                    std::slice::from_raw_parts(response.saves, response.save_count as usize);
                slice.iter().map(|s| parse_cloud_save_info(s)).collect()
            };

            TapEvent::CloudSaveList(CloudSaveListData {
                request_id: response.request_id,
                error,
                saves,
            })
        }

        event_id::CLOUD_SAVE_CREATE | event_id::CLOUD_SAVE_UPDATE => {
            if data.is_null() {
                return TapEvent::Unknown { event_id };
            }
            let response = &*(data as *const tapsdk_pc_sys::TapCloudSaveCreateResponse);

            let error = parse_sdk_error(response.error);

            let save = if response.save.is_null() {
                None
            } else {
                Some(parse_cloud_save_info(&*response.save))
            };

            let event_data = CloudSaveCreateData {
                request_id: response.request_id,
                error,
                save,
            };

            if event_id == event_id::CLOUD_SAVE_CREATE {
                TapEvent::CloudSaveCreate(event_data)
            } else {
                TapEvent::CloudSaveUpdate(event_data)
            }
        }

        event_id::CLOUD_SAVE_DELETE => {
            if data.is_null() {
                return TapEvent::Unknown { event_id };
            }
            let response = &*(data as *const tapsdk_pc_sys::TapCloudSaveDeleteResponse);

            let error = parse_sdk_error(response.error);

            let uuid = if response.uuid.is_null() {
                String::new()
            } else {
                CStr::from_ptr(response.uuid).to_string_lossy().into_owned()
            };

            TapEvent::CloudSaveDelete(CloudSaveDeleteData {
                request_id: response.request_id,
                error,
                uuid,
            })
        }

        event_id::CLOUD_SAVE_GET_DATA | event_id::CLOUD_SAVE_GET_COVER => {
            if data.is_null() {
                return TapEvent::Unknown { event_id };
            }
            let response = &*(data as *const tapsdk_pc_sys::TapCloudSaveGetFileResponse);

            let error = parse_sdk_error(response.error);

            let file_data = if response.data.is_null() || response.size == 0 {
                Vec::new()
            } else {
                let slice =
                    std::slice::from_raw_parts(response.data as *const u8, response.size as usize);
                slice.to_vec()
            };

            let event_data = CloudSaveGetFileData {
                request_id: response.request_id,
                error,
                data: file_data,
            };

            if event_id == event_id::CLOUD_SAVE_GET_DATA {
                TapEvent::CloudSaveGetData(event_data)
            } else {
                TapEvent::CloudSaveGetCover(event_data)
            }
        }

        event_id::ACHIEVEMENT_UNLOCK | event_id::ACHIEVEMENT_INCREMENT => {
            if data.is_null() {
                return TapEvent::Unknown { event_id };
            }

            let response = &*(data as *const tapsdk_pc_sys::TapAchievementUnlockResponse);
            let event_data = AchievementData {
                request_id: response.request_id,
                error: parse_sdk_error(response.error),
                achievement: parse_achievement_info(response.achievement),
                platinum_achievement: parse_achievement_info(response.platinum_achievement),
            };

            if event_id == event_id::ACHIEVEMENT_UNLOCK {
                TapEvent::AchievementUnlock(event_data)
            } else {
                TapEvent::AchievementIncrement(event_data)
            }
        }

        event_id::COMPLIANCE_ENSURE_REAL_NAME => {
            if data.is_null() {
                return TapEvent::Unknown { event_id };
            }

            let response = &*(data as *const tapsdk_pc_sys::TapComplianceEnsureRealNameResponse);
            TapEvent::ComplianceEnsureRealName(ComplianceEnsureRealNameData {
                request_id: response.request_id,
                error: parse_sdk_error(response.error),
                status: response.status,
            })
        }

        event_id::COMPLIANCE_ACTIONS_EVENT => {
            if data.is_null() {
                return TapEvent::Unknown { event_id };
            }

            let response = &*(data as *const tapsdk_pc_sys::TapComplianceActionsEvent);
            let actions = if response.actions.is_null() || response.count == 0 {
                Vec::new()
            } else {
                std::slice::from_raw_parts(response.actions, response.count as usize)
                    .iter()
                    .map(|action| parse_compliance_action(action))
                    .collect()
            };

            TapEvent::ComplianceActionsEvent(ComplianceActionsData { actions })
        }

        _ => TapEvent::Unknown { event_id },
    }
}

/// Parse SDK error from raw pointer
unsafe fn parse_sdk_error(error: *const tapsdk_pc_sys::TapSDK_Error) -> Option<(i64, String)> {
    if error.is_null() {
        return None;
    }

    let err = &*error;
    let message = if err.message.is_null() {
        String::new()
    } else {
        CStr::from_ptr(err.message).to_string_lossy().into_owned()
    };

    Some((err.code, message))
}

/// Parse cloud save info from raw struct
unsafe fn parse_cloud_save_info(info: &tapsdk_pc_sys::TapCloudSaveInfo) -> CloudSaveInfo {
    CloudSaveInfo {
        uuid: ptr_to_string(info.uuid),
        file_id: ptr_to_string(info.file_id),
        name: ptr_to_string(info.name),
        save_size: info.save_size,
        cover_size: info.cover_size,
        summary: ptr_to_optional_string(info.summary),
        extra: ptr_to_optional_string(info.extra),
        playtime: info.playtime,
        created_time: info.created_time,
        modified_time: info.modified_time,
    }
}

/// Parse achievement info from raw pointer
unsafe fn parse_achievement_info(
    info: *const tapsdk_pc_sys::TapAchievementInfo,
) -> Option<AchievementInfo> {
    if info.is_null() {
        return None;
    }

    let info = &*info;
    Some(AchievementInfo {
        id: ptr_to_string(info.id),
        name: ptr_to_string(info.name),
        current_steps: info.current_steps,
        newly_unlocked: info.newly_unlocked,
    })
}

unsafe fn parse_compliance_action(action: &tapsdk_pc_sys::TapComplianceAction) -> ComplianceAction {
    ComplianceAction {
        action_type: action.action_type,
        title: ptr_to_string(action.title),
        description: ptr_to_string(action.description),
        display_duration_seconds: action.display_duration_seconds,
    }
}

unsafe fn parse_online_game_event(event: *const tapsdk_pc_sys::TapOnlineGameEvent) -> TapEvent {
    if event.is_null() {
        return TapEvent::Unknown {
            event_id: event_id::ONLINE_GAME_EVENT,
        };
    }

    let event = &*event;
    TapEvent::OnlineGame(OnlineGameEventData {
        request_id: event.request_id,
        error: parse_sdk_error(event.error),
        online_game_event_id: event.event_id,
        data: parse_online_game_payload(event.event_id, event.event_data),
    })
}

unsafe fn parse_online_game_payload(
    event_id: u32,
    data: *const std::ffi::c_void,
) -> OnlineGameEventPayload {
    if data.is_null() {
        return OnlineGameEventPayload::Empty;
    }

    match event_id {
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_ConnectResponse.0 as u32 => {
            let response = &*(data as *const tapsdk_pc_sys::TapOnlineGameConnectResponse);
            OnlineGameEventPayload::Connect {
                player_id: ptr_to_string(response.player_id),
            }
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_CreateRoomResponse.0 as u32 => {
            let response = &*(data as *const tapsdk_pc_sys::TapOnlineGameCreateRoomResponse);
            parse_online_game_room_info(response.room_info)
                .map(|room_info| OnlineGameEventPayload::Room { room_info })
                .unwrap_or(OnlineGameEventPayload::Empty)
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_MatchRoomResponse.0 as u32 => {
            let response = &*(data as *const tapsdk_pc_sys::TapOnlineGameMatchRoomResponse);
            parse_online_game_room_info(response.room_info)
                .map(|room_info| OnlineGameEventPayload::Room { room_info })
                .unwrap_or(OnlineGameEventPayload::Empty)
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_GetRoomListResponse.0 as u32 => {
            let response = &*(data as *const tapsdk_pc_sys::TapOnlineGameGetRoomListResponse);
            let rooms = if response.rooms.is_null() || response.room_count == 0 {
                Vec::new()
            } else {
                std::slice::from_raw_parts(response.rooms, response.room_count as usize)
                    .iter()
                    .map(|room| parse_online_game_room_basic_info(room))
                    .collect()
            };

            OnlineGameEventPayload::RoomList {
                rooms,
                offset: response.offset,
                has_more: response.has_more,
            }
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_JoinRoomResponse.0 as u32 => {
            let response = &*(data as *const tapsdk_pc_sys::TapOnlineGameJoinRoomResponse);
            parse_online_game_room_info(response.room_info)
                .map(|room_info| OnlineGameEventPayload::Room { room_info })
                .unwrap_or(OnlineGameEventPayload::Empty)
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_PlayerOfflineNotification.0 as u32 => {
            let notification =
                &*(data as *const tapsdk_pc_sys::TapOnlineGamePlayerOfflineNotification);
            OnlineGameEventPayload::PlayerOffline {
                room_id: ptr_to_string(notification.room_id),
                room_owner_id: ptr_to_string(notification.room_owner_id),
                player_id: ptr_to_string(notification.player_id),
            }
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_EnterRoomNotification.0 as u32 => {
            let notification =
                &*(data as *const tapsdk_pc_sys::TapOnlineGamePlayerEnterRoomNotification);
            parse_online_game_player_info(notification.player_info)
                .map(|player_info| OnlineGameEventPayload::PlayerEnterRoom {
                    room_id: ptr_to_string(notification.room_id),
                    player_info,
                })
                .unwrap_or(OnlineGameEventPayload::Empty)
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_LeaveRoomNotification.0 as u32 => {
            let notification =
                &*(data as *const tapsdk_pc_sys::TapOnlineGamePlayerLeaveRoomNotification);
            OnlineGameEventPayload::PlayerLeaveRoom {
                room_id: ptr_to_string(notification.room_id),
                room_owner_id: ptr_to_string(notification.room_owner_id),
                player_id: ptr_to_string(notification.player_id),
            }
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_PlayerCustomStatusNotification.0 as u32 => {
            let notification =
                &*(data as *const tapsdk_pc_sys::TapOnlineGamePlayerCustomStatusNotification);
            OnlineGameEventPayload::PlayerCustomStatus {
                player_id: ptr_to_string(notification.player_id),
                status: notification.status,
            }
        }
        id if id
            == tapsdk_pc_sys::TapOnlineGameEventID_PlayerCustomPropertiesNotification.0 as u32 =>
        {
            let notification =
                &*(data as *const tapsdk_pc_sys::TapOnlineGamePlayerCustomPropertiesNotification);
            OnlineGameEventPayload::PlayerCustomProperties {
                player_id: ptr_to_string(notification.player_id),
                properties: ptr_to_string(notification.properties),
            }
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_RoomPropertiesNotification.0 as u32 => {
            let notification =
                &*(data as *const tapsdk_pc_sys::TapOnlineGameRoomPropertiesNotification);
            OnlineGameEventPayload::RoomProperties {
                id: ptr_to_string(notification.id),
                name: ptr_to_string(notification.name),
                custom_properties: ptr_to_string(notification.custom_properties),
            }
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_CustomMessageNotification.0 as u32 => {
            let notification =
                &*(data as *const tapsdk_pc_sys::TapOnlineGameCustomMessageNotification);
            OnlineGameEventPayload::CustomMessage {
                player_id: ptr_to_string(notification.player_id),
                msg: ptr_to_string(notification.msg),
            }
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_RoomPlayerKickedNotification.0 as u32 => {
            let notification =
                &*(data as *const tapsdk_pc_sys::TapOnlineGameRoomPlayerKickedNotification);
            OnlineGameEventPayload::RoomPlayerKicked {
                room_id: ptr_to_string(notification.room_id),
                player_id: ptr_to_string(notification.player_id),
            }
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_FrameSyncStartNotification.0 as u32 => {
            let notification =
                &*(data as *const tapsdk_pc_sys::TapOnlineGameFrameSyncStartNotification);
            parse_online_game_room_info(notification.room_info)
                .map(|room_info| OnlineGameEventPayload::FrameSyncStart {
                    room_info,
                    frame_sync_id: notification.frame_sync_id,
                    seed: notification.seed,
                    server_tms: notification.server_tms,
                })
                .unwrap_or(OnlineGameEventPayload::Empty)
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_FrameNotification.0 as u32 => {
            let frame = &*(data as *const tapsdk_pc_sys::TapOnlineGameFrame);
            let inputs = if frame.inputs.is_null() || frame.input_count == 0 {
                Vec::new()
            } else {
                std::slice::from_raw_parts(frame.inputs, frame.input_count as usize)
                    .iter()
                    .map(|input| parse_online_game_frame_input(input))
                    .collect()
            };

            OnlineGameEventPayload::Frame {
                id: frame.id,
                inputs,
            }
        }
        id if id == tapsdk_pc_sys::TapOnlineGameEventID_FrameSyncStopNotification.0 as u32 => {
            let notification =
                &*(data as *const tapsdk_pc_sys::TapOnlineGameFrameSyncStopNotification);
            OnlineGameEventPayload::FrameSyncStop {
                room_id: ptr_to_string(notification.room_id),
                frame_sync_id: notification.frame_sync_id,
                reason: notification.reason,
            }
        }
        _ => OnlineGameEventPayload::Unknown,
    }
}

unsafe fn parse_online_game_player_info(
    player: *const tapsdk_pc_sys::TapOnlineGamePlayerInfo,
) -> Option<OnlineGamePlayerInfo> {
    if player.is_null() {
        return None;
    }

    let player = &*player;
    Some(OnlineGamePlayerInfo {
        id: ptr_to_string(player.id),
        status: player.status,
        custom_status: player.custom_status,
        custom_properties: ptr_to_string(player.custom_properties),
    })
}

unsafe fn parse_online_game_room_info(
    room: *const tapsdk_pc_sys::TapOnlineGameRoomInfo,
) -> Option<OnlineGameRoomInfo> {
    if room.is_null() {
        return None;
    }

    let room = &*room;
    let players = if room.players.is_null() || room.player_count == 0 {
        Vec::new()
    } else {
        std::slice::from_raw_parts(room.players, room.player_count as usize)
            .iter()
            .map(|player| OnlineGamePlayerInfo {
                id: ptr_to_string(player.id),
                status: player.status,
                custom_status: player.custom_status,
                custom_properties: ptr_to_string(player.custom_properties),
            })
            .collect()
    };

    Some(OnlineGameRoomInfo {
        id: ptr_to_string(room.id),
        name: ptr_to_string(room.name),
        room_type: ptr_to_string(room.room_type),
        owner_id: ptr_to_string(room.owner_id),
        status: room.status,
        custom_properties: ptr_to_string(room.custom_properties),
        max_player_count: room.max_player_count,
        player_count: room.player_count,
        players,
        create_time: room.create_time,
    })
}

unsafe fn parse_online_game_room_basic_info(
    room: &tapsdk_pc_sys::TapOnlineGameRoomBasicInfo,
) -> OnlineGameRoomBasicInfo {
    OnlineGameRoomBasicInfo {
        id: ptr_to_string(room.id),
        name: ptr_to_string(room.name),
        room_type: ptr_to_string(room.room_type),
        status: room.status,
        custom_properties: ptr_to_string(room.custom_properties),
        max_player_count: room.max_player_count,
        player_count: room.player_count,
        create_time: room.create_time,
    }
}

unsafe fn parse_online_game_frame_input(
    input: &tapsdk_pc_sys::TapOnlineGameFrameInput,
) -> OnlineGameFrameInput {
    OnlineGameFrameInput {
        player_id: ptr_to_string(input.player_id),
        data: ptr_to_string(input.data),
        server_tms: input.server_tms,
    }
}

/// Convert a fixed C char array to a Rust String.
///
/// # Safety
/// The char array must be valid memory and contain a NUL terminator or UTF-8 bytes.
pub unsafe fn fixed_c_char_array_to_string<const N: usize>(
    chars: &[std::os::raw::c_char; N],
) -> String {
    CStr::from_ptr(chars.as_ptr())
        .to_string_lossy()
        .into_owned()
}

/// Convert a C string pointer to a Rust String
unsafe fn ptr_to_string(ptr: *const std::os::raw::c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

/// Convert a C string pointer to an optional Rust String
unsafe fn ptr_to_optional_string(ptr: *const std::os::raw::c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
}
