#ifndef TAPSDK_H
#define TAPSDK_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
#define T_API extern "C"
#else
#define T_API
#endif

#ifdef _WIN32
#define T_CALLTYPE __cdecl
#else
#define T_CALLTYPE
#endif

//------------------------------------------------------------------------------
// 基础类型定义
//------------------------------------------------------------------------------

typedef char ErrMsg[1024];       // 错误信息，最大长度 1023 字节 + '\0'

//------------------------------------------------------------------------------
// 常量定义
//------------------------------------------------------------------------------

#ifdef __cplusplus

// SDK 初始化结果枚举
enum class TapSDK_Init_Result : uint32_t {
    OK = 0,                     // 初始化成功
    FailedGeneric = 1,          // 其他错误
    NoPlatform = 2,             // 未找到 TapTap 平台
    NotLaunchedByPlatform = 3,  // 未通过 TapTap 启动
    PlatformVersionMismatch = 4 // 平台版本不匹配，请引导用户升级 TapTap 与游戏至最新版本，再重新运行游戏
};

// 授权结果枚举
enum class TapUser_AsyncAuthorize_Result : uint32_t {
    Unknown = 0,    // 未知错误，无法请求授权，请检查 SDK 是否完成初始化 (调用 TapSDK_Init 并返回 TapSDK_Init_Result::OK)
    OK = 1,         // 成功发起授权流程，等待用户确认或者自动授权完成（此时授权流程还未完成）
    Failed = 2,     // 发起授权流程失败，可能的原因：1. 用户网络问题，2. TapTap 平台内部错误。请引导用户稍后重试
    InFlight = 3    // 授权流程正在执行中，请等待授权流程完成
};

#else

// SDK 初始化结果枚举值
enum {
    TapSDK_Init_Result_OK = 0,                      // 初始化成功
    TapSDK_Init_Result_FailedGeneric = 1,           // 其他错误
    TapSDK_Init_Result_NoPlatform = 2,              // 未找到 TapTap 平台
    TapSDK_Init_Result_NotLaunchedByPlatform = 3,   // 未通过 TapTap 启动
    TapSDK_Init_Result_PlatformVersionMismatch = 4  // 平台版本不匹配，请引导用户升级 TapTap 与游戏至最新版本，再重新运行游戏
};
// SDK 初始化结果，取值请参考枚举值 TapSDK_Init_Result_*
typedef uint32_t TapSDK_Init_Result;

// 授权结果枚举值
enum {
    TapUser_AsyncAuthorize_Result_Unknown = 0, // 未知错误，无法请求授权，请检查 SDK 是否完成初始化 (调用 TapSDK_Init 并返回 TapSDK_Init_Result::OK)}
    TapUser_AsyncAuthorize_Result_OK = 1,      // 成功发起授权流程，等待用户确认或者自动授权完成（此时授权流程还未完成）
    TapUser_AsyncAuthorize_Result_Failed = 2,  // 发起授权流程失败，可能的原因：1. 用户网络问题，2. TapTap 平台内部错误。请引导用户稍后重试
    TapUser_AsyncAuthorize_Result_InFlight = 3 // 授权流程正在执行中，请等待授权流程完成
};
// 授权结果，取值请参考枚举值 TapUser_AsyncAuthorize_Result_*
typedef uint32_t TapUser_AsyncAuthorize_Result;

#endif

// 错误码枚举值
enum {
  TapSDK_ErrorCode_Success = 0,              // 请求执行成功
  TapSDK_ErrorCode_Unknown = 1,              // 未知错误
  TapSDK_ErrorCode_Unauthorized = 2,         // 用户凭证失效，请引导用户重新登录 TapTap
  TapSDK_ErrorCode_MethodNotAllowed = 3,     // 不允许的接口请求
  TapSDK_ErrorCode_Unimplemented = 4,        // 接口未实现
  TapSDK_ErrorCode_InvalidArguments = 5,     // 参数错误
  TapSDK_ErrorCode_Forbidden = 6,            // 用户没有当前动作的权限
  TapSDK_ErrorCode_UserIsDeactivated = 7,    // 用户被冻结
  TapSDK_ErrorCode_InternalServerError = 8,  // 服务器内部错误
  TapSDK_ErrorCode_InternalSdkError = 9,     // SDK内部错误
  TapSDK_ErrorCode_NetworkError = 10,        // 网络错误

	// reserved 200000 ~ 299999 防沉迷使用

