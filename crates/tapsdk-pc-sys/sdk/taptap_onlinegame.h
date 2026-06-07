#pragma once

#include "taptap_api.h"

// 注意：所有接口的字符串参数（char *）都必须是 UTF-8 编码

//------------------------------------------------------------------------------
// 枚举定义
//------------------------------------------------------------------------------

// 联机游戏事件类型枚举值
enum {
    TapOnlineGameEventID_Unknown = 0,

    // [1, 1000) 预留给异步请求的回调 (Response)
    TapOnlineGameEventID_ConnectResponse = 1,                  // 建连请求回调，成功时使用TapOnlineGameConnectResponse*强转event_data
    TapOnlineGameEventID_DisconnectResponse = 2,               // 断连请求回调，必定成功，event_data为NULL
    TapOnlineGameEventID_CreateRoomResponse = 3,               // 创建房间请求回调，成功时使用TapOnlineGameCreateRoomResponse*强转event_data
    TapOnlineGameEventID_MatchRoomResponse = 4,                // 匹配房间请求回调，成功时使用TapOnlineGameMatchRoomResponse*强转event_data
    TapOnlineGameEventID_GetRoomListResponse = 5,              // 获取房间列表请求回调，成功时使用TapOnlineGameGetRoomListResponse*强转event_data
    TapOnlineGameEventID_JoinRoomResponse = 6,                 // 加入房间请求回调，成功时使用TapOnlineGameJoinRoomResponse*强转event_data
    TapOnlineGameEventID_LeaveRoomResponse = 7,                // 离开房间请求回调，event_data为NULL
    TapOnlineGameEventID_UpdatePlayerCustomStatusResponse = 8, // 更新玩家自定义状态请求回调，event_data为NULL
    TapOnlineGameEventID_UpdatePlayerCustomPropertiesResponse = 9, // 更新玩家自定义属性请求回调，event_data为NULL
    TapOnlineGameEventID_UpdateRoomPropertiesResponse = 10,    // 更新房间属性请求回调，event_data为NULL
    TapOnlineGameEventID_SendCustomMessageResponse = 11,       // 发送自定义消息请求回调，event_data为NULL
    TapOnlineGameEventID_KickRoomPlayerResponse = 12,          // 踢玩家出房间请求回调，event_data为NULL
    TapOnlineGameEventID_StartFrameSyncResponse = 13,          // 开始帧同步请求回调，event_data为NULL
    TapOnlineGameEventID_SendFrameInputResponse = 14,          // 发送操作数据请求回调，event_data为NULL
    TapOnlineGameEventID_StopFrameSyncResponse = 15,           // 结束帧同步请求回调，event_data为NULL

