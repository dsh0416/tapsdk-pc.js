// Stub bindings for non-Windows platforms.
// TapTap PC SDK currently only ships a Windows implementation.

use std::os::raw::{c_char, c_void};

pub type TapCloudSaveHandle = *mut c_void;
pub type ITapCloudSave = c_void;
pub type ITapAchievement = c_void;
pub type ITapCompliance = c_void;
pub type ITapOnlineGame = c_void;
pub type TapSDK_Result = u32;

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TapOnlineGameEventID(pub u32);

pub const TapOnlineGameEventID_Unknown: TapOnlineGameEventID = TapOnlineGameEventID(0);
pub const TapOnlineGameEventID_ConnectResponse: TapOnlineGameEventID = TapOnlineGameEventID(1);
pub const TapOnlineGameEventID_DisconnectResponse: TapOnlineGameEventID = TapOnlineGameEventID(2);
pub const TapOnlineGameEventID_CreateRoomResponse: TapOnlineGameEventID = TapOnlineGameEventID(3);
pub const TapOnlineGameEventID_MatchRoomResponse: TapOnlineGameEventID = TapOnlineGameEventID(4);
pub const TapOnlineGameEventID_GetRoomListResponse: TapOnlineGameEventID = TapOnlineGameEventID(5);
pub const TapOnlineGameEventID_JoinRoomResponse: TapOnlineGameEventID = TapOnlineGameEventID(6);
pub const TapOnlineGameEventID_LeaveRoomResponse: TapOnlineGameEventID = TapOnlineGameEventID(7);
pub const TapOnlineGameEventID_UpdatePlayerCustomStatusResponse: TapOnlineGameEventID =
    TapOnlineGameEventID(8);
pub const TapOnlineGameEventID_UpdatePlayerCustomPropertiesResponse: TapOnlineGameEventID =
    TapOnlineGameEventID(9);
pub const TapOnlineGameEventID_UpdateRoomPropertiesResponse: TapOnlineGameEventID =
    TapOnlineGameEventID(10);
pub const TapOnlineGameEventID_SendCustomMessageResponse: TapOnlineGameEventID =
    TapOnlineGameEventID(11);
pub const TapOnlineGameEventID_KickRoomPlayerResponse: TapOnlineGameEventID =
    TapOnlineGameEventID(12);
pub const TapOnlineGameEventID_StartFrameSyncResponse: TapOnlineGameEventID =
    TapOnlineGameEventID(13);
pub const TapOnlineGameEventID_SendFrameInputResponse: TapOnlineGameEventID =
    TapOnlineGameEventID(14);
pub const TapOnlineGameEventID_StopFrameSyncResponse: TapOnlineGameEventID =
    TapOnlineGameEventID(15);
pub const TapOnlineGameEventID_ServiceErrorNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1000);
pub const TapOnlineGameEventID_DisconnectNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1001);
pub const TapOnlineGameEventID_PlayerOfflineNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1002);
pub const TapOnlineGameEventID_EnterRoomNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1003);
pub const TapOnlineGameEventID_LeaveRoomNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1004);
pub const TapOnlineGameEventID_PlayerCustomStatusNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1005);
pub const TapOnlineGameEventID_PlayerCustomPropertiesNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1006);
pub const TapOnlineGameEventID_RoomPropertiesNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1007);
pub const TapOnlineGameEventID_CustomMessageNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1008);
pub const TapOnlineGameEventID_RoomPlayerKickedNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1009);
pub const TapOnlineGameEventID_FrameSyncStartNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1010);
pub const TapOnlineGameEventID_FrameNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1011);
pub const TapOnlineGameEventID_FrameSyncStopNotification: TapOnlineGameEventID =
    TapOnlineGameEventID(1012);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapSDK_Error {
    pub code: i64,
    pub message: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapSystemStateNotification {
    pub state: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AuthorizeFinishedResponse {
    pub is_cancel: bool,
    pub error: [c_char; 256],
    pub token_type: [c_char; 64],
    pub kid: [c_char; 256],
    pub mac_key: [c_char; 256],
    pub mac_algorithm: [c_char; 64],
    pub scope: [c_char; 256],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GamePlayableStatusChangedResponse {
    pub is_playable: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DLCPlayableStatusChangedResponse {
    pub dlc_id: [c_char; 256],
    pub is_playable: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveInfo {
    pub uuid: *const c_char,
    pub file_id: *const c_char,
    pub name: *const c_char,
    pub save_size: u32,
    pub cover_size: u32,
    pub summary: *const c_char,
    pub extra: *const c_char,
    pub playtime: u32,
    pub created_time: u32,
    pub modified_time: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveListResponse {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub saves: *const TapCloudSaveInfo,
    pub save_count: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveCreateResponse {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub save: *const TapCloudSaveInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveDeleteResponse {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub uuid: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveGetFileResponse {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub data: *const c_void,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveCreateRequest {
    pub name: *const c_char,
    pub summary: *const c_char,
    pub extra: *const c_char,
    pub playtime: u32,
    pub data_file_path: *const c_char,
    pub cover_file_path: *const c_char,
    pub __bindgen_padding_0: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveUpdateRequest {
    pub uuid: *const c_char,
    pub name: *const c_char,
    pub summary: *const c_char,
    pub extra: *const c_char,
    pub playtime: u32,
    pub data_file_path: *const c_char,
    pub cover_file_path: *const c_char,
    pub __bindgen_padding_0: [u8; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapCloudSaveGetFileRequest {
    pub uuid: *const c_char,
    pub file_id: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapAchievementInfo {
    pub id: *const c_char,
    pub name: *const c_char,
    pub current_steps: u64,
    pub newly_unlocked: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapAchievementUnlockRequest {
    pub achievement_id: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapAchievementUnlockResponse {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub achievement: *const TapAchievementInfo,
    pub platinum_achievement: *const TapAchievementInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapAchievementIncrementRequest {
    pub achievement_id: *const c_char,
    pub steps: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapAchievementIncrementResponse {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub achievement: *const TapAchievementInfo,
    pub platinum_achievement: *const TapAchievementInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapComplianceAction {
    pub action_type: u32,
    pub title: *const c_char,
    pub description: *const c_char,
    pub display_duration_seconds: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapComplianceActionsEvent {
    pub count: u32,
    pub actions: *const TapComplianceAction,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapComplianceEnsureRealNameResponse {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub status: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapComplianceCheckPaymentLimitRequest {
    pub amount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapComplianceCheckPaymentLimitResponse {
    pub allow: bool,
    pub title: [c_char; 256],
    pub description: [c_char; 4096],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapComplianceSubmitPaymentRequest {
    pub amount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameEvent {
    pub request_id: i64,
    pub error: *const TapSDK_Error,
    pub event_id: u32,
    pub event_data: *const c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameConnectResponse {
    pub player_id: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameMatchParam {
    pub key: *const c_char,
    pub value: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameRoomConfig {
    pub max_player_count: u32,
    pub room_type: *const c_char,
    pub match_param_count: u32,
    pub match_params: *const TapOnlineGameMatchParam,
    pub name: *const c_char,
    pub custom_properties: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGamePlayerConfig {
    pub custom_status: i32,
    pub custom_properties: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameCreateRoomRequest {
    pub room_cfg: TapOnlineGameRoomConfig,
    pub player_cfg: TapOnlineGamePlayerConfig,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGamePlayerInfo {
    pub id: *const c_char,
    pub status: i32,
    pub custom_status: i32,
    pub custom_properties: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameRoomInfo {
    pub id: *const c_char,
    pub name: *const c_char,
    pub room_type: *const c_char,
    pub owner_id: *const c_char,
    pub status: i32,
    pub custom_properties: *const c_char,
    pub max_player_count: u32,
    pub player_count: u32,
    pub players: *const TapOnlineGamePlayerInfo,
    pub create_time: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameCreateRoomResponse {
    pub room_info: *const TapOnlineGameRoomInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameMatchRoomRequest {
    pub room_cfg: TapOnlineGameRoomConfig,
    pub player_cfg: TapOnlineGamePlayerConfig,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameMatchRoomResponse {
    pub room_info: *const TapOnlineGameRoomInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameGetRoomListRequest {
    pub room_type: *const c_char,
    pub offset: u32,
    pub limit: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameRoomBasicInfo {
    pub id: *const c_char,
    pub name: *const c_char,
    pub room_type: *const c_char,
    pub status: i32,
    pub custom_properties: *const c_char,
    pub max_player_count: u32,
    pub player_count: u32,
    pub create_time: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameGetRoomListResponse {
    pub room_count: u32,
    pub rooms: *const TapOnlineGameRoomBasicInfo,
    pub offset: u32,
    pub has_more: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameJoinRoomRequest {
    pub room_id: *const c_char,
    pub player_cfg: TapOnlineGamePlayerConfig,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameJoinRoomResponse {
    pub room_info: *const TapOnlineGameRoomInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameUpdateRoomPropertiesRequest {
    pub name: *const c_char,
    pub custom_properties: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameSendCustomMessageRequest {
    pub msg: *const c_char,
    pub receiver_type: u32,
    pub receiver_count: u32,
    pub receivers: *mut *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameSendFrameInputRequest {
    pub data: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGamePlayerOfflineNotification {
    pub room_id: *const c_char,
    pub room_owner_id: *const c_char,
    pub player_id: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGamePlayerEnterRoomNotification {
    pub room_id: *const c_char,
    pub player_info: *const TapOnlineGamePlayerInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGamePlayerLeaveRoomNotification {
    pub room_id: *const c_char,
    pub room_owner_id: *const c_char,
    pub player_id: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGamePlayerCustomStatusNotification {
    pub player_id: *const c_char,
    pub status: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGamePlayerCustomPropertiesNotification {
    pub player_id: *const c_char,
    pub properties: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameRoomPropertiesNotification {
    pub id: *const c_char,
    pub name: *const c_char,
    pub custom_properties: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameCustomMessageNotification {
    pub player_id: *const c_char,
    pub msg: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameRoomPlayerKickedNotification {
    pub room_id: *const c_char,
    pub player_id: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameFrameSyncStartNotification {
    pub room_info: *const TapOnlineGameRoomInfo,
    pub frame_sync_id: i32,
    pub seed: i32,
    pub server_tms: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameFrameInput {
    pub player_id: *const c_char,
    pub data: *const c_char,
    pub server_tms: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameFrame {
    pub id: u32,
    pub input_count: u32,
    pub inputs: *const TapOnlineGameFrameInput,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TapOnlineGameFrameSyncStopNotification {
    pub room_id: *const c_char,
    pub frame_sync_id: i32,
    pub reason: i32,
}

pub type TapCallback = Option<unsafe extern "C" fn(event_id: u32, data: *mut c_void)>;
pub type TapOnlineGameCallback = Option<unsafe extern "C" fn(event: *const TapOnlineGameEvent)>;

#[inline(always)]
fn unsupported() -> ! {
    panic!("TapTap PC SDK is only supported on Windows. This platform (macOS/Linux) is not supported.")
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_RestartAppIfNecessary(_client_id: *const c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_Init(_err_msg: *mut c_char, _pub_key: *const c_char) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_Shutdown() {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_RunCallbacks() {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_GetClientID(_buffer: *mut c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_RegisterCallback(_event_id: u32, _cb: TapCallback) {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapSDK_UnregisterCallback(_event_id: u32, _cb: TapCallback) {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapUser_AsyncAuthorize(_scopes: *const c_char) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapUser_GetOpenID(_buffer: *mut c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapApps_IsOwned() -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapDLC_IsOwned(_dlc_id: *const c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapDLC_ShowStore(_dlc_id: *const c_char) -> bool {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapAchievement() -> *mut ITapAchievement {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapAchievement_AsyncUnlock(
    _handle: *mut ITapAchievement,
    _request_id: i64,
    _request: *const TapAchievementUnlockRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapAchievement_AsyncIncrement(
    _handle: *mut ITapAchievement,
    _request_id: i64,
    _request: *const TapAchievementIncrementRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapAchievement_ShowAchievements(_handle: *mut ITapAchievement) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave() -> *mut ITapCloudSave {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncList(
    _handle: *mut ITapCloudSave,
    _request_id: i64,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncCreate(
    _handle: *mut ITapCloudSave,
    _request_id: i64,
    _request: *const TapCloudSaveCreateRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncUpdate(
    _handle: *mut ITapCloudSave,
    _request_id: i64,
    _request: *const TapCloudSaveUpdateRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncDelete(
    _handle: *mut ITapCloudSave,
    _request_id: i64,
    _uuid: *const c_char,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncGetData(
    _handle: *mut ITapCloudSave,
    _request_id: i64,
    _request: *const TapCloudSaveGetFileRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCloudSave_AsyncGetCover(
    _handle: *mut ITapCloudSave,
    _request_id: i64,
    _request: *const TapCloudSaveGetFileRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCompliance() -> *mut ITapCompliance {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCompliance_AsyncEnsureRealName(
    _handle: *mut ITapCompliance,
    _request_id: i64,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCompliance_EnableAntiAddiction(
    _handle: *mut ITapCompliance,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCompliance_CheckPaymentLimit(
    _handle: *mut ITapCompliance,
    _request: *const TapComplianceCheckPaymentLimitRequest,
    _response: *mut TapComplianceCheckPaymentLimitResponse,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapCompliance_SubmitPayment(
    _handle: *mut ITapCompliance,
    _request: *const TapComplianceSubmitPaymentRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame() -> *mut ITapOnlineGame {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_RunCallbacks(
    _handle: *mut ITapOnlineGame,
    _cb: TapOnlineGameCallback,
    _max_events: u32,
    _left_events: *mut u32,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncConnect(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncDisconnect(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncCreateRoom(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
    _request: *const TapOnlineGameCreateRoomRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncMatchRoom(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
    _request: *const TapOnlineGameMatchRoomRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncGetRoomList(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
    _request: *const TapOnlineGameGetRoomListRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncJoinRoom(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
    _request: *const TapOnlineGameJoinRoomRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncLeaveRoom(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncUpdatePlayerCustomStatus(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
    _status: i32,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncUpdatePlayerCustomProperties(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
    _properties: *const c_char,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncUpdateRoomProperties(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
    _request: *const TapOnlineGameUpdateRoomPropertiesRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncSendCustomMessage(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
    _request: *const TapOnlineGameSendCustomMessageRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncKickRoomPlayer(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
    _player_id: *const c_char,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncStartFrameSync(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncSendFrameInput(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
    _request: *const TapOnlineGameSendFrameInputRequest,
) -> u32 {
    unsupported()
}

#[no_mangle]
pub unsafe extern "C" fn TapOnlineGame_AsyncStopFrameSync(
    _handle: *mut ITapOnlineGame,
    _request_id: i64,
) -> u32 {
    unsupported()
}
