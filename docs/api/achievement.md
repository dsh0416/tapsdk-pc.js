# Achievement

The Achievement class provides access to TapTap achievement APIs, including unlocking achievements, incrementing step-based achievements, and opening the TapTap achievements page.

## Usage

```typescript
import { TapSdk, Achievement, EventId } from 'tapsdk-pc';

const sdk = new TapSdk('your_public_key');
const achievement = Achievement.get();

sdk.on('event', (event) => {
  if (event.eventId === EventId.ACHIEVEMENT_UNLOCK) {
    if (event.error) {
      console.error('Unlock failed:', event.error.message);
      return;
    }

    console.log('Achievement:', event.achievement?.name);
    console.log('Newly unlocked:', event.achievement?.newlyUnlocked);
  }
});

achievement.unlock(1, 'first_win');
```

## Static Methods

### get

Get the Achievement singleton instance.

```typescript
static get(): Achievement
```

**Returns:** Achievement instance

**Throws:** Error if SDK is not initialized

## Instance Methods

### unlock

Unlock an achievement.

```typescript
unlock(requestId: number, achievementId: string): void
```

**Parameters:**

- `requestId` - A unique ID to identify this request in the callback
- `achievementId` - The achievement identifier configured in TapTap

**Event:** `AchievementUnlockEvent` (EventId: `ACHIEVEMENT_UNLOCK`)

### increment

Increment progress for a step-based achievement.

```typescript
increment(requestId: number, achievementId: string, steps: number): void
```

**Parameters:**

- `requestId` - A unique ID to identify this request in the callback
- `achievementId` - The achievement identifier configured in TapTap
- `steps` - Number of steps to add

**Event:** `AchievementIncrementEvent` (EventId: `ACHIEVEMENT_INCREMENT`)

### showAchievements

Open the TapTap achievements page.

```typescript
showAchievements(): void
```

This method is synchronous and does not emit a request event.

## Event Data

```typescript
interface AchievementInfo {
  id: string;
  name: string;
  currentSteps: number;
  newlyUnlocked: boolean;
}

interface AchievementUnlockEvent {
  eventId: typeof EventId.ACHIEVEMENT_UNLOCK;
  requestId: number;
  error?: SdkError;
  achievement?: AchievementInfo;
  platinumAchievement?: AchievementInfo;
}

interface AchievementIncrementEvent {
  eventId: typeof EventId.ACHIEVEMENT_INCREMENT;
  requestId: number;
  error?: SdkError;
  achievement?: AchievementInfo;
  platinumAchievement?: AchievementInfo;
}
```
