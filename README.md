# TapTap PC SDK - JavaScript Bindings

Node.js bindings for the TapTap PC SDK, built with Rust and NAPI-RS.

Based on **TapTap PC SDK v4.1.1**.

[![CI](https://github.com/dsh0416/tapsdk-pc.js/actions/workflows/build.yml/badge.svg)](https://github.com/dsh0416/tapsdk-pc.js/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![npm version](https://img.shields.io/npm/v/tapsdk-pc.svg)](https://www.npmjs.com/package/tapsdk-pc)

## Features

- **Game Ownership** - Verify if users own your game or DLC
- **User Authentication** - OAuth authentication via TapTap
- **Cloud Saves** - Full cloud save support (create, update, download, delete)
- **Native Performance** - Built with Rust and NAPI-RS

## Documentation

📚 **[View Full Documentation](https://dsh0416.github.io/tapsdk-pc.js/)** - Or run `pnpm docs:dev` to start locally.

## Quick Start

### Installation

```bash
pnpm add tapsdk-pc
```

Or with npm/yarn:

```bash
npm install tapsdk-pc
yarn add tapsdk-pc
```

### Basic Usage

```typescript
import { TapSdk, EventId, SystemState } from 'tapsdk-pc';

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
      case EventId.AUTHORIZE_FINISHED:
        if (event.token) {
          console.log('Authorized! OpenID:', sdk.getOpenId());
        }
        break;
        
      case EventId.SYSTEM_STATE_CHANGED:
        if (event.state === SystemState.PLATFORM_SHUTDOWN) {
          sdk.shutdown();
          process.exit(0);
        }
        break;
    }
  }
  
  requestAnimationFrame(gameLoop);
}

gameLoop();
```

### Cloud Save

```typescript
import { CloudSave, EventId } from 'tapsdk-pc';

const cloudSave = CloudSave.get();

// List saves
cloudSave.list(1);

// Create a save
cloudSave.create(2, {
  name: 'save1',
  summary: 'Chapter 1 completed',
  playtime: 3600,
  dataFilePath: './savegame.dat',
  coverFilePath: './screenshot.png'
});

// Handle responses in runCallbacks()
for (const event of sdk.runCallbacks()) {
  if (event.eventId === EventId.CLOUD_SAVE_LIST) {
    console.log(`Found ${event.saves.length} saves`);
  }
}
```

## Project Structure

```
tapsdk-pc-js/
├── docs/                   # VitePress documentation
├── crates/
│   ├── tapsdk-pc-sys/     # Raw FFI bindings (bindgen)
│   │   └── sdk/           # Bundled SDK files (headers, DLL, lib)
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

- **Node.js** >= 20
- **pnpm** >= 9 (recommended) or npm/yarn
- **Windows** x64
- **TapTap Client** installed and running
- For building from source:
  - **Rust** (stable toolchain)
  - **LLVM/Clang** (for bindgen)

## Building from Source

### 1. Install dependencies

```bash
pnpm install
```

### 2. Build Rust crates

```bash
cargo build --workspace --release
```

### 3. Build Node.js module

```bash
cd packages/tapsdk-pc-js
pnpm run build
```

### 4. Run documentation locally

```bash
pnpm docs:dev
```

## API Reference

### TapSdk

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

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT
