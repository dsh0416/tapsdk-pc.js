/**
 * Type definitions for TapTap PC SDK
 */

/** Authorization token returned after successful authorization */
export interface AuthToken {
  tokenType: string;
  kid: string;
  macKey: string;
  macAlgorithm: string;
  scope: string;
}

/** SDK Error info */
export interface SdkError {
  code: number;
  message: string;
}

/** Cloud save information */
export interface CloudSaveInfo {
  uuid: string;
  fileId: string;
  name: string;
  saveSize: number;
  coverSize: number;
  summary?: string;
  extra?: string;
  playtime: number;
  createdTime: number;
  modifiedTime: number;
}

/** Achievement information returned from achievement events */
export interface AchievementInfo {
  id: string;
  name: string;
  currentSteps: number;
  newlyUnlocked: boolean;
}

/** Compliance payment limit response */
export interface PaymentLimitResponse {
  allow: boolean;
  title: string;
  description: string;
}

/** Leaderboard collection type */
export const LeaderboardCollection = {
  PUBLIC: 0,
  FRIENDS: 1,
} as const;

export type LeaderboardCollection =
  (typeof LeaderboardCollection)[keyof typeof LeaderboardCollection];

/** Leaderboard score item to submit */
export interface LeaderboardScoreItem {
  leaderboardId: string;
  score: number;
}

/** Leaderboard period information */
export interface LeaderboardPeriod {
  periodToken: string;
  display: string;
}

/** Leaderboard user information */
export interface LeaderboardUserInfo {
  openId: string;
  unionId: string;
  name: string;
  avatar: string;
}

/** Leaderboard score information */
export interface LeaderboardScore {
  rank: number;
  score: number;
  user: LeaderboardUserInfo;
  scoreSubmittedTime: number;
}

/** Leaderboard information */
export interface LeaderboardInfo {
  id: string;
  name: string;
  period?: LeaderboardPeriod;
  availablePeriods: LeaderboardPeriod[];
}

/** Request to load leaderboard scores */
export interface LoadScoresRequest {
  leaderboardId: string;
  collection: LeaderboardCollection;
  continuationToken?: string;
  periodToken?: string;
}

/** Request to load current user's score */
export interface LoadMyScoresRequest {
  leaderboardId: string;
  collection: LeaderboardCollection;
  periodToken?: string;
}

/** Request to load scores near current user */
export interface LoadMyCenteredScoresRequest {
  leaderboardId: string;
  collection: LeaderboardCollection;
  maxResults: number;
}

/** Request to open leaderboards page */
export interface ShowLeaderboardRequest {
  leaderboardId: string;
  collection: LeaderboardCollection;
}

/** Online game event ID constants */
export const OnlineGameEventId = {
  UNKNOWN: 0,
  CONNECT_RESPONSE: 1,
  DISCONNECT_RESPONSE: 2,
  CREATE_ROOM_RESPONSE: 3,
  MATCH_ROOM_RESPONSE: 4,
  GET_ROOM_LIST_RESPONSE: 5,
  JOIN_ROOM_RESPONSE: 6,
  LEAVE_ROOM_RESPONSE: 7,
  UPDATE_PLAYER_CUSTOM_STATUS_RESPONSE: 8,
  UPDATE_PLAYER_CUSTOM_PROPERTIES_RESPONSE: 9,
  UPDATE_ROOM_PROPERTIES_RESPONSE: 10,
  SEND_CUSTOM_MESSAGE_RESPONSE: 11,
  KICK_ROOM_PLAYER_RESPONSE: 12,
  START_FRAME_SYNC_RESPONSE: 13,
  SEND_FRAME_INPUT_RESPONSE: 14,
  STOP_FRAME_SYNC_RESPONSE: 15,
  SERVICE_ERROR_NOTIFICATION: 1000,
  DISCONNECT_NOTIFICATION: 1001,
  PLAYER_OFFLINE_NOTIFICATION: 1002,
  ENTER_ROOM_NOTIFICATION: 1003,
  LEAVE_ROOM_NOTIFICATION: 1004,
  PLAYER_CUSTOM_STATUS_NOTIFICATION: 1005,
  PLAYER_CUSTOM_PROPERTIES_NOTIFICATION: 1006,
  ROOM_PROPERTIES_NOTIFICATION: 1007,
  CUSTOM_MESSAGE_NOTIFICATION: 1008,
  ROOM_PLAYER_KICKED_NOTIFICATION: 1009,
  FRAME_SYNC_START_NOTIFICATION: 1010,
  FRAME_NOTIFICATION: 1011,
  FRAME_SYNC_STOP_NOTIFICATION: 1012,
} as const;

