# API Reference

This section provides detailed documentation for all classes, methods, and types in the TapTap PC SDK.

## Main Classes

| Class | Description |
|-------|-------------|
| [TapSdk](/api/tapsdk) | Main SDK class for initialization, authentication, and ownership |
| [CloudSave](/api/cloudsave) | Cloud save functionality for game saves |

## Exports

The package exports the following:

```typescript
import {
  // Classes
  TapSdk,
  CloudSave,
  
  // Constants
  EventId,
  SystemState,
  
  // Types
  type TapEvent,
  type AuthToken,
  type SdkError,
  type CloudSaveInfo,
  type CreateSaveRequest,
  type UpdateSaveRequest,
  // ... and more event types
} from 'tapsdk-pc';
```

## Quick Reference

### TapSdk Methods

| Method | Description |
|--------|-------------|
| `TapSdk.restartAppIfNecessary(clientId)` | Check if app needs restart |
| `new TapSdk(pubKey)` | Initialize the SDK |
| `TapSdk.isInitialized()` | Check if SDK is initialized |
| `sdk.getClientId()` | Get the client ID |
| `sdk.runCallbacks()` | Poll for events |
| `sdk.authorize(scopes)` | Request user authorization |
| `sdk.getOpenId()` | Get user's OpenID |
| `sdk.isGameOwned()` | Check if user owns game |
| `sdk.isDlcOwned(dlcId)` | Check if user owns DLC |
| `sdk.showDlcStore(dlcId)` | Open DLC store page |
| `sdk.shutdown()` | Shut down the SDK |

### CloudSave Methods

| Method | Description |
|--------|-------------|
| `CloudSave.get()` | Get singleton instance |
| `cloudSave.list(requestId)` | List all saves |
| `cloudSave.create(requestId, request)` | Create a new save |
| `cloudSave.update(requestId, request)` | Update existing save |
| `cloudSave.delete(requestId, uuid)` | Delete a save |
| `cloudSave.getData(requestId, uuid, fileId)` | Download save data |
| `cloudSave.getCover(requestId, uuid, fileId)` | Download save cover |

### Event Types

| Event ID | Constant | Description |
|----------|----------|-------------|
| 1 | `SYSTEM_STATE_CHANGED` | Platform online/offline/shutdown |
| 2002 | `AUTHORIZE_FINISHED` | Auth flow completed |
| 4001 | `GAME_PLAYABLE_STATUS_CHANGED` | Game ownership changed |
| 4002 | `DLC_PLAYABLE_STATUS_CHANGED` | DLC ownership changed |
| 6001 | `CLOUD_SAVE_LIST` | Save list response |
| 6002 | `CLOUD_SAVE_CREATE` | Save created |
| 6003 | `CLOUD_SAVE_UPDATE` | Save updated |
| 6004 | `CLOUD_SAVE_DELETE` | Save deleted |
| 6005 | `CLOUD_SAVE_GET_DATA` | Save data downloaded |
| 6006 | `CLOUD_SAVE_GET_COVER` | Cover image downloaded |
