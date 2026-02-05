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
    }

    // Then drain the event queue
    let mut queue = EVENT_QUEUE.lock().unwrap();
    queue.drain(..).collect()
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
