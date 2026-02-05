---
layout: home

hero:
  name: TapTap PC SDK
  text: Node.js Bindings
  tagline: Integrate TapTap features into your PC games with ease
  image:
    src: /logo.svg
    alt: TapTap PC SDK
  actions:
    - theme: brand
      text: Get Started
      link: /getting-started
    - theme: alt
      text: View on GitHub
      link: https://github.com/user/tapsdk-pc-js

features:
  - icon: 🎮
    title: Game Ownership
    details: Verify if users own your game or DLC through TapTap platform integration.
  - icon: 🔐
    title: User Authentication
    details: Authenticate users via TapTap OAuth with customizable permission scopes.
  - icon: ☁️
    title: Cloud Saves
    details: Full cloud save support - create, update, download, and manage game saves.
  - icon: ⚡
    title: Native Performance
    details: Built with Rust and NAPI-RS for native Node.js performance.
---

## Quick Start

```bash
npm install tapsdk-pc
```

```typescript
import { TapSdk, EventId, SystemState } from 'tapsdk-pc';

// Check if restart is needed (call before init)
if (TapSdk.restartAppIfNecessary('your_client_id')) {
  process.exit(0);
}

// Initialize the SDK
const sdk = new TapSdk('your_public_key');

// Check game ownership
if (!sdk.isGameOwned()) {
  console.log('User does not own this game');
  process.exit(1);
}

// Poll for events in your game loop
const events = sdk.runCallbacks();
for (const event of events) {
  // Handle events...
}
```
