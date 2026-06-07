//! Online game functionality

use std::ffi::CString;

use crate::error::{Result, SdkResult, TapSdkError};
use crate::sdk::is_initialized;

/// Online game custom message receiver type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageReceiverType {
    Room = 0,
    Players = 1,
}

/// Online game API handle.
///
/// Get an instance via `OnlineGame::get()` after initializing the SDK.
pub struct OnlineGame {
    handle: *mut tapsdk_pc_sys::ITapOnlineGame,
}

// The SDK exposes online game through a process-wide singleton.
unsafe impl Send for OnlineGame {}
unsafe impl Sync for OnlineGame {}

impl OnlineGame {
    /// Get the online game singleton instance.
    pub fn get() -> Option<Self> {
        if !is_initialized() {
            return None;
        }

        let handle = unsafe { tapsdk_pc_sys::TapOnlineGame() };

        if handle.is_null() {
            None
        } else {
            Some(OnlineGame { handle })
        }
    }

    /// Connect to the online game service.
    pub fn connect(&self, request_id: i64) -> Result<()> {
        let result = unsafe { tapsdk_pc_sys::TapOnlineGame_AsyncConnect(self.handle, request_id) };

        check_sdk_result(result)
    }

    /// Disconnect from the online game service.
    pub fn disconnect(&self, request_id: i64) -> Result<()> {
        let result =
            unsafe { tapsdk_pc_sys::TapOnlineGame_AsyncDisconnect(self.handle, request_id) };

        check_sdk_result(result)
    }

    /// Create a room.
    pub fn create_room(&self, request_id: i64, request: &CreateRoomRequest) -> Result<()> {
        let raw = RawCreateOrMatchRoomRequest::new(&request.room, &request.player)?;
        let raw_request = raw.as_create_room_request();

        let result = unsafe {
            tapsdk_pc_sys::TapOnlineGame_AsyncCreateRoom(self.handle, request_id, &raw_request)
        };

        check_sdk_result(result)
    }

    /// Match or create a room.
    pub fn match_room(&self, request_id: i64, request: &MatchRoomRequest) -> Result<()> {
        let raw = RawCreateOrMatchRoomRequest::new(&request.room, &request.player)?;
        let raw_request = raw.as_match_room_request();

        let result = unsafe {
            tapsdk_pc_sys::TapOnlineGame_AsyncMatchRoom(self.handle, request_id, &raw_request)
        };

        check_sdk_result(result)
    }

    /// Get joinable room list.
    pub fn get_room_list(&self, request_id: i64, request: &GetRoomListRequest) -> Result<()> {
        let room_type = request
            .room_type
            .as_ref()
            .map(|room_type| CString::new(room_type.as_str()))
            .transpose()?;

        let raw_request = tapsdk_pc_sys::TapOnlineGameGetRoomListRequest {
            room_type: room_type
                .as_ref()
                .map(|room_type| room_type.as_ptr())
                .unwrap_or(std::ptr::null()),
            offset: request.offset,
            limit: request.limit,
        };

        let result = unsafe {
            tapsdk_pc_sys::TapOnlineGame_AsyncGetRoomList(self.handle, request_id, &raw_request)
        };

        check_sdk_result(result)
    }

    /// Join a room.
    pub fn join_room(&self, request_id: i64, request: &JoinRoomRequest) -> Result<()> {
        let room_id = CString::new(request.room_id.as_str())?;
        let raw_player = RawPlayerConfig::new(&request.player)?;

        let raw_request = tapsdk_pc_sys::TapOnlineGameJoinRoomRequest {
            room_id: room_id.as_ptr(),
            player_cfg: raw_player.as_raw(),
        };

        let result = unsafe {
            tapsdk_pc_sys::TapOnlineGame_AsyncJoinRoom(self.handle, request_id, &raw_request)
        };

        check_sdk_result(result)
    }

    /// Leave the current room.
    pub fn leave_room(&self, request_id: i64) -> Result<()> {
        let result =
            unsafe { tapsdk_pc_sys::TapOnlineGame_AsyncLeaveRoom(self.handle, request_id) };

        check_sdk_result(result)
    }

    /// Update the current player's custom status.
    pub fn update_player_custom_status(&self, request_id: i64, status: i32) -> Result<()> {
        let result = unsafe {
            tapsdk_pc_sys::TapOnlineGame_AsyncUpdatePlayerCustomStatus(
                self.handle,
                request_id,
                status,
            )
        };

        check_sdk_result(result)
    }

    /// Update the current player's custom properties.
    pub fn update_player_custom_properties(&self, request_id: i64, properties: &str) -> Result<()> {
        let properties = CString::new(properties)?;
        let result = unsafe {
            tapsdk_pc_sys::TapOnlineGame_AsyncUpdatePlayerCustomProperties(
                self.handle,
                request_id,
                properties.as_ptr(),
            )
        };

        check_sdk_result(result)
    }