export type OnlineGameEventId = (typeof OnlineGameEventId)[keyof typeof OnlineGameEventId];

/** Online game custom message receiver type */
export const OnlineGameMessageReceiverType = {
  ROOM: 0,
  PLAYERS: 1,
} as const;

export type OnlineGameMessageReceiverType =
  (typeof OnlineGameMessageReceiverType)[keyof typeof OnlineGameMessageReceiverType];

/** Online game room match parameter */
export interface OnlineGameMatchParam {
  key: string;
  value: string;
}

/** Online game room configuration */
export interface OnlineGameRoomConfig {
  maxPlayerCount: number;
  roomType: string;
  matchParams?: OnlineGameMatchParam[];
  name?: string;
  customProperties?: string;
}

/** Online game player configuration */
export interface OnlineGamePlayerConfig {
  customStatus: number;
  customProperties?: string;
}

/** Online game create or match room request */
export interface OnlineGameRoomRequest {
  room: OnlineGameRoomConfig;
  player: OnlineGamePlayerConfig;
}

/** Online game get room list request */
export interface OnlineGameGetRoomListRequest {
  roomType?: string;
  offset?: number;
  limit?: number;
}

/** Online game join room request */
export interface OnlineGameJoinRoomRequest {
  roomId: string;
  player: OnlineGamePlayerConfig;
}

/** Online game update room properties request */
export interface OnlineGameUpdateRoomPropertiesRequest {
  name?: string;
  customProperties?: string;
}

/** Online game send custom message request */
export interface OnlineGameSendCustomMessageRequest {
  msg: string;
  receiverType: OnlineGameMessageReceiverType;
  receivers?: string[];
}

/** Request to create a cloud save */
export interface CreateSaveRequest {
  /** Save name (max 60 bytes, no Chinese characters) */
  name: string;
  /** Save description (max 500 bytes) */
  summary: string;
  /** Developer-defined extra data (max 1000 bytes, optional) */
  extra?: string;
  /** Game playtime in seconds */
  playtime: number;
  /** Path to the save data file (max 10MB) */
  dataFilePath: string;
  /** Path to the cover image file (max 512KB, optional) */
  coverFilePath?: string;
}

/** Request to update a cloud save */
export interface UpdateSaveRequest {
  /** UUID of the cloud save to update */
  uuid: string;
  /** Save name (max 60 bytes, no Chinese characters) */
  name: string;
  /** Save description (max 500 bytes) */
  summary: string;
  /** Developer-defined extra data (max 1000 bytes, optional) */
  extra?: string;
  /** Game playtime in seconds */
  playtime: number;
  /** Path to the save data file (max 10MB) */
  dataFilePath: string;
  /** Path to the cover image file (max 512KB, optional) */
  coverFilePath?: string;
}

// Event Types

/** System state changed event */
export interface SystemStateChangedEvent {
  eventId: typeof EventId.SYSTEM_STATE_CHANGED;
  state: SystemState;
}

/** Authorization finished event */
export interface AuthorizeFinishedEvent {
  eventId: typeof EventId.AUTHORIZE_FINISHED;
  isCancel: boolean;
  error?: string;
  token?: AuthToken;
}

/** Game playable status changed event */
export interface GamePlayableStatusChangedEvent {
  eventId: typeof EventId.GAME_PLAYABLE_STATUS_CHANGED;
  isPlayable: boolean;
}

