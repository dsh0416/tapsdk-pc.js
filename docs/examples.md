# Examples

This page provides complete, runnable examples for common use cases.

## Basic Integration

A minimal example showing SDK initialization and event handling.

```typescript
import { TapSdk, EventId, SystemState } from 'tapsdk-pc';

// Configuration
const CLIENT_ID = 'your_client_id';
const PUBLIC_KEY = 'your_public_key';

// Check restart requirement
if (TapSdk.restartAppIfNecessary(CLIENT_ID)) {
  console.log('Restarting via TapTap...');
  process.exit(0);
}

// Initialize SDK
const sdk = new TapSdk(PUBLIC_KEY);
console.log('SDK initialized');

// Verify game ownership
if (!sdk.isGameOwned()) {
  console.error('User does not own this game');
  process.exit(1);
}
console.log('Game ownership verified');

// Game loop
let running = true;

function gameLoop() {
  if (!running) return;
  
  // Poll for events
  const events = sdk.runCallbacks();
  
  for (const event of events) {
    if (event.eventId === EventId.SYSTEM_STATE_CHANGED) {
      if (event.state === SystemState.PLATFORM_SHUTDOWN) {
        console.log('TapTap shutting down, exiting...');
        running = false;
      }
    }
  }
  
  // Continue loop
  setTimeout(gameLoop, 16);
}

// Start game loop
gameLoop();

// Cleanup on exit
process.on('SIGINT', () => {
  console.log('Shutting down...');
  sdk.shutdown();
  process.exit(0);
});
```

## User Authentication

Complete example with user login flow.

```typescript
import { TapSdk, EventId, SystemState } from 'tapsdk-pc';

const CLIENT_ID = 'your_client_id';
const PUBLIC_KEY = 'your_public_key';

if (TapSdk.restartAppIfNecessary(CLIENT_ID)) {
  process.exit(0);
}

const sdk = new TapSdk(PUBLIC_KEY);

if (!sdk.isGameOwned()) {
  console.error('Game not owned');
  process.exit(1);
}

// Request authorization
console.log('Requesting user authorization...');
sdk.authorize('public_profile');

// Handle events
let authorized = false;

function update() {
  const events = sdk.runCallbacks();
  
  for (const event of events) {
    switch (event.eventId) {
      case EventId.AUTHORIZE_FINISHED:
        if (event.token) {
          authorized = true;
          const openId = sdk.getOpenId();
          console.log('Authorization successful!');
          console.log('User OpenID:', openId);
          console.log('Token type:', event.token.tokenType);
          console.log('Scopes:', event.token.scope);
        } else if (event.isCancel) {
          console.log('User cancelled authorization');
        } else if (event.error) {
          console.error('Authorization error:', event.error);
        }
        break;
        
      case EventId.SYSTEM_STATE_CHANGED:
        console.log('System state:', event.state);
        if (event.state === SystemState.PLATFORM_SHUTDOWN) {
          sdk.shutdown();
          process.exit(0);
        }
        break;
    }
  }
  
  setTimeout(update, 100);
}

update();
```

## DLC Management

Example showing how to check and manage DLC ownership.

```typescript
import { TapSdk, EventId } from 'tapsdk-pc';

const CLIENT_ID = 'your_client_id';
const PUBLIC_KEY = 'your_public_key';

// DLC identifiers
const DLC_EXPANSION = 'expansion_pack_1';
const DLC_SOUNDTRACK = 'soundtrack_dlc';

if (TapSdk.restartAppIfNecessary(CLIENT_ID)) {
  process.exit(0);
}

const sdk = new TapSdk(PUBLIC_KEY);

if (!sdk.isGameOwned()) {
  process.exit(1);
}

// Check DLC ownership
console.log('Checking DLC ownership...');

const dlcs = [
  { id: DLC_EXPANSION, name: 'Expansion Pack' },
  { id: DLC_SOUNDTRACK, name: 'Soundtrack' },
];

for (const dlc of dlcs) {
  const owned = sdk.isDlcOwned(dlc.id);
  console.log(`${dlc.name}: ${owned ? 'Owned' : 'Not owned'}`);
  
  if (!owned) {
    // Optionally open store page
    // sdk.showDlcStore(dlc.id);
  }
}

// Listen for ownership changes
function update() {
  const events = sdk.runCallbacks();
  
  for (const event of events) {
    if (event.eventId === EventId.DLC_PLAYABLE_STATUS_CHANGED) {
      const dlc = dlcs.find(d => d.id === event.dlcId);
      const name = dlc?.name || event.dlcId;
      
      if (event.isPlayable) {
        console.log(`${name} is now available!`);
        // Enable DLC content
      } else {
        console.log(`${name} is no longer available`);
        // Disable DLC content
      }
    }
  }
  
  setTimeout(update, 100);
}

update();
```