	// reserved 400000 ~ 499999 云存档使用
  TapSDK_ErrorCode_CloudSave_InvalidFileSize = 400000,               // 非法的存档文件/封面大小
  TapSDK_ErrorCode_CloudSave_UploadRateLimit = 400001,               // 存档上传频率超限
  TapSDK_ErrorCode_CloudSave_FileNotFound = 400002,                  // 存档文件不存在
  TapSDK_ErrorCode_CloudSave_FileCountLimitPerClient = 400003,       // 用户在该用下存档文件数量超限
  TapSDK_ErrorCode_CloudSave_StorageSizeLimitPerClient = 400004,     // 用户在该应用下使用存储空间超限
  TapSDK_ErrorCode_CloudSave_TotalStorageSizeLimit = 400005,         // 用户总使用存储空间超限
  TapSDK_ErrorCode_CloudSave_Timeout = 400006,                       // 请求超时，通常是由于网络卡顿，创建/更新存档耗时过长导致
  TapSDK_ErrorCode_CloudSave_ConcurrentCallDisallowed = 400007,      // 不允许并发调用的请求
  TapSDK_ErrorCode_CloudSave_StorageServerError = 400008,            // 存储服务故障
  TapSDK_ErrorCode_CloudSave_InvalidName = 400009,                   // 存档名称不合法

	// reserved 500000 ~ 599999 排行榜使用
};
// 错误码，取值请参考枚举值 TapSDK_ErrorCode_*
typedef int64_t TapSDK_ErrorCode;

// TapPC系统状态枚举
enum {
  TapSystemState_Unknown = 0,

  // TapTap客户端当前可以正常访问TapTap服务端。
  // 开发者收到这个状态通知时，可以解除之前收到TapSystemState_PlatformOffline状态通知时对游戏做的限制。
  TapSystemState_PlatformOnline = 1,
  // TapTap客户端当前无法访问TapTap服务端：网络异常或者TapTap服务端故障。
  // 当TapTap客户端处于这个状态时，无法实时获得游戏/DLC所有权变化通知，比如已退款。
  // 开发者收到这个状态通知时，可以提醒玩家检查网络状态，或者做其他游戏限制。
  TapSystemState_PlatformOffline = 2,
  // TapTap客户端退出。
  // 开发者收到这个状态通知时，应该立刻保存游戏存档，然后退出游戏。
  TapSystemState_PlatformShutdown = 3,
};
// TapPC系统状态，取值请参考枚举值TapSystemState_*
typedef uint32_t TapSystemState;

//------------------------------------------------------------------------------
// 回调相关定义
//------------------------------------------------------------------------------

#ifdef __cplusplus

// 事件 ID 枚举
enum class TapEventID : uint32_t {
  Unknown = 0,

  // [1, 2000), reserved for TapTap platform events

  // TapTap客户端系统状态变化通知，SDK初始化成功后注册此事件的回调函数，以便及时获知TapTap客户端状态变化
  // 使用 TapSystemStateNotification 结构体解析
  SystemStateChanged = 1,

  // [2001, 4000), reserved for TapTap user events
  AuthorizeFinished = 2002,

  // [4001, 6000), reserved for TapTap ownership events
  GamePlayableStatusChanged = 4001,
  DLCPlayableStatusChanged = 4002,

  // [6001, 8000), reserved for TapTap CloudSave events
  CloudSaveList = 6001,     // 获取云存档列表
  CloudSaveCreate = 6002,   // 创建云存档
  CloudSaveUpdate = 6003,   // 更新云存档
  CloudSaveDelete = 6004,   // 删除云存档
  CloudSaveGetData = 6005,  // 获取云存档数据
  CloudSaveGetCover = 6006, // 获取云存档封面
};

#else

// 事件 ID 枚举值
enum {
  TapEventID_Unknown = 0,

  // [1, 2000), reserved for TapTap platform events

  // TapTap客户端系统状态变化通知，SDK初始化成功后注册此事件的回调函数，以便及时获知TapTap客户端状态变化。
  // 使用 TapSystemStateNotification 结构体解析
  TapEventID_SystemStateChanged = 1,

  // [2001, 4000), reserved for TapTap user events
  TapEventID_AuthorizeFinished = 2002,

  // [4001, 6000), reserved for TapTap ownership events
  TapEventID_GamePlayableStatusChanged = 4001,
  TapEventID_DLCPlayableStatusChanged = 4002,

  // [6001, 8000), reserved for TapTap CloudSave events
  TapEventID_CloudSaveList = 6001,     // 获取云存档列表回调，使用 TapCloudSaveListResponse 结构体解析
  TapEventID_CloudSaveCreate = 6002,   // 创建云存档回调，使用 TapCloudSaveCreateResponse 结构体解析
  TapEventID_CloudSaveUpdate = 6003,   // 更新云存档回调，使用 TapCloudSaveUpdateResponse 结构体解析
  TapEventID_CloudSaveDelete = 6004,   // 删除云存档回调，使用 TapCloudSaveDeleteResponse 结构体解析
  TapEventID_CloudSaveGetData = 6005,  // 获取云存档数据回调，使用 TapCloudSaveGetFileResponse 结构体解析
  TapEventID_CloudSaveGetCover = 6006, // 获取云存档封面回调，使用 TapCloudSaveGetFileResponse 结构体解析
};
// 事件 ID
typedef uint32_t TapEventID;

