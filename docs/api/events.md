# Events

The TapTap PC SDK uses an event-driven architecture. Events are automatically polled in a background thread and delivered via the `'event'` event on the `TapSdk` instance.

## Event Overview

```typescript
import { TapSdk, EventId, SystemState } from 'tapsdk-pc';

const sdk = new TapSdk('your_public_key');

sdk.on('event', (event) => {
  switch (event.eventId) {
    case EventId.SYSTEM_STATE_CHANGED:
      // Handle system state
      break;
    case EventId.AUTHORIZE_FINISHED:
      // Handle authorization result
      break;
    // ... other events
  }
});
```

## Event IDs

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
};
```

## System Events

### SystemStateChangedEvent

Fired when the TapTap platform state changes.

```typescript
interface SystemStateChangedEvent {
  eventId: 1;  // EventId.SYSTEM_STATE_CHANGED
  state: SystemState;
}
```

**System States:**
```typescript
const SystemState = {
  UNKNOWN: 0,
  PLATFORM_ONLINE: 1,   // TapTap client is online
  PLATFORM_OFFLINE: 2,  // TapTap client went offline
  PLATFORM_SHUTDOWN: 3, // TapTap client is shutting down
};
```

**Example:**
```typescript
if (event.eventId === EventId.SYSTEM_STATE_CHANGED) {
  switch (event.state) {
    case SystemState.PLATFORM_ONLINE:
      console.log('TapTap is online');
      break;
    case SystemState.PLATFORM_OFFLINE:
      console.log('TapTap went offline');
      break;
    case SystemState.PLATFORM_SHUTDOWN:
      console.log('TapTap is shutting down - save and exit');
      sdk.shutdown();
      process.exit(0);
      break;
  }
}
```

::: warning Important
When you receive `PLATFORM_SHUTDOWN`, you should save any unsaved data and exit your application gracefully.
:::

## Authorization Events

### AuthorizeFinishedEvent

Fired when user authorization completes (success, failure, or cancel).

```typescript
interface AuthorizeFinishedEvent {
  eventId: 2002;  // EventId.AUTHORIZE_FINISHED
  isCancel: boolean;
  error?: string;
  token?: AuthToken;
}

interface AuthToken {
  tokenType: string;
  kid: string;
  macKey: string;
  macAlgorithm: string;
  scope: string;
}
```

**Example:**
```typescript
if (event.eventId === EventId.AUTHORIZE_FINISHED) {
  if (event.token) {
    // Success
    console.log('Authorization successful!');
    console.log('Token type:', event.token.tokenType);
    console.log('OpenID:', sdk.getOpenId());
  } else if (event.isCancel) {
    // User cancelled
    console.log('User cancelled authorization');
  } else if (event.error) {
    // Error
    console.error('Authorization failed:', event.error);
  }
}
```

## Ownership Events

### GamePlayableStatusChangedEvent

Fired when the game's playable status changes.

```typescript
interface GamePlayableStatusChangedEvent {
  eventId: 4001;  // EventId.GAME_PLAYABLE_STATUS_CHANGED
  isPlayable: boolean;
}
```

**Example:**
```typescript
if (event.eventId === EventId.GAME_PLAYABLE_STATUS_CHANGED) {
  if (!event.isPlayable) {
    console.log('Game is no longer playable');
    // Handle appropriately (e.g., return to menu, exit game)
  }
}
```

### DlcPlayableStatusChangedEvent

Fired when a DLC's playable status changes.

```typescript
interface DlcPlayableStatusChangedEvent {
  eventId: 4002;  // EventId.DLC_PLAYABLE_STATUS_CHANGED
  dlcId: string;
  isPlayable: boolean;
}
```

**Example:**
```typescript
if (event.eventId === EventId.DLC_PLAYABLE_STATUS_CHANGED) {
  console.log(`DLC ${event.dlcId} playable: ${event.isPlayable}`);
  if (!event.isPlayable) {
    // Disable DLC content
    disableDlcContent(event.dlcId);
  }
}
```

## Cloud Save Events

### CloudSaveListEvent

Response to `cloudSave.list()`.

```typescript
interface CloudSaveListEvent {
  eventId: 6001;  // EventId.CLOUD_SAVE_LIST
  requestId: number;
  error?: SdkError;
  saves: CloudSaveInfo[];
}
```

### CloudSaveCreateEvent

Response to `cloudSave.create()`.

```typescript
interface CloudSaveCreateEvent {
  eventId: 6002;  // EventId.CLOUD_SAVE_CREATE
  requestId: number;
  error?: SdkError;
  save?: CloudSaveInfo;
}
```

### CloudSaveUpdateEvent

Response to `cloudSave.update()`.

```typescript
interface CloudSaveUpdateEvent {
  eventId: 6003;  // EventId.CLOUD_SAVE_UPDATE
  requestId: number;
  error?: SdkError;
  save?: CloudSaveInfo;
}
```

### CloudSaveDeleteEvent

Response to `cloudSave.delete()`.

```typescript
interface CloudSaveDeleteEvent {
  eventId: 6004;  // EventId.CLOUD_SAVE_DELETE
  requestId: number;
  error?: SdkError;
  uuid: string;
}
```

### CloudSaveGetDataEvent

Response to `cloudSave.getData()`.

```typescript
interface CloudSaveGetDataEvent {
  eventId: 6005;  // EventId.CLOUD_SAVE_GET_DATA
  requestId: number;
  error?: SdkError;
  data: Buffer;
}
```

### CloudSaveGetCoverEvent

Response to `cloudSave.getCover()`.

```typescript
interface CloudSaveGetCoverEvent {
  eventId: 6006;  // EventId.CLOUD_SAVE_GET_COVER
  requestId: number;
  error?: SdkError;
  data: Buffer;
}
```

## Error Handling

Cloud save events include an optional `error` field:

```typescript
interface SdkError {
  code: number;
  message: string;
}
```

**Example:**
```typescript
if (event.eventId === EventId.CLOUD_SAVE_CREATE) {
  if (event.error) {
    console.error(`Error ${event.error.code}: ${event.error.message}`);
  } else if (event.save) {
    console.log('Save created:', event.save.uuid);
  }
}
```

## Unknown Events

If an unknown event is received:

```typescript
interface UnknownEvent {
  eventId: number;
}
```

This can happen if the native SDK sends events that this binding doesn't recognize yet.

## TapEvent Union Type

All event types are combined into a union type:

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

TypeScript will narrow the type when you check `event.eventId`.
