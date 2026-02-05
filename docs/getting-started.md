# Getting Started

## Prerequisites

Before you begin, ensure you have:

- **Node.js** >= 20
- **Windows** x64 (the SDK only supports Windows currently)
- **TapTap Client** installed and running
- A registered game on the [TapTap Developer Center](https://developer.taptap.io/)

## Installation

Install the package via pnpm (recommended):

```bash
pnpm add tapsdk-pc
```

Or with npm/yarn:

```bash
npm install tapsdk-pc
yarn add tapsdk-pc
```

## Basic Setup

### 1. Check for Restart

Before initializing the SDK, you must check if the app needs to restart. This is required when the game is launched from outside the TapTap client.

```typescript
import { TapSdk } from 'tapsdk-pc';

// Call this BEFORE creating a TapSdk instance
if (TapSdk.restartAppIfNecessary('your_client_id')) {
  // TapTap will relaunch the game - exit immediately
  process.exit(0);
}
```

::: warning Important
Always call `restartAppIfNecessary()` before creating a `TapSdk` instance. If it returns `true`, exit your application immediately.
:::

### 2. Initialize the SDK

```typescript
const sdk = new TapSdk('your_public_key');
```

You can obtain your `client_id` and `public_key` from the [TapTap Developer Center](https://developer.taptap.io/).

### 3. Verify Game Ownership

```typescript
if (!sdk.isGameOwned()) {
  console.log('User does not own this game');
  process.exit(1);
}

console.log('Game ownership verified!');
```

### 4. Handle Events

The SDK uses an event-driven architecture. You need to poll for events regularly in your game loop:

```typescript
import { TapSdk, EventId, SystemState } from 'tapsdk-pc';

function gameLoop() {
  const events = sdk.runCallbacks();
  
  for (const event of events) {
    switch (event.eventId) {
      case EventId.SYSTEM_STATE_CHANGED:
        if (event.state === SystemState.PLATFORM_SHUTDOWN) {
          // TapTap is shutting down - save and exit
          sdk.shutdown();
          process.exit(0);
        }
        break;
        
      case EventId.AUTHORIZE_FINISHED:
        if (event.token) {
          console.log('User authorized!');
          console.log('OpenID:', sdk.getOpenId());
        } else if (event.error) {
          console.error('Authorization failed:', event.error);
        } else if (event.isCancel) {
          console.log('User cancelled authorization');
        }
        break;
        
      case EventId.GAME_PLAYABLE_STATUS_CHANGED:
        if (!event.isPlayable) {
          console.log('Game is no longer playable');
        }
        break;
    }
  }
  
  // Continue game loop...
  setTimeout(gameLoop, 16); // ~60fps
}

gameLoop();
```

### 5. Request User Authorization (Optional)

If you need user information, request authorization:

```typescript
sdk.authorize('public_profile');

// The result will be delivered via the AUTHORIZE_FINISHED event
```

### 6. Shutdown

When your game exits, properly shut down the SDK:

```typescript
sdk.shutdown();
```

## Architecture Overview

The SDK is built in layers:

```
┌─────────────────────────────────────┐
│         Your Game (Node.js)         │
├─────────────────────────────────────┤
│         tapsdk-pc-js                │  ← This package
│         (NAPI-RS bindings)          │
├─────────────────────────────────────┤
│         tapsdk-pc (Rust)            │  ← Safe Rust wrapper
├─────────────────────────────────────┤
│         taptap_api.dll              │  ← Native TapTap SDK
└─────────────────────────────────────┘
```

## Next Steps

- [API Reference](/api/) - Detailed API documentation
- [Cloud Saves](/api/cloudsave) - Implement cloud save functionality
- [Examples](/examples) - Complete code examples