#endif

// 回调函数类型定义
typedef void (T_CALLTYPE *callback_t)(TapEventID, void *);

//------------------------------------------------------------------------------
// 结构体定义
//------------------------------------------------------------------------------

#pragma pack(push, 8)

// 错误信息结构体
typedef struct {
    TapSDK_ErrorCode code; // 错误码
    const char* message;   // 错误信息
} TapSDK_Error;

// TapPC系统状态通知
typedef struct {
    TapSystemState state; // TapPC当前系统状态
} TapSystemStateNotification;

struct AuthorizeFinishedResponse {
    bool is_cancel;
    char error[1024];

    char token_type[32];
    char kid[8*1024];
    char mac_key[8*1024];
    char mac_algorithm[32];
    char scope[1024];
};

//  游戏本体可玩状态变更事件响应结构体
struct GamePlayableStatusChangedResponse {
    bool is_playable;            // 游戏本体是否可玩
};

// DLC 可玩状态变更事件响应结构体
struct DLCPlayableStatusChangedResponse {
    char dlc_id[32];          // DLC ID
    bool is_playable;            // 是否可玩，当用户购买 DLC （外置 DLC 为购买且下载完成后），此值返回 true。其他情况返回 false
};

#pragma pack(pop)

//------------------------------------------------------------------------------
// SDK 核心功能
//------------------------------------------------------------------------------

/**
 * 检查是否需要重启应用
 * 此函数应该在初始化 (TapSDK_Init) 前调用，用于检查是否需要重启应用
 * @param clientID 客户端 ID
 * @return true 表示需要重启，此时 TapTap 将会重新打开游戏，请尽快退出游戏进程
 */
T_API bool T_CALLTYPE TapSDK_RestartAppIfNecessary(const char *clientID);

/**
 * 初始化 SDK，除 TapSDK_RestartAppIfNecessary 外，其他函数都应该在初始化完成后调用
 * @param errMsg 错误信息缓冲区，长度为 1024 字节。错误信息以 \0 结尾
 * @param pubKey 从 TapTap 开发者中心获取的公钥
 * @return SDK 初始化结果
 */
T_API TapSDK_Init_Result T_CALLTYPE TapSDK_Init(ErrMsg *errMsg, const char *pubKey);

/**
 * 关闭 SDK，释放资源
 * @return 是否成功关闭
 */
T_API bool T_CALLTYPE TapSDK_Shutdown();

/**
 * 获取当前客户端 ID
 * @param buffer 用于存储客户端 ID 的缓冲区，以 \0 结尾，固定长度为 256 字节
 * @return 是否成功获取客户端 ID
 */
T_API bool T_CALLTYPE TapSDK_GetClientID(char *buffer);

/**
 * 检查是否拥有当前游戏
 * @return true 表示拥有当前游戏，false 表示未拥有
 */
T_API bool T_CALLTYPE TapApps_IsOwned();

//------------------------------------------------------------------------------
// 回调相关功能
//------------------------------------------------------------------------------

/**
 * 注册事件回调
 * @param eventID 事件 ID
 * @param callback 回调函数
 */
T_API void T_CALLTYPE TapSDK_RegisterCallback(TapEventID eventID, callback_t callback);

/**
 * 注销事件回调
 * @param eventID 事件 ID
 * @param callback 要注销的回调函数
 */
T_API void T_CALLTYPE TapSDK_UnregisterCallback(TapEventID eventID, callback_t callback);

/**
 * 处理回调事件，建议每帧调用
 */
T_API void T_CALLTYPE TapSDK_RunCallbacks();

//------------------------------------------------------------------------------
// 用户相关功能
//------------------------------------------------------------------------------

/**
 * 异步请求用户授权（简化版本）
 * @param scopes 权限范围字符串，多个权限用逗号分隔，如 "public_profile,user_friends"
 * @return 授权请求结果
 */
T_API TapUser_AsyncAuthorize_Result T_CALLTYPE TapUser_AsyncAuthorize(const char* scopes);

/**
 * 获取用户 OpenID
 * @param buffer 用于存储用户 OpenID 的缓冲区，以 \0 结尾，固定长度为 256 字节
 * @return 是否成功获取用户 OpenID
 */
T_API bool T_CALLTYPE TapUser_GetOpenID(char *buffer);

//------------------------------------------------------------------------------
// DLC 相关功能
//------------------------------------------------------------------------------

/**
 * 显示指定 DLC 的商店页面
 * @param dlc_id DLC ID
 * @return 是否成功显示商店页面
 */
T_API bool T_CALLTYPE TapDLC_ShowStore(const char *dlc_id);

/**
 * 查询用户是否拥有指定的 DLC
 * @param dlc_id DLC ID
 * @return true 表示用户拥有该 DLC，false 表示未拥有
 */
T_API bool T_CALLTYPE TapDLC_IsOwned(const char *dlc_id);

#endif // TAPSDK_H