    // [1000, 2000) 预留给服务端主动推送的通知 (Notification)
    TapOnlineGameEventID_ServiceErrorNotification = 1000,          // 对战服务异常通知。收到该通知时，游戏需要退出房间、对战状态
    TapOnlineGameEventID_DisconnectNotification = 1001,            // 被踢、异常断线时通知，通过error字段获取具体的错误信息。如果是断线，可以重连；如果是被踢，不要重连
    TapOnlineGameEventID_PlayerOfflineNotification = 1002,         // 玩家离线通知。只有对战中的玩家离线才会触发，通知房间内所有玩家，使用TapOnlineGamePlayerOfflineNotification*强转event_data
    TapOnlineGameEventID_EnterRoomNotification = 1003,             // 通知房间内所有玩家，有新玩家进入房间，使用TapOnlineGamePlayerEnterRoomNotification*强转event_data
    TapOnlineGameEventID_LeaveRoomNotification = 1004,             // 通知房间内所有玩家，有玩家离开房间，使用TapOnlineGamePlayerLeaveRoomNotification*强转event_data
    TapOnlineGameEventID_PlayerCustomStatusNotification = 1005,    // 玩家自定义状态变更时触发，通知房间内所有玩家，使用TapOnlineGamePlayerCustomStatusNotification*强转event_data
    TapOnlineGameEventID_PlayerCustomPropertiesNotification = 1006,// 玩家自定义属性更新时触发，通知房间内所有玩家，使用TapOnlineGamePlayerCustomPropertiesNotification*强转event_data
    TapOnlineGameEventID_RoomPropertiesNotification = 1007,        // 房间属性更新时触发，通知房间内所有玩家，使用TapOnlineGameRoomPropertiesNotification*强转event_data
    TapOnlineGameEventID_CustomMessageNotification = 1008,         // 收到自定义消息时回调，使用TapOnlineGameCustomMessageNotification*强转event_data
    TapOnlineGameEventID_RoomPlayerKickedNotification = 1009,      // 玩家被踢出房间通知，使用TapOnlineGameRoomPlayerKickedNotification*强转event_data
    TapOnlineGameEventID_FrameSyncStartNotification = 1010,        // 通知房间内所有玩家，帧同步开始，使用TapOnlineGameFrameSyncStartNotification*强转event_data
    TapOnlineGameEventID_FrameNotification = 1011,                 // 给房间内所有玩家同步最新的对战帧，使用TapOnlineGameFrame*强转event_data
    TapOnlineGameEventID_FrameSyncStopNotification = 1012,         // 通知房间内所有玩家，帧同步结束，使用TapOnlineGameFrameSyncStopNotification*强转event_data
};
// 联机游戏事件类型，取值请参考枚举值 TapOnlineGameEventID_*
typedef uint32_t TapOnlineGameEventID;

//------------------------------------------------------------------------------
// 结构体定义
//------------------------------------------------------------------------------

#pragma pack(push, 8)

// 联机游戏通用事件结构体
typedef struct {
    int64_t request_id;             // 请求 ID。原样返回开发者调用异步接口时传入的 ID，开发者可使用该 ID 对应到原始请求。如果是服务端通知，该字段的值为 0
    const TapSDK_Error* error;      // 错误信息。NULL 表示请求成功；非 NULL 表示请求失败，可根据其中的错误码做相应处理
    TapOnlineGameEventID event_id;  // 根据事件 ID，使用正确的结构体强转 event_data
    const void* event_data;         // 事件数据，比如异步请求结果或者服务端通知消息
} TapOnlineGameEvent;

// TapOnlineGame_AsyncConnect() 请求成功时的返回
typedef struct {
	const char* player_id; // 玩家ID
} TapOnlineGameConnectResponse;

// 匹配参数键值对
typedef struct {
	const char* key;   // 匹配参数键，最大32字节
	const char* value; // 匹配参数值，最大64字节
} TapOnlineGameMatchParam;

// 房间配置
typedef struct {
	uint32_t max_player_count;                    // 房间最大人数，[1,20]，必填
	const char* room_type;                        // 房间类型，最大32字节，必填
	uint32_t match_param_count;                   // match_params数组的长度，最多支持3条匹配规则
	const TapOnlineGameMatchParam* match_params;  // 自定义匹配参数数组，长度为match_param_count。房间匹配时，同时满足max_player_count+room_type+match_params才能匹配上
	const char* name;                             // 房间名称
	const char* custom_properties;                // 房间自定义属性，最大长度为2048字节
} TapOnlineGameRoomConfig;

// 玩家配置
typedef struct {
	int32_t custom_status;         // 玩家自定义状态
	const char* custom_properties; // 玩家自定义属性，最大长度为2048字节
} TapOnlineGamePlayerConfig;

// 创建房间请求，用于 TapOnlineGame_AsyncCreateRoom()
typedef struct {
	TapOnlineGameRoomConfig room_cfg;   // 房间配置
	TapOnlineGamePlayerConfig player_cfg; // 玩家配置
} TapOnlineGameCreateRoomRequest;

// 玩家信息
typedef struct {
	const char* id; // 玩家ID
	int32_t status; // 玩家状态：0-离线、1-在线
	int32_t custom_status; // 玩家自定义状态
	const char* custom_properties; // 玩家自定义属性，最大长度为2048字节
} TapOnlineGamePlayerInfo;

