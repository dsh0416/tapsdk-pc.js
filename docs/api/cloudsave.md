# CloudSave

The CloudSave class provides cloud save functionality for storing and retrieving game saves from TapTap servers.

## Import

```typescript
import { CloudSave } from 'tapsdk-pc';
```

## Getting Started

CloudSave uses an asynchronous, event-driven pattern. You call methods with a `requestId`, then receive results through events emitted by the `TapSdk` instance.

```typescript
import { TapSdk, CloudSave, EventId } from 'tapsdk-pc';

const sdk = new TapSdk('your_public_key');
const cloudSave = CloudSave.get();

// Listen for cloud save events
sdk.on('event', (event) => {
  if (event.eventId === EventId.CLOUD_SAVE_LIST && event.requestId === 1) {
    if (event.error) {
      console.error('Failed:', event.error.message);
    } else {
      console.log(`Found ${event.saves.length} saves`);
    }
  }
});

// Make a request
cloudSave.list(1); // requestId = 1
```

## Static Methods

### get()

Get the CloudSave singleton instance.

```typescript
static get(): CloudSave
```

**Returns:** CloudSave instance

**Throws:** Error if SDK is not initialized

**Example:**
```typescript
const cloudSave = CloudSave.get();
```

## Instance Methods

### list()

Request the list of cloud saves.

```typescript
list(requestId: number): void
```

**Parameters:**
- `requestId` - A unique ID to identify this request in the callback

**Event:** `CloudSaveListEvent` (EventId: `CLOUD_SAVE_LIST`)

**Example:**
```typescript
cloudSave.list(1);

// Handle via event listener
sdk.on('event', (event) => {
  if (event.eventId === EventId.CLOUD_SAVE_LIST) {
    for (const save of event.saves) {
      console.log(`${save.name} - ${save.saveSize} bytes`);
    }
  }
});
```

---

### create()

Create a new cloud save.

```typescript
create(requestId: number, request: CreateSaveRequest): void
```

**Parameters:**
- `requestId` - A unique ID to identify this request in the callback
- `request` - The create request parameters

**Event:** `CloudSaveCreateEvent` (EventId: `CLOUD_SAVE_CREATE`)

**CreateSaveRequest:**
```typescript
interface CreateSaveRequest {
  name: string;           // Save name (max 60 bytes, no Chinese)
  summary: string;        // Description (max 500 bytes)
  extra?: string;         // Developer data (max 1000 bytes)
  playtime: number;       // Game playtime in seconds
  dataFilePath: string;   // Path to save file (max 10MB)
  coverFilePath?: string; // Path to cover image (max 512KB)
}
```

**Example:**
```typescript
cloudSave.create(2, {
  name: 'save1',
  summary: 'Chapter 1 - Forest Temple',
  playtime: 3600, // 1 hour
  dataFilePath: './saves/savegame.dat',
  coverFilePath: './saves/screenshot.png',
});

// Handle via event listener
sdk.on('event', (event) => {
  if (event.eventId === EventId.CLOUD_SAVE_CREATE) {
    if (event.save) {
      console.log('Save created with UUID:', event.save.uuid);
    }
  }
});
```

---

### update()

Update an existing cloud save.

```typescript
update(requestId: number, request: UpdateSaveRequest): void
```

**Parameters:**
- `requestId` - A unique ID to identify this request in the callback
- `request` - The update request parameters

**Event:** `CloudSaveUpdateEvent` (EventId: `CLOUD_SAVE_UPDATE`)

**UpdateSaveRequest:**
```typescript
interface UpdateSaveRequest {
  uuid: string;           // UUID of save to update
  name: string;           // Save name (max 60 bytes, no Chinese)
  summary: string;        // Description (max 500 bytes)
  extra?: string;         // Developer data (max 1000 bytes)
  playtime: number;       // Game playtime in seconds
  dataFilePath: string;   // Path to save file (max 10MB)
  coverFilePath?: string; // Path to cover image (max 512KB)
}
```

**Example:**
```typescript
cloudSave.update(3, {
  uuid: 'existing-save-uuid',
  name: 'save1',
  summary: 'Chapter 3 - Boss Defeated',
  playtime: 7200, // 2 hours
  dataFilePath: './saves/savegame.dat',
});
```

---

### delete()

Delete a cloud save.

```typescript
delete(requestId: number, uuid: string): void
```

**Parameters:**
- `requestId` - A unique ID to identify this request in the callback
- `uuid` - The unique ID of the cloud save to delete

**Event:** `CloudSaveDeleteEvent` (EventId: `CLOUD_SAVE_DELETE`)

