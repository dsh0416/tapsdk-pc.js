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
