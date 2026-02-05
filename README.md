# TapTap PC SDK - JavaScript Bindings

This project provides Rust and Node.js bindings for the TapTap PC SDK.

## Project Structure

```
tapsdk-pc-js/
├── reference/              # Original SDK files (headers, DLL, lib)
├── crates/
│   ├── tapsdk-pc-sys/     # Raw FFI bindings (bindgen)
│   └── tapsdk-pc/         # Safe Rust API
└── packages/
    └── tapsdk-pc-js/      # Node.js bindings (NAPI-RS)
```

## Architecture

```
┌─────────────────────────────────────┐
│         JavaScript App              │
│         (Node.js / Electron)        │
├─────────────────────────────────────┤
│         tapsdk-pc-js                │  ← NAPI-RS bindings
│         (Native Node module)        │
├─────────────────────────────────────┤
│         tapsdk-pc                   │  ← Safe Rust wrapper
│         (High-level API)            │
├─────────────────────────────────────┤
│         tapsdk-pc-sys               │  ← Raw FFI (bindgen)
│         (Unsafe bindings)           │
├─────────────────────────────────────┤
│         taptap_api.dll              │  ← Native C library
└─────────────────────────────────────┘
```

## Prerequisites

- **Rust** (stable toolchain)
- **Node.js** >= 16
- **LLVM/Clang** (for bindgen)
- **TapTap Client** (for running the SDK)

## Building

### 1. Build Rust crates

```bash
cargo build --workspace --release
```

### 2. Build Node.js module

```bash
cd packages/tapsdk-pc-js
npm install
npm run build
```

## Usage

### Node.js

```javascript
const { TapSdk, CloudSave, event_id, system_state } = require('tapsdk-pc');

// Check if restart is needed (call before init)
if (TapSdk.restartAppIfNecessary('your_client_id')) {
  process.exit(0); // TapTap will relaunch the game
}

// Initialize SDK
const sdk = new TapSdk('your_public_key');

// Check game ownership
if (!sdk.isGameOwned()) {
  console.log('User does not own this game');
  process.exit(1);
}

// Request authorization
sdk.authorize('public_profile');

// Poll for events in your game loop
function gameLoop() {
  const events = sdk.runCallbacks();
  
  for (const event of events) {
    switch (event.eventId) {
      case event_id.AUTHORIZE_FINISHED:
        if (event.token) {
          console.log('Authorized! OpenID:', sdk.getOpenId());
        }
        break;
        
      case event_id.SYSTEM_STATE_CHANGED:
        if (event.state === system_state.PLATFORM_SHUTDOWN) {
          // Save game and exit
          sdk.shutdown();
          process.exit(0);
        }
        break;
    }
  }
  
  requestAnimationFrame(gameLoop);
}
```

### Cloud Save

```javascript
const cloudSave = CloudSave.get();

// List saves
cloudSave.list(1); // request_id = 1

// Create a save
cloudSave.create(2, {
  name: 'save1',
  summary: 'Chapter 1 completed',
  playtime: 3600, // seconds
  dataFilePath: './savegame.dat',
  coverFilePath: './screenshot.png'
});

// Handle responses in runCallbacks()
for (const event of sdk.runCallbacks()) {
  if (event.eventId === event_id.CLOUD_SAVE_LIST) {
    console.log(`Found ${event.saves.length} saves`);
  }
}
```

### Rust

```rust
use tapsdk_pc::{TapSdk, user, ownership, dlc, callback::TapEvent};

fn main() -> tapsdk_pc::Result<()> {
    // Check if restart needed
    if tapsdk_pc::restart_app_if_necessary("client_id")? {
        return Ok(());
    }

    // Initialize
    let sdk = TapSdk::init("public_key")?;

    // Check ownership
    if !ownership::is_game_owned() {
        return Err("User doesn't own game".into());
    }

    // Authorize
    user::authorize("public_profile")?;

    // Game loop
    loop {
        for event in sdk.run_callbacks() {
            match event {
                TapEvent::AuthorizeFinished(data) => {
                    if let Some(token) = data.token {
                        println!("Authorized!");
                    }
                }
                TapEvent::SystemStateChanged(data) => {
                    // Handle platform state changes
                }
                _ => {}
            }
        }
    }
}
```

## API Reference

### TapSdk

| Method | Description |
|--------|-------------|
| `TapSdk.restartAppIfNecessary(clientId)` | Check if app needs restart |
| `new TapSdk(pubKey)` | Initialize the SDK |
| `sdk.getClientId()` | Get the client ID |
| `TapSdk.isInitialized()` | Check if SDK is initialized |
| `sdk.runCallbacks()` | Poll for events |
| `sdk.authorize(scopes)` | Request user authorization |
| `sdk.getOpenId()` | Get user's OpenID |
| `sdk.isGameOwned()` | Check if user owns game |
| `sdk.isDlcOwned(dlcId)` | Check if user owns DLC |
| `sdk.showDlcStore(dlcId)` | Open DLC store page |
| `sdk.shutdown()` | Shut down the SDK |

### CloudSave

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

| Event ID | Event Type | Description |
|----------|------------|-------------|
| 1 | SystemStateChanged | Platform online/offline/shutdown |
| 2002 | AuthorizeFinished | Auth flow completed |
| 4001 | GamePlayableStatusChanged | Game ownership changed |
| 4002 | DlcPlayableStatusChanged | DLC ownership changed |
| 6001 | CloudSaveList | Save list response |
| 6002 | CloudSaveCreate | Save created |
| 6003 | CloudSaveUpdate | Save updated |
| 6004 | CloudSaveDelete | Save deleted |
| 6005 | CloudSaveGetData | Save data downloaded |
| 6006 | CloudSaveGetCover | Cover image downloaded |

## License

MIT