## Cloud Save Integration

Complete cloud save example with all operations.

```typescript
import { TapSdk, CloudSave, EventId } from 'tapsdk-pc';
import * as fs from 'fs';
import * as path from 'path';

const CLIENT_ID = 'your_client_id';
const PUBLIC_KEY = 'your_public_key';

if (TapSdk.restartAppIfNecessary(CLIENT_ID)) {
  process.exit(0);
}

const sdk = new TapSdk(PUBLIC_KEY);

if (!sdk.isGameOwned()) {
  process.exit(1);
}

const cloudSave = CloudSave.get();

// Request ID management
let nextRequestId = 1;
const pendingRequests = new Map<number, {
  type: string;
  callback: (event: any) => void;
}>();

function request(type: string, callback: (event: any) => void): number {
  const id = nextRequestId++;
  pendingRequests.set(id, { type, callback });
  return id;
}

// Cloud save operations
async function listSaves(): Promise<void> {
  return new Promise((resolve, reject) => {
    const id = request('list', (event) => {
      if (event.error) {
        console.error('List failed:', event.error.message);
        reject(new Error(event.error.message));
      } else {
        console.log(`\nFound ${event.saves.length} cloud saves:`);
        for (const save of event.saves) {
          const date = new Date(save.modifiedTime * 1000);
          const hours = Math.floor(save.playtime / 3600);
          console.log(`  - ${save.name}`);
          console.log(`    UUID: ${save.uuid}`);
          console.log(`    Size: ${(save.saveSize / 1024).toFixed(1)} KB`);
          console.log(`    Playtime: ${hours}h`);
          console.log(`    Modified: ${date.toLocaleString()}`);
        }
        resolve();
      }
    });
    cloudSave.list(id);
  });
}

async function createSave(
  name: string,
  dataPath: string,
  playtime: number
): Promise<string> {
  return new Promise((resolve, reject) => {
    if (!fs.existsSync(dataPath)) {
      reject(new Error(`Save file not found: ${dataPath}`));
      return;
    }
    
    const id = request('create', (event) => {
      if (event.error) {
        console.error('Create failed:', event.error.message);
        reject(new Error(event.error.message));
      } else if (event.save) {
        console.log(`Created save: ${event.save.uuid}`);
        resolve(event.save.uuid);
      }
    });
    
    cloudSave.create(id, {
      name,
      summary: `Game save - ${new Date().toISOString()}`,
      playtime,
      dataFilePath: dataPath,
    });
  });
}

async function downloadSave(uuid: string, fileId: string): Promise<Buffer> {
  return new Promise((resolve, reject) => {
    const id = request('getData', (event) => {
      if (event.error) {
        reject(new Error(event.error.message));
      } else if (event.data) {
        console.log(`Downloaded ${event.data.length} bytes`);
        resolve(event.data);
      }
    });
    
    cloudSave.getData(id, uuid, fileId);
  });
}

async function deleteSave(uuid: string): Promise<void> {
  return new Promise((resolve, reject) => {
    const id = request('delete', (event) => {
      if (event.error) {
        reject(new Error(event.error.message));
      } else {
        console.log(`Deleted save: ${event.uuid}`);
        resolve();
      }
    });
    
    cloudSave.delete(id, uuid);
  });
}

// Event handler
function handleEvents() {
  const events = sdk.runCallbacks();
  
  for (const event of events) {
    // Check if this is a cloud save event with a request ID
    if ('requestId' in event) {
      const pending = pendingRequests.get(event.requestId);
      if (pending) {
        pendingRequests.delete(event.requestId);
        pending.callback(event);
      }
    }
  }
}

// Main loop
setInterval(handleEvents, 50);

// Demo usage
async function demo() {
  console.log('=== Cloud Save Demo ===\n');
  
  // List existing saves
  await listSaves();
  
  // Create a test save file
  const testSavePath = './test_save.dat';
  fs.writeFileSync(testSavePath, JSON.stringify({
    level: 5,
    gold: 1000,
    inventory: ['sword', 'shield', 'potion'],
    timestamp: Date.now(),
  }));
  
  // Create cloud save
  console.log('\nCreating new save...');
  const uuid = await createSave('QuickSave', testSavePath, 3600);
  
  // List again to see new save
  await listSaves();
  
  // Cleanup test file
  fs.unlinkSync(testSavePath);
  
  console.log('\nDemo complete!');
}

demo().catch(console.error);
```

## Electron Integration

Example for Electron applications.