    /// Update the current room's properties.
    pub fn update_room_properties(
        &self,
        request_id: i64,
        request: &UpdateRoomPropertiesRequest,
    ) -> Result<()> {
        let name = request
            .name
            .as_ref()
            .map(|name| CString::new(name.as_str()))
            .transpose()?;
        let custom_properties = request
            .custom_properties
            .as_ref()
            .map(|properties| CString::new(properties.as_str()))
            .transpose()?;

        let raw_request = tapsdk_pc_sys::TapOnlineGameUpdateRoomPropertiesRequest {
            name: name
                .as_ref()
                .map(|name| name.as_ptr())
                .unwrap_or(std::ptr::null()),
            custom_properties: custom_properties
                .as_ref()
                .map(|properties| properties.as_ptr())
                .unwrap_or(std::ptr::null()),
        };

        let result = unsafe {
            tapsdk_pc_sys::TapOnlineGame_AsyncUpdateRoomProperties(
                self.handle,
                request_id,
                &raw_request,
            )
        };

        check_sdk_result(result)
    }

    /// Send a custom message.
    pub fn send_custom_message(
        &self,
        request_id: i64,
        request: &SendCustomMessageRequest,
    ) -> Result<()> {
        let msg = CString::new(request.msg.as_str())?;
        let receivers = request
            .receivers
            .iter()
            .map(|receiver| CString::new(receiver.as_str()))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let mut receiver_ptrs = receivers
            .iter()
            .map(|receiver| receiver.as_ptr())
            .collect::<Vec<_>>();

        let raw_request = tapsdk_pc_sys::TapOnlineGameSendCustomMessageRequest {
            msg: msg.as_ptr(),
            receiver_type: request.receiver_type as u32,
            receiver_count: receiver_ptrs.len() as u32,
            receivers: receiver_ptrs.as_mut_ptr(),
        };

        let result = unsafe {
            tapsdk_pc_sys::TapOnlineGame_AsyncSendCustomMessage(
                self.handle,
                request_id,
                &raw_request,
            )
        };

        check_sdk_result(result)
    }

    /// Kick a player from the current room.
    pub fn kick_room_player(&self, request_id: i64, player_id: &str) -> Result<()> {
        let player_id = CString::new(player_id)?;
        let result = unsafe {
            tapsdk_pc_sys::TapOnlineGame_AsyncKickRoomPlayer(
                self.handle,
                request_id,
                player_id.as_ptr(),
            )
        };

        check_sdk_result(result)
    }

    /// Start frame synchronization.
    pub fn start_frame_sync(&self, request_id: i64) -> Result<()> {
        let result =
            unsafe { tapsdk_pc_sys::TapOnlineGame_AsyncStartFrameSync(self.handle, request_id) };

        check_sdk_result(result)
    }

    /// Send frame input data.
    pub fn send_frame_input(&self, request_id: i64, data: &str) -> Result<()> {
        let data = CString::new(data)?;
        let raw_request = tapsdk_pc_sys::TapOnlineGameSendFrameInputRequest {
            data: data.as_ptr(),
        };

        let result = unsafe {
            tapsdk_pc_sys::TapOnlineGame_AsyncSendFrameInput(self.handle, request_id, &raw_request)
        };

        check_sdk_result(result)
    }

    /// Stop frame synchronization.
    pub fn stop_frame_sync(&self, request_id: i64) -> Result<()> {
        let result =
            unsafe { tapsdk_pc_sys::TapOnlineGame_AsyncStopFrameSync(self.handle, request_id) };

        check_sdk_result(result)
    }
}

/// Online game room match parameter.
#[derive(Debug, Clone)]
pub struct MatchParam {
    pub key: String,
    pub value: String,
}

/// Online game room configuration.
#[derive(Debug, Clone)]
pub struct RoomConfig {
    pub max_player_count: u32,
    pub room_type: String,
    pub match_params: Vec<MatchParam>,
    pub name: Option<String>,
    pub custom_properties: Option<String>,
}

/// Online game player configuration.
#[derive(Debug, Clone)]
pub struct PlayerConfig {
    pub custom_status: i32,
    pub custom_properties: Option<String>,
}

/// Create room request.
#[derive(Debug, Clone)]
pub struct CreateRoomRequest {
    pub room: RoomConfig,
    pub player: PlayerConfig,
}

/// Match room request.
#[derive(Debug, Clone)]
pub struct MatchRoomRequest {
    pub room: RoomConfig,
    pub player: PlayerConfig,
}

/// Get room list request.
#[derive(Debug, Clone)]
pub struct GetRoomListRequest {
    pub room_type: Option<String>,
    pub offset: u32,
    pub limit: u32,
}

/// Join room request.
#[derive(Debug, Clone)]
pub struct JoinRoomRequest {
    pub room_id: String,
    pub player: PlayerConfig,
}

