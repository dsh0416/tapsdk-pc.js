# Types

This page documents all TypeScript types exported by the TapTap PC SDK.

## Import

```typescript
import type {
  TapEvent,
  AuthToken,
  SdkError,
  CloudSaveInfo,
  CreateSaveRequest,
  UpdateSaveRequest,
  SystemStateChangedEvent,
  AuthorizeFinishedEvent,
  GamePlayableStatusChangedEvent,
  DlcPlayableStatusChangedEvent,
  CloudSaveListEvent,
  CloudSaveCreateEvent,
  CloudSaveUpdateEvent,
  CloudSaveDeleteEvent,
  CloudSaveGetDataEvent,
  CloudSaveGetCoverEvent,
  UnknownEvent,
} from 'tapsdk-pc';
```

## Authentication Types

### AuthToken

Authorization token returned after successful authorization.

```typescript
interface AuthToken {
  /** Token type (e.g., "mac") */
  tokenType: string;
  /** Key ID */
  kid: string;
  /** MAC key for signing requests */
  macKey: string;
  /** MAC algorithm (e.g., "hmac-sha-256") */
  macAlgorithm: string;
  /** Granted permission scopes */
  scope: string;
}
```

## Error Types

### SdkError

SDK error information.

```typescript
interface SdkError {
  /** Error code */
  code: number;
  /** Error message */
  message: string;
}
```

## Cloud Save Types

### CloudSaveInfo

Information about a cloud save.

```typescript
interface CloudSaveInfo {
  /** Unique identifier for the save */
  uuid: string;
  /** File ID for downloading data/cover */
  fileId: string;
  /** Save name */
  name: string;
  /** Size of save data in bytes */
  saveSize: number;
  /** Size of cover image in bytes */
  coverSize: number;
  /** Save description */
  summary?: string;
  /** Developer-defined extra data */
  extra?: string;
  /** Game playtime in seconds */
  playtime: number;
  /** Creation timestamp (Unix epoch) */
  createdTime: number;
  /** Last modified timestamp (Unix epoch) */
  modifiedTime: number;
}
```

### CreateSaveRequest

Request parameters for creating a cloud save.

```typescript
interface CreateSaveRequest {
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
```

### UpdateSaveRequest

Request parameters for updating a cloud save.

```typescript
interface UpdateSaveRequest {
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
```

## Event Types

### SystemStateChangedEvent

```typescript
interface SystemStateChangedEvent {
  eventId: typeof EventId.SYSTEM_STATE_CHANGED; // 1
  state: SystemState;
}
```

### AuthorizeFinishedEvent

```typescript
interface AuthorizeFinishedEvent {
  eventId: typeof EventId.AUTHORIZE_FINISHED; // 2002
  isCancel: boolean;
  error?: string;
  token?: AuthToken;
}
```

### GamePlayableStatusChangedEvent

```typescript
interface GamePlayableStatusChangedEvent {
  eventId: typeof EventId.GAME_PLAYABLE_STATUS_CHANGED; // 4001
  isPlayable: boolean;
}
```

### DlcPlayableStatusChangedEvent

```typescript
interface DlcPlayableStatusChangedEvent {
  eventId: typeof EventId.DLC_PLAYABLE_STATUS_CHANGED; // 4002
  dlcId: string;
  isPlayable: boolean;
}
```

### CloudSaveListEvent

```typescript
interface CloudSaveListEvent {
  eventId: typeof EventId.CLOUD_SAVE_LIST; // 6001
  requestId: number;
  error?: SdkError;
  saves: CloudSaveInfo[];
}
```

### CloudSaveCreateEvent

```typescript
interface CloudSaveCreateEvent {
  eventId: typeof EventId.CLOUD_SAVE_CREATE; // 6002
  requestId: number;
  error?: SdkError;
  save?: CloudSaveInfo;
}
```

### CloudSaveUpdateEvent

```typescript
interface CloudSaveUpdateEvent {
  eventId: typeof EventId.CLOUD_SAVE_UPDATE; // 6003
  requestId: number;
  error?: SdkError;
  save?: CloudSaveInfo;
}
```

### CloudSaveDeleteEvent

```typescript
interface CloudSaveDeleteEvent {
  eventId: typeof EventId.CLOUD_SAVE_DELETE; // 6004
  requestId: number;
  error?: SdkError;
  uuid: string;
}
```

### CloudSaveGetDataEvent

```typescript
interface CloudSaveGetDataEvent {
  eventId: typeof EventId.CLOUD_SAVE_GET_DATA; // 6005
  requestId: number;
  error?: SdkError;
  data: Buffer;
}
```

### CloudSaveGetCoverEvent

```typescript
interface CloudSaveGetCoverEvent {
  eventId: typeof EventId.CLOUD_SAVE_GET_COVER; // 6006
  requestId: number;
  error?: SdkError;
  data: Buffer;
}
```

### UnknownEvent

```typescript
interface UnknownEvent {
  eventId: number;
}
```

### TapEvent

Union type of all possible events.

```typescript
type TapEvent =
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
```

## Constants

### EventId

Event ID constants.

```typescript
const EventId = {
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
```

### SystemState

System state constants.

```typescript
const SystemState = {
  UNKNOWN: 0,
  PLATFORM_ONLINE: 1,
  PLATFORM_OFFLINE: 2,
  PLATFORM_SHUTDOWN: 3,
} as const;

type SystemState = (typeof SystemState)[keyof typeof SystemState];
```

## Type Guards

You can use type guards to narrow event types:

```typescript
function isAuthEvent(event: TapEvent): event is AuthorizeFinishedEvent {
  return event.eventId === EventId.AUTHORIZE_FINISHED;
}

function isCloudSaveEvent(event: TapEvent): boolean {
  return event.eventId >= 6001 && event.eventId <= 6006;
}

// Usage
const events = sdk.runCallbacks();
for (const event of events) {
  if (isAuthEvent(event)) {
    // TypeScript knows event is AuthorizeFinishedEvent here
    console.log('Token:', event.token);
  }
}
```