// 房间信息
typedef struct {
	const char* id; // 房间ID
	const char* name; // 房间名称
	const char* room_type; // 房间类型
	const char* owner_id; // 房主ID
	int32_t status; // 房间状态，0-未开始帧同步，1-帧同步中
	const char* custom_properties; // 房间自定义属性，最大长度为2048字节
	uint32_t max_player_count; // 房间最大人数，[1,20]
	uint32_t player_count; // 房间内玩家数量
	const TapOnlineGamePlayerInfo* players; // 房间内玩家列表，长度为player_count
	int64_t create_time; // 房间创建时间，1970年开始的秒数
} TapOnlineGameRoomInfo;

// TapOnlineGame_AsyncCreateRoom() 请求成功时的返回
typedef struct {
	const TapOnlineGameRoomInfo* room_info; // 成功创建的房间信息
} TapOnlineGameCreateRoomResponse;

// 匹配房间请求，用于 TapOnlineGame_AsyncMatchRoom()
typedef struct {
	TapOnlineGameRoomConfig room_cfg;   // 房间配置，用于匹配不到时创建房间
	TapOnlineGamePlayerConfig player_cfg; // 玩家配置
} TapOnlineGameMatchRoomRequest;

// TapOnlineGame_AsyncMatchRoom() 请求成功时的返回
typedef struct {
	const TapOnlineGameRoomInfo* room_info; // 成功匹配/创建的房间信息
} TapOnlineGameMatchRoomResponse;

// 拉取房间列表请求，用于 TapOnlineGame_AsyncGetRoomList()
typedef struct {
	const char* room_type; // 房间类型。不填则拉取全部类型的房间
	uint32_t offset;      // 偏移量，第一次请求时为0，或者不填
	uint32_t limit;       // 请求获取的房间数量，不填则默认为20，最大100
} TapOnlineGameGetRoomListRequest;

// 房间基本信息
typedef struct {
	const char* id; // 房间ID
	const char* name; // 房间名称
	const char* room_type; // 房间类型
	int32_t  status; // 房间状态，0-未开始帧同步，1-帧同步中
	const char* custom_properties; // 房间自定义属性，最大长度为2048字节
	uint32_t max_player_count; // 房间最大人数
	uint32_t player_count; // 房间当前人数
	int64_t  create_time; // 房间创建时间，1970年开始的秒数
} TapOnlineGameRoomBasicInfo;

// TapOnlineGame_AsyncGetRoomList() 请求成功时的返回
typedef struct {
	uint32_t room_count; // rooms数组的长度
	const TapOnlineGameRoomBasicInfo* rooms; // 本次请求得到的可加入房间列表，长度为room_count
	uint32_t offset; // 用于请求下一页的偏移量
	bool has_more; // 是否还有更多房间可以拉取
} TapOnlineGameGetRoomListResponse;

// 加入房间请求，用于 TapOnlineGame_AsyncJoinRoom()
typedef struct {
	const char* room_id; // 房间ID，不允许空
	TapOnlineGamePlayerConfig player_cfg; // 玩家配置
} TapOnlineGameJoinRoomRequest;

// TapOnlineGame_AsyncJoinRoom() 请求成功时的返回
typedef struct {
	const TapOnlineGameRoomInfo* room_info; // 成功进入的房间信息
} TapOnlineGameJoinRoomResponse;

// 更新房间属性请求，用于 TapOnlineGame_AsyncUpdateRoomProperties()
typedef struct {
	const char* name;              // 房间名称，最大64个字节
	const char* custom_properties; // 房间自定义属性，最大2048字节
} TapOnlineGameUpdateRoomPropertiesRequest;