/** DLC playable status changed event */
export interface DlcPlayableStatusChangedEvent {
  eventId: typeof EventId.DLC_PLAYABLE_STATUS_CHANGED;
  dlcId: string;
  isPlayable: boolean;
}

/** Cloud save list event */
export interface CloudSaveListEvent {
  eventId: typeof EventId.CLOUD_SAVE_LIST;
  requestId: number;
  error?: SdkError;
  saves: CloudSaveInfo[];
}

/** Cloud save create event */
export interface CloudSaveCreateEvent {
  eventId: typeof EventId.CLOUD_SAVE_CREATE;
  requestId: number;
  error?: SdkError;
  save?: CloudSaveInfo;
}

/** Cloud save update event */
export interface CloudSaveUpdateEvent {
  eventId: typeof EventId.CLOUD_SAVE_UPDATE;
  requestId: number;
  error?: SdkError;
  save?: CloudSaveInfo;
}

/** Cloud save delete event */
export interface CloudSaveDeleteEvent {
  eventId: typeof EventId.CLOUD_SAVE_DELETE;
  requestId: number;
  error?: SdkError;
  uuid: string;
}

/** Cloud save get data event */
export interface CloudSaveGetDataEvent {
  eventId: typeof EventId.CLOUD_SAVE_GET_DATA;
  requestId: number;
  error?: SdkError;
  data: Buffer;
}

/** Cloud save get cover event */
export interface CloudSaveGetCoverEvent {
  eventId: typeof EventId.CLOUD_SAVE_GET_COVER;
  requestId: number;
  error?: SdkError;
  data: Buffer;
}

/** Achievement unlock event */
export interface AchievementUnlockEvent {
  eventId: typeof EventId.ACHIEVEMENT_UNLOCK;
  requestId: number;
  error?: SdkError;
  achievement?: AchievementInfo;
  platinumAchievement?: AchievementInfo;
}

/** Achievement increment event */
export interface AchievementIncrementEvent {
  eventId: typeof EventId.ACHIEVEMENT_INCREMENT;
  requestId: number;
  error?: SdkError;
  achievement?: AchievementInfo;
  platinumAchievement?: AchievementInfo;
}

/** Compliance action */
export interface ComplianceAction {
  actionType: number;
  title: string;
  description: string;
  displayDurationSeconds: number;
}

/** Compliance real-name response event */
export interface ComplianceEnsureRealNameEvent {
  eventId: typeof EventId.COMPLIANCE_ENSURE_REAL_NAME;
  requestId: number;
  error?: SdkError;
  status: number;
}

/** Compliance anti-addiction actions event */
export interface ComplianceActionsEvent {
  eventId: typeof EventId.COMPLIANCE_ACTIONS_EVENT;
  actions: ComplianceAction[];
}

/** Leaderboard submit score result data */
export interface LeaderboardSubmitScoreResultData {
  newBest: boolean;
  rawScore: number;
}

/** Leaderboard submit score result */
export interface LeaderboardSubmitScoreResult {
  leaderboardId: string;
  periodToken: string;
  scoreResult?: LeaderboardSubmitScoreResultData;
  openId: string;
  unionId: string;
}

/** Leaderboard submit scores event */
export interface LeaderboardSubmitScoresEvent {
  eventId: typeof EventId.LEADERBOARD_SUBMIT_SCORES;
  requestId: number;
  error?: SdkError;
  results: LeaderboardSubmitScoreResult[];
}

/** Leaderboard load scores event */
export interface LeaderboardLoadScoresEvent {
  eventId: typeof EventId.LEADERBOARD_LOAD_SCORES;
  requestId: number;
  error?: SdkError;
  leaderboard?: LeaderboardInfo;
  scores: LeaderboardScore[];
  continuationToken?: string;
  isTruncated: boolean;
}

/** Leaderboard load current user's score event */
export interface LeaderboardLoadMyScoresEvent {
  eventId: typeof EventId.LEADERBOARD_LOAD_MY_SCORES;
  requestId: number;
  error?: SdkError;
  leaderboard?: LeaderboardInfo;
  score?: LeaderboardScore;
}

