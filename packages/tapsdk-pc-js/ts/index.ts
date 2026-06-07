/**
 * TapTap PC SDK - Node.js Bindings
 *
 * This package provides TypeScript bindings for the TapTap PC SDK.
 *
 * @packageDocumentation
 */

export { TapSdk } from './sdk.js';
export { CloudSave } from './cloudsave.js';
export { Achievement } from './achievement.js';
export {
  EventId,
  SystemState,
  type AuthToken,
  type SdkError,
  type AchievementInfo,
  type CloudSaveInfo,
  type CreateSaveRequest,
  type UpdateSaveRequest,
  type SystemStateChangedEvent,
  type AuthorizeFinishedEvent,
  type GamePlayableStatusChangedEvent,
  type DlcPlayableStatusChangedEvent,
  type CloudSaveListEvent,
  type CloudSaveCreateEvent,
  type CloudSaveUpdateEvent,
  type CloudSaveDeleteEvent,
  type CloudSaveGetDataEvent,
  type CloudSaveGetCoverEvent,
  type AchievementUnlockEvent,
  type AchievementIncrementEvent,
  type UnknownEvent,
  type TapEvent,
  type TapSdkEvents,
} from './types.js';