// 发送自定义消息请求，用于 TapOnlineGame_AsyncSendCustomMessage()
typedef struct {
	const char* msg;         // 自定义消息，不能为空，格式由开发者决定，必须是utf8字符串，最大2048字节
	uint32_t receiver_type;  // 消息接收者类型。0 - 玩家所在房间的所有人（不包括发送者），1 - 指定玩家
	uint32_t receiver_count; // receivers数组的长度，最多20个玩家ID
	const char** receivers;  // 消息接收者ID数组，type为1时有效，长度为receiver_count
} TapOnlineGameSendCustomMessageRequest;

// 发送操作数据，用于 TapOnlineGame_AsyncSendFrameInput()
typedef struct {
	const char* data; // 游戏操作，utf8字符串格式
} TapOnlineGameSendFrameInputRequest;

// 玩家离线通知，仅当帧同步开始后离线才会发送该通知
typedef struct {
	const char* room_id; // 离线玩家所属房间ID
	const char* room_owner_id; // 房主ID。如果离线的是房主，则room_owner_id为新房主ID；反之，为原房主ID
	const char* player_id; // 离线玩家ID
} TapOnlineGamePlayerOfflineNotification;

// 玩家进入房间通知
typedef struct {
	const char* room_id; // 玩家进入的房间ID
	const TapOnlineGamePlayerInfo* player_info; // 玩家信息
} TapOnlineGamePlayerEnterRoomNotification;

// 玩家离开房间通知
typedef struct {
	const char* room_id; // 玩家离开的房间ID
	const char* room_owner_id; // 房主ID。如果离开的是房主，则room_owner_id为新房主ID；反之，为原房主ID
	const char* player_id; // 玩家ID
} TapOnlineGamePlayerLeaveRoomNotification;

// 玩家自定义状态变更通知
typedef struct {
	const char* player_id; // 玩家ID
	int32_t status; // 玩家自定义状态
} TapOnlineGamePlayerCustomStatusNotification;

// 玩家自定义属性变更通知
typedef struct {
	const char* player_id; // 玩家ID
	const char* properties; // 玩家自定义属性
} TapOnlineGamePlayerCustomPropertiesNotification;

// 房间属性变更通知
typedef struct {
	const char* id; // 房间ID
	const char* name; // 房间名称
	const char* custom_properties; // 房间自定义属性
} TapOnlineGameRoomPropertiesNotification;

// 自定义消息通知
typedef struct {
	const char* player_id; // 消息发送者玩家ID
	const char* msg;       // 自定义消息，格式由开发者决定，必须是utf8字符串，最大2048字节
} TapOnlineGameCustomMessageNotification;

// 玩家被踢出房间通知
typedef struct {
	const char* room_id;   // 被踢玩家所属房间ID
	const char* player_id; // 被踢玩家ID
} TapOnlineGameRoomPlayerKickedNotification;

// 帧同步开始通知
typedef struct {
	const TapOnlineGameRoomInfo* room_info; // 开始帧同步的房间信息
	int32_t frame_sync_id;                  // 帧同步ID，房间内唯一，房间内每次开始帧同步都会变化
	int32_t seed;                           // 用于初始化线性同余伪随机数生成器的随机数种子
	int64_t server_tms;                     // 对战开始的服务端时间，1970年开始的毫秒数
} TapOnlineGameFrameSyncStartNotification;

// 单个玩家在一帧内的一个操作数据
typedef struct {
	const char* player_id; // 玩家ID
	const char* data;      // 玩家的一个操作数据，utf8字符串格式
	int64_t server_tms;   // 服务器收到该输入的时间，1970年开始的毫秒数
} TapOnlineGameFrameInput;

// 帧同步数据结构体
typedef struct {
	uint32_t id;                              // 当前帧ID，从1开始递增
	uint32_t input_count;                     // inputs数组的长度
	const TapOnlineGameFrameInput* inputs;    // 当前帧的所有玩家操作，按服务端收到顺序排序，长度为input_count
} TapOnlineGameFrame;