/** Leaderboard load scores near current user event */
export interface LeaderboardLoadMyCenteredScoresEvent {
  eventId: typeof EventId.LEADERBOARD_LOAD_MY_CENTERED_SCORES;
  requestId: number;
  error?: SdkError;
  leaderboard?: LeaderboardInfo;
  scores: LeaderboardScore[];
}

/** Online game player information */
export interface OnlineGamePlayerInfo {
  id: string;
  status: number;
  customStatus: number;
  customProperties: string;
}

/** Online game room information */
export interface OnlineGameRoomInfo {
  id: string;
  name: string;
  roomType: string;
  ownerId: string;
  status: number;
  customProperties: string;
  maxPlayerCount: number;
  playerCount: number;
  players: OnlineGamePlayerInfo[];
  createTime: number;
}

/** Online game room basic information */
export interface OnlineGameRoomBasicInfo {
  id: string;
  name: string;
  roomType: string;
  status: number;
  customProperties: string;
  maxPlayerCount: number;
  playerCount: number;
  createTime: number;
}

/** Online game event */
export interface OnlineGameEvent {
  eventId: typeof EventId.ONLINE_GAME_EVENT;
  requestId: number;
  error?: SdkError;
  onlineGameEventId: OnlineGameEventId;
  data?: unknown;
}

/** Unknown event */
export interface UnknownEvent {
  eventId: number;
}

/** Union type of all possible events */
export type TapEvent =
  | SystemStateChangedEvent
  | AuthorizeFinishedEvent
  | GamePlayableStatusChangedEvent
  | DlcPlayableStatusChangedEvent
  | CloudSaveListEvent
  | CloudSaveCreateEvent
  | CloudSaveUpdateEvent
  | CloudSaveDeleteEvent
  | CloudSaveGetDataEvent
  | CloudSaveGetCoverEvent
  | AchievementUnlockEvent
  | AchievementIncrementEvent
  | ComplianceEnsureRealNameEvent
  | ComplianceActionsEvent
  | LeaderboardSubmitScoresEvent
  | LeaderboardLoadScoresEvent
  | LeaderboardLoadMyScoresEvent
  | LeaderboardLoadMyCenteredScoresEvent
  | OnlineGameEvent
  | UnknownEvent;

// Constants

/** Event ID constants */
export const EventId = {
  UNKNOWN: 0,
  SYSTEM_STATE_CHANGED: 1,
  AUTHORIZE_FINISHED: 2002,
  GAME_PLAYABLE_STATUS_CHANGED: 4001,
  DLC_PLAYABLE_STATUS_CHANGED: 4002,
  CLOUD_SAVE_LIST: 6001,
  CLOUD_SAVE_CREATE: 6002,
  CLOUD_SAVE_UPDATE: 6003,
  CLOUD_SAVE_DELETE: 6004,
  CLOUD_SAVE_GET_DATA: 6005,
  CLOUD_SAVE_GET_COVER: 6006,
  ACHIEVEMENT_UNLOCK: 7001,
  ACHIEVEMENT_INCREMENT: 7002,
  COMPLIANCE_ENSURE_REAL_NAME: 8001,
  COMPLIANCE_ACTIONS_EVENT: 8002,
  LEADERBOARD_SUBMIT_SCORES: 9001,
  LEADERBOARD_LOAD_SCORES: 9002,
  LEADERBOARD_LOAD_MY_SCORES: 9003,
  LEADERBOARD_LOAD_MY_CENTERED_SCORES: 9004,
  ONLINE_GAME_EVENT: 10001,
} as const;

/** System state constants */
export const SystemState = {
  UNKNOWN: 0,
  PLATFORM_ONLINE: 1,
  PLATFORM_OFFLINE: 2,
  PLATFORM_SHUTDOWN: 3,
} as const;

export type SystemState = (typeof SystemState)[keyof typeof SystemState];

/** Event map for typed EventEmitter usage */
export interface TapSdkEvents {
  event: [TapEvent];
}
