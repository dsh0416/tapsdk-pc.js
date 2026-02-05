# TapSdk

The main SDK class for initializing the TapTap PC SDK, handling user authentication, and checking game/DLC ownership.

## Import

```typescript
import { TapSdk } from 'tapsdk-pc';
```

## Static Methods

### restartAppIfNecessary()

Check if the app needs to restart. **Must be called before creating a TapSdk instance.**

```typescript
static restartAppIfNecessary(clientId: string): boolean
```

**Parameters:**
- `clientId` - The client ID from TapTap developer center

**Returns:** `true` if app needs restart, `false` otherwise

**Example:**
```typescript
if (TapSdk.restartAppIfNecessary('your_client_id')) {
  // TapTap will relaunch the game
  process.exit(0);
}
```

::: warning Important
If this method returns `true`, you must exit your application immediately. TapTap will relaunch it properly.
:::

---

### isInitialized()

Check if the SDK is initialized.

```typescript
static isInitialized(): boolean
```

**Returns:** `true` if SDK is initialized, `false` otherwise

**Example:**
```typescript
if (TapSdk.isInitialized()) {
  console.log('SDK is ready');
}
```

## Constructor

### new TapSdk()

Initialize the SDK with your public key.

```typescript
constructor(pubKey: string)
```

**Parameters:**
- `pubKey` - The public key from TapTap developer center

**Throws:** Error if SDK initialization fails

**Example:**
```typescript
const sdk = new TapSdk('your_public_key');
```

## Instance Methods

### getClientId()

Get the client ID.

```typescript
getClientId(): string | null
```

**Returns:** The client ID or `null` if not available

---

### runCallbacks()

Poll for events from the SDK. Call this regularly in your game loop.

```typescript
runCallbacks(): TapEvent[]
```

**Returns:** Array of events that occurred since the last poll

**Example:**
```typescript
function gameLoop() {
  const events = sdk.runCallbacks();
  
  for (const event of events) {
    switch (event.eventId) {
      case EventId.SYSTEM_STATE_CHANGED:
        handleSystemState(event);
        break;
      case EventId.AUTHORIZE_FINISHED:
        handleAuth(event);
        break;
      // ... handle other events
    }
  }
  
  requestAnimationFrame(gameLoop);
}
```

---

### authorize()

Request user authorization.

```typescript
authorize(scopes: string): void
```

**Parameters:**
- `scopes` - Permission scopes to request (e.g., `"public_profile"`)

**Example:**
```typescript
sdk.authorize('public_profile');

// Handle the result in runCallbacks()
const events = sdk.runCallbacks();
for (const event of events) {
  if (event.eventId === EventId.AUTHORIZE_FINISHED) {
    if (event.token) {
      console.log('Authorized! OpenID:', sdk.getOpenId());
    }
  }
}
```

---

### getOpenId()

Get the current user's OpenID.

```typescript
getOpenId(): string | null
```

**Returns:** The user's OpenID or `null` if not authorized

---

### isGameOwned()

Check if the user owns the current game.

```typescript
isGameOwned(): boolean
```

**Returns:** `true` if user owns the game, `false` otherwise

**Example:**
```typescript
if (!sdk.isGameOwned()) {
  console.log('Please purchase the game on TapTap');
  process.exit(1);
}
```

---

### isDlcOwned()

Check if the user owns a specific DLC.

```typescript
isDlcOwned(dlcId: string): boolean
```

**Parameters:**
- `dlcId` - The DLC identifier

**Returns:** `true` if user owns the DLC, `false` otherwise

**Example:**
```typescript
if (sdk.isDlcOwned('expansion_pack_1')) {
  // Enable DLC content
  enableExpansionPack();
}
```

---

### showDlcStore()

Show the store page for a specific DLC.

```typescript
showDlcStore(dlcId: string): boolean
```

**Parameters:**
- `dlcId` - The DLC identifier

**Returns:** `true` if store page opened, `false` otherwise

**Example:**
```typescript
if (!sdk.isDlcOwned('expansion_pack_1')) {
  // Prompt user to purchase
  console.log('This content requires the Expansion Pack');
  sdk.showDlcStore('expansion_pack_1');
}
```

---

### shutdown()

Shut down the SDK. The SDK instance cannot be used after this.

```typescript
shutdown(): void
```

**Example:**
```typescript
// When exiting the game
sdk.shutdown();
process.exit(0);
```

## Complete Example

```typescript
import { TapSdk, EventId, SystemState } from 'tapsdk-pc';

// 1. Check restart before anything else
if (TapSdk.restartAppIfNecessary('your_client_id')) {
  process.exit(0);
}

// 2. Initialize
const sdk = new TapSdk('your_public_key');

// 3. Verify ownership
if (!sdk.isGameOwned()) {
  console.log('Game not owned');
  process.exit(1);
}

// 4. Request authorization
sdk.authorize('public_profile');

// 5. Game loop
let running = true;
while (running) {
  const events = sdk.runCallbacks();
  
  for (const event of events) {
    switch (event.eventId) {
      case EventId.SYSTEM_STATE_CHANGED:
        if (event.state === SystemState.PLATFORM_SHUTDOWN) {
          running = false;
        }
        break;
        
      case EventId.AUTHORIZE_FINISHED:
        if (event.token) {
          console.log('OpenID:', sdk.getOpenId());
        }
        break;
    }
  }
  
  // Your game update logic here...
}

// 6. Cleanup
sdk.shutdown();
```