**Example:**
```typescript
cloudSave.delete(4, 'save-uuid-to-delete');

// Handle via event listener
sdk.on('event', (event) => {
  if (event.eventId === EventId.CLOUD_SAVE_DELETE) {
    if (!event.error) {
      console.log('Deleted save:', event.uuid);
    }
  }
});
```

---

### getData()

Download the data file for a cloud save.

```typescript
getData(requestId: number, uuid: string, fileId: string): void
```

**Parameters:**
- `requestId` - A unique ID to identify this request in the callback
- `uuid` - The unique ID of the cloud save
- `fileId` - The file ID from CloudSaveInfo

**Event:** `CloudSaveGetDataEvent` (EventId: `CLOUD_SAVE_GET_DATA`)

**Example:**
```typescript
// First, get save info from list()
const saveInfo = event.saves[0];

// Then download the data
cloudSave.getData(5, saveInfo.uuid, saveInfo.fileId);

// Handle via event listener
sdk.on('event', (event) => {
  if (event.eventId === EventId.CLOUD_SAVE_GET_DATA) {
    if (event.data) {
      // event.data is a Buffer containing the save file
      fs.writeFileSync('./saves/downloaded.dat', event.data);
    }
  }
});
```

---

### getCover()

Download the cover image for a cloud save.

```typescript
getCover(requestId: number, uuid: string, fileId: string): void
```

**Parameters:**
- `requestId` - A unique ID to identify this request in the callback
- `uuid` - The unique ID of the cloud save
- `fileId` - The file ID from CloudSaveInfo

**Event:** `CloudSaveGetCoverEvent` (EventId: `CLOUD_SAVE_GET_COVER`)

**Example:**
```typescript
cloudSave.getCover(6, saveInfo.uuid, saveInfo.fileId);

// Handle via event listener
sdk.on('event', (event) => {
  if (event.eventId === EventId.CLOUD_SAVE_GET_COVER) {
    if (event.data) {
      fs.writeFileSync('./saves/cover.png', event.data);
    }
  }
});
```

## CloudSaveInfo

Information about a cloud save returned from list operations:

```typescript
interface CloudSaveInfo {
  uuid: string;         // Unique identifier
  fileId: string;       // File ID for downloading
  name: string;         // Save name
  saveSize: number;     // Size of save data in bytes
  coverSize: number;    // Size of cover image in bytes
  summary?: string;     // Save description
  extra?: string;       // Developer-defined data
  playtime: number;     // Playtime in seconds
  createdTime: number;  // Creation timestamp
  modifiedTime: number; // Last modified timestamp
}
```

## Complete Example

```typescript
import { TapSdk, CloudSave, EventId } from 'tapsdk-pc';
import * as fs from 'fs';

const sdk = new TapSdk('your_public_key');
const cloudSave = CloudSave.get();

// Track pending requests
const pendingRequests = new Map<number, string>();
let nextRequestId = 1;

function listSaves() {
  const id = nextRequestId++;
  pendingRequests.set(id, 'list');
  cloudSave.list(id);
}

function createSave(name: string, dataPath: string) {
  const id = nextRequestId++;
  pendingRequests.set(id, 'create');
  cloudSave.create(id, {
    name,
    summary: `Save created at ${new Date().toISOString()}`,
    playtime: 3600,
    dataFilePath: dataPath,
  });
}

function downloadSave(save: CloudSaveInfo) {
  const id = nextRequestId++;
  pendingRequests.set(id, 'download');
  cloudSave.getData(id, save.uuid, save.fileId);
}

// Handle events (automatically polled in background)
sdk.on('event', (event) => {
  switch (event.eventId) {
    case EventId.CLOUD_SAVE_LIST:
      console.log(`Found ${event.saves.length} cloud saves:`);
      for (const save of event.saves) {
        console.log(`  - ${save.name} (${save.uuid})`);
      }
      break;
      
    case EventId.CLOUD_SAVE_CREATE:
      if (event.save) {
        console.log('Created save:', event.save.uuid);
      } else if (event.error) {
        console.error('Create failed:', event.error.message);
      }
      break;
      
    case EventId.CLOUD_SAVE_GET_DATA:
      if (event.data) {
        fs.writeFileSync('./downloaded_save.dat', event.data);
        console.log('Save downloaded successfully');
      }
      break;
  }
});

// Start
listSaves();
```

## Limits and Constraints

| Item | Limit |
|------|-------|
| Save name | Max 60 bytes, no Chinese characters |
| Summary | Max 500 bytes |
| Extra data | Max 1000 bytes |
| Save file size | Max 10 MB |
| Cover image size | Max 512 KB |