// 帧同步结束通知
typedef struct {
	const char* room_id;     // 帧同步结束的房间ID
	int32_t frame_sync_id;   // 帧同步ID，房间内唯一，房间内每次开始帧同步都会变化
	int32_t reason;          // 帧同步结束原因。0房主主动结束帧同步，1帧同步因超时结束
} TapOnlineGameFrameSyncStopNotification;

#pragma pack(pop)

//------------------------------------------------------------------------------
// 联机游戏功能
//------------------------------------------------------------------------------

// 回调函数类型定义
typedef void (T_CALLTYPE *TapOnlineGameCallback)(const TapOnlineGameEvent*);

// 联机游戏接口对象，通过 TapOnlineGame() 获取
typedef struct ITapOnlineGame ITapOnlineGame;

/**
 * 获取联机游戏接口单例对象
 * @return 联机游戏接口单例对象
 */
T_API ITapOnlineGame* T_CALLTYPE TapOnlineGame();

/**
 * 调用cb处理联机游戏事件，建议每帧调用
 *
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param cb 事件处理函数
 * @param maxEvents 本次调用最多处理事件数量，传0表示全量处理。一般传10即可。
 * @param leftEvents 输出参数，返回剩余未处理的事件数。开发者可以通过判断未处理事件数来决定是否需要多调用几次本函数以处理完全部事件。
 *
 * @return 本次调用处理的事件数
 */
T_API uint32_t T_CALLTYPE TapOnlineGame_RunCallbacks(
	ITapOnlineGame* self,
	TapOnlineGameCallback cb,
	uint32_t maxEvents,
	uint32_t* leftEvents
);