```typescript
// main.ts (Electron main process)
import { app, BrowserWindow, ipcMain } from 'electron';
import { TapSdk, CloudSave, EventId, SystemState } from 'tapsdk-pc';

const CLIENT_ID = 'your_client_id';
const PUBLIC_KEY = 'your_public_key';

let sdk: TapSdk | null = null;
let mainWindow: BrowserWindow | null = null;

// Check restart before app is ready
if (TapSdk.restartAppIfNecessary(CLIENT_ID)) {
  app.quit();
}

app.whenReady().then(() => {
  // Initialize SDK
  sdk = new TapSdk(PUBLIC_KEY);
  
  // Verify ownership
  if (!sdk.isGameOwned()) {
    console.error('Game not owned');
    app.quit();
    return;
  }
  
  // Create window
  mainWindow = new BrowserWindow({
    width: 1280,
    height: 720,
    webPreferences: {
      nodeIntegration: false,
      contextIsolation: true,
      preload: './preload.js',
    },
  });
  
  mainWindow.loadFile('index.html');
  
  // Start event polling
  startEventLoop();
});

function startEventLoop() {
  setInterval(() => {
    if (!sdk) return;
    
    const events = sdk.runCallbacks();
    
    for (const event of events) {
      // Send events to renderer
      mainWindow?.webContents.send('tap-event', event);
      
      // Handle system shutdown
      if (event.eventId === EventId.SYSTEM_STATE_CHANGED) {
        if (event.state === SystemState.PLATFORM_SHUTDOWN) {
          sdk?.shutdown();
          app.quit();
        }
      }
    }
  }, 50);
}

// IPC handlers
ipcMain.handle('tap:authorize', () => {
  sdk?.authorize('public_profile');
});

ipcMain.handle('tap:getOpenId', () => {
  return sdk?.getOpenId() ?? null;
});

ipcMain.handle('tap:isGameOwned', () => {
  return sdk?.isGameOwned() ?? false;
});

ipcMain.handle('tap:isDlcOwned', (_, dlcId: string) => {
  return sdk?.isDlcOwned(dlcId) ?? false;
});

ipcMain.handle('tap:showDlcStore', (_, dlcId: string) => {
  return sdk?.showDlcStore(dlcId) ?? false;
});

// Cloud save handlers
ipcMain.handle('cloudsave:list', (_, requestId: number) => {
  CloudSave.get().list(requestId);
});

ipcMain.handle('cloudsave:create', (_, requestId: number, request: any) => {
  CloudSave.get().create(requestId, request);
});

ipcMain.handle('cloudsave:delete', (_, requestId: number, uuid: string) => {
  CloudSave.get().delete(requestId, uuid);
});

app.on('window-all-closed', () => {
  sdk?.shutdown();
  app.quit();
});
```

```typescript
// preload.ts
import { contextBridge, ipcRenderer } from 'electron';

contextBridge.exposeInMainWorld('tapSdk', {
  authorize: () => ipcRenderer.invoke('tap:authorize'),
  getOpenId: () => ipcRenderer.invoke('tap:getOpenId'),
  isGameOwned: () => ipcRenderer.invoke('tap:isGameOwned'),
  isDlcOwned: (dlcId: string) => ipcRenderer.invoke('tap:isDlcOwned', dlcId),
  showDlcStore: (dlcId: string) => ipcRenderer.invoke('tap:showDlcStore', dlcId),
  onEvent: (callback: (event: any) => void) => {
    ipcRenderer.on('tap-event', (_, event) => callback(event));
  },
  cloudSave: {
    list: (requestId: number) => ipcRenderer.invoke('cloudsave:list', requestId),
    create: (requestId: number, request: any) => 
      ipcRenderer.invoke('cloudsave:create', requestId, request),
    delete: (requestId: number, uuid: string) => 
      ipcRenderer.invoke('cloudsave:delete', requestId, uuid),
  },
});
```

```typescript
// renderer.ts
declare global {
  interface Window {
    tapSdk: {
      authorize: () => Promise<void>;
      getOpenId: () => Promise<string | null>;
      isGameOwned: () => Promise<boolean>;
      isDlcOwned: (dlcId: string) => Promise<boolean>;
      showDlcStore: (dlcId: string) => Promise<boolean>;
      onEvent: (callback: (event: any) => void) => void;
      cloudSave: {
        list: (requestId: number) => Promise<void>;
        create: (requestId: number, request: any) => Promise<void>;
        delete: (requestId: number, uuid: string) => Promise<void>;
      };
    };
  }
}

// Listen for SDK events
window.tapSdk.onEvent((event) => {
  console.log('Received event:', event);
  
  if (event.eventId === 2002) { // AUTHORIZE_FINISHED
    if (event.token) {
      console.log('User authorized!');
      updateUI();
    }
  }
});

async function updateUI() {
  const openId = await window.tapSdk.getOpenId();
  document.getElementById('user-id')!.textContent = openId ?? 'Not logged in';
}

// Login button
document.getElementById('login-btn')?.addEventListener('click', () => {
  window.tapSdk.authorize();
});
```