/// Update room properties request.
#[derive(Debug, Clone)]
pub struct UpdateRoomPropertiesRequest {
    pub name: Option<String>,
    pub custom_properties: Option<String>,
}

/// Send custom message request.
#[derive(Debug, Clone)]
pub struct SendCustomMessageRequest {
    pub msg: String,
    pub receiver_type: MessageReceiverType,
    pub receivers: Vec<String>,
}

struct RawCreateOrMatchRoomRequest {
    room: RawRoomConfig,
    player: RawPlayerConfig,
}

impl RawCreateOrMatchRoomRequest {
    fn new(room: &RoomConfig, player: &PlayerConfig) -> Result<Self> {
        Ok(Self {
            room: RawRoomConfig::new(room)?,
            player: RawPlayerConfig::new(player)?,
        })
    }

    fn as_create_room_request(&self) -> tapsdk_pc_sys::TapOnlineGameCreateRoomRequest {
        tapsdk_pc_sys::TapOnlineGameCreateRoomRequest {
            room_cfg: self.room.as_raw(),
            player_cfg: self.player.as_raw(),
        }
    }

    fn as_match_room_request(&self) -> tapsdk_pc_sys::TapOnlineGameMatchRoomRequest {
        tapsdk_pc_sys::TapOnlineGameMatchRoomRequest {
            room_cfg: self.room.as_raw(),
            player_cfg: self.player.as_raw(),
        }
    }
}

struct RawRoomConfig {
    room_type: CString,
    match_param_keys: Vec<CString>,
    match_param_values: Vec<CString>,
    match_params: Vec<tapsdk_pc_sys::TapOnlineGameMatchParam>,
    name: Option<CString>,
    custom_properties: Option<CString>,
    max_player_count: u32,
}

impl RawRoomConfig {
    fn new(config: &RoomConfig) -> Result<Self> {
        let room_type = CString::new(config.room_type.as_str())?;
        let match_param_keys = config
            .match_params
            .iter()
            .map(|param| CString::new(param.key.as_str()))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let match_param_values = config
            .match_params
            .iter()
            .map(|param| CString::new(param.value.as_str()))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let match_params = match_param_keys
            .iter()
            .zip(match_param_values.iter())
            .map(|(key, value)| tapsdk_pc_sys::TapOnlineGameMatchParam {
                key: key.as_ptr(),
                value: value.as_ptr(),
            })
            .collect::<Vec<_>>();
        let name = config
            .name
            .as_ref()
            .map(|name| CString::new(name.as_str()))
            .transpose()?;
        let custom_properties = config
            .custom_properties
            .as_ref()
            .map(|properties| CString::new(properties.as_str()))
            .transpose()?;

        Ok(Self {
            room_type,
            match_param_keys,
            match_param_values,
            match_params,
            name,
            custom_properties,
            max_player_count: config.max_player_count,
        })
    }

    fn as_raw(&self) -> tapsdk_pc_sys::TapOnlineGameRoomConfig {
        // Keep the CString vectors alive while this raw struct is in use.
        let _ = (&self.match_param_keys, &self.match_param_values);

        let mut config: tapsdk_pc_sys::TapOnlineGameRoomConfig = unsafe { std::mem::zeroed() };
        config.max_player_count = self.max_player_count;
        config.room_type = self.room_type.as_ptr();
        config.match_param_count = self.match_params.len() as u32;
        config.match_params = self.match_params.as_ptr();
        config.name = self
            .name
            .as_ref()
            .map(|name| name.as_ptr())
            .unwrap_or(std::ptr::null());
        config.custom_properties = self
            .custom_properties
            .as_ref()
            .map(|properties| properties.as_ptr())
            .unwrap_or(std::ptr::null());
        config
    }
}

struct RawPlayerConfig {
    custom_properties: Option<CString>,
    custom_status: i32,
}

impl RawPlayerConfig {
    fn new(config: &PlayerConfig) -> Result<Self> {
        Ok(Self {
            custom_properties: config
                .custom_properties
                .as_ref()
                .map(|properties| CString::new(properties.as_str()))
                .transpose()?,
            custom_status: config.custom_status,
        })
    }

    fn as_raw(&self) -> tapsdk_pc_sys::TapOnlineGamePlayerConfig {
        let mut config: tapsdk_pc_sys::TapOnlineGamePlayerConfig = unsafe { std::mem::zeroed() };
        config.custom_status = self.custom_status;
        config.custom_properties = self
            .custom_properties
            .as_ref()
            .map(|properties| properties.as_ptr())
            .unwrap_or(std::ptr::null());
        config
    }
}

fn check_sdk_result(result: u32) -> Result<()> {
    match SdkResult::from(result) {
        SdkResult::Ok => Ok(()),
        result => Err(TapSdkError::SdkRequestFailed(result)),
    }
}