/**
 * 发起和联机游戏服务的建立长连接的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_ConnectResponse事件返回
 * @note 请求成功事件返回后，才能发起创建房间、匹配房间等请求
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncConnect(
    ITapOnlineGame* self,
    int64_t request_id
);

/**
 * 发起和联机游戏服务断开连接的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_DisconnectResponse事件返回
 * @note 断连后，再次执行TapOnlineGame_AsyncConnect成功前，无法发起创建房间、匹配房间等请求
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncDisconnect(
    ITapOnlineGame* self,
    int64_t request_id
);

/**
 * 发起创建房间的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_CreateRoomResponse事件返回
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @param request 创建房间请求参数，不允许空指针
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncCreateRoom(
    ITapOnlineGame* self,
    int64_t request_id,
    const TapOnlineGameCreateRoomRequest* request
);

/**
 * 发起匹配房间的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_MatchRoomResponse事件返回
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @param request 匹配房间请求参数，不允许空指针
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncMatchRoom(
    ITapOnlineGame* self,
    int64_t request_id,
    const TapOnlineGameMatchRoomRequest* request
);

/**
 * 发起拉取房间列表的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_GetRoomListResponse事件返回
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @param request 拉取房间列表请求参数
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncGetRoomList(
    ITapOnlineGame* self,
    int64_t request_id,
    const TapOnlineGameGetRoomListRequest* request
);

/**
 * 发起加入房间的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_JoinRoomResponse事件返回
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @param request 加入房间请求参数
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncJoinRoom(
    ITapOnlineGame* self,
    int64_t request_id,
    const TapOnlineGameJoinRoomRequest* request
);

/**
 * 发起离开房间的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_LeaveRoomResponse事件返回
 * @note 处于帧同步状态时不允许调用，会通过TapOnlineGameEventID_LeaveRoomResponse返回对应错误
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncLeaveRoom(
    ITapOnlineGame* self,
    int64_t request_id
);

/**
 * 发起更新玩家自定义状态的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_UpdatePlayerCustomStatusResponse事件返回
 * @note 处于帧同步状态时不允许调用，会通过TapOnlineGameEventID_UpdatePlayerCustomStatusResponse返回对应错误
 * @note 和TapOnlineGame_AsyncUpdatePlayerCustomProperties()、TapOnlineGame_AsyncUpdateRoomProperties()、TapOnlineGame_AsyncSendCustomMessage()共享每秒15次调用限制
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @param status 玩家自定义状态
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncUpdatePlayerCustomStatus(
    ITapOnlineGame* self,
    int64_t request_id,
    int32_t status
);

/**
 * 发起更新玩家自定义属性的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_UpdatePlayerCustomPropertiesResponse事件返回
 * @note 处于帧同步状态时不允许调用，会通过TapOnlineGameEventID_UpdatePlayerCustomPropertiesResponse返回对应错误
 * @note 和TapOnlineGame_AsyncUpdatePlayerCustomStatus()、TapOnlineGame_AsyncUpdateRoomProperties()、TapOnlineGame_AsyncSendCustomMessage()共享每秒15次调用限制
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @param properties 玩家自定义属性，UTF-8 编码的字符串，允许空
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncUpdatePlayerCustomProperties(
    ITapOnlineGame* self,
    int64_t request_id,
    const char* properties
);

/**
 * 发起更新房间属性的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_UpdateRoomPropertiesResponse事件返回
 * @note 处于帧同步状态时不允许调用，且仅限房主调用，会通过TapOnlineGameEventID_UpdateRoomPropertiesResponse返回对应错误
 * @note 和TapOnlineGame_AsyncUpdatePlayerCustomStatus()、TapOnlineGame_AsyncUpdatePlayerCustomProperties()、TapOnlineGame_AsyncSendCustomMessage()共享每秒15次调用限制
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @param request 更新房间属性请求参数，不允许空指针
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncUpdateRoomProperties(
    ITapOnlineGame* self,
    int64_t request_id,
    const TapOnlineGameUpdateRoomPropertiesRequest* request
);

/**
 * 发起发送自定义消息的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_SendCustomMessageResponse事件返回
 * @note 和TapOnlineGame_AsyncUpdatePlayerCustomStatus()、TapOnlineGame_AsyncUpdatePlayerCustomProperties()、TapOnlineGame_AsyncUpdateRoomProperties()共享每秒15次调用限制
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @param request 发送自定义消息请求参数，不允许空指针
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncSendCustomMessage(
    ITapOnlineGame* self,
    int64_t request_id,
    const TapOnlineGameSendCustomMessageRequest* request
);

/**
 * 发起把玩家踢出房间的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_KickRoomPlayerResponse事件返回
 * @note 帧同步中不允许踢人，仅限房主调用，会通过TapOnlineGameEventID_KickRoomPlayerResponse返回对应错误
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @param player_id 被踢玩家的 ID，UTF-8 编码的字符串，不允许空
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncKickRoomPlayer(
    ITapOnlineGame* self,
    int64_t request_id,
    const char* player_id
);

/**
 * 发起开始帧同步的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_StartFrameSyncResponse事件返回
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncStartFrameSync(
    ITapOnlineGame* self,
    int64_t request_id
);

/**
 * 发起发送玩家操作的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_SendFrameInputResponse事件返回
 * @note 同一帧允许最多发送5次操作，超限会通过TapOnlineGameEventID_SendFrameInputResponse返回对应错误
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @param request 发送玩家操作请求参数，不允许空指针
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncSendFrameInput(
    ITapOnlineGame* self,
    int64_t request_id,
    const TapOnlineGameSendFrameInputRequest* request
);

/**
 * 发起停止帧同步的异步请求。如果请求发起成功，请求处理结果会通过TapOnlineGameEventID_StopFrameSyncResponse事件返回
 * @param self TapOnlineGame() 返回的联机游戏单例对象
 * @param request_id 开发者生成的请求 ID，请求处理完成后，调用回调函数时原样返回，开发者可使用该 ID 对应到原始请求
 * @return 请求发起结果，如果不是 TapSDK_Result_OK，表示请求发起失败，不会触发回调函数
 */
T_API TapSDK_Result T_CALLTYPE TapOnlineGame_AsyncStopFrameSync(
    ITapOnlineGame* self,
    int64_t request_id
);
