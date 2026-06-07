/**
 * TapTap PC SDK - Achievement functionality
 */

import { native } from './native.js';

/**
 * Achievement API
 *
 * @example
 * ```typescript
 * import { Achievement, EventId } from 'tapsdk-pc';
 *
 * const achievement = Achievement.get();
 *
 * achievement.unlock(1, 'first_win');
 * achievement.increment(2, 'kill_100_enemies', 1);
 * achievement.showAchievements();
 * ```
 */
export class Achievement {
  private readonly _native: ReturnType<typeof native.Achievement.get>;

  private constructor(nativeInstance: ReturnType<typeof native.Achievement.get>) {
    this._native = nativeInstance;
  }

  /**
   * Get the achievement singleton instance.
   *
   * @returns Achievement instance
   * @throws Error if SDK is not initialized
   */
  static get(): Achievement {
    const nativeInstance = native.Achievement.get();
    return new Achievement(nativeInstance);
  }

  /**
   * Unlock an achievement.
   *
   * The result will be delivered via the AchievementUnlock event.
   *
   * @param requestId - A unique ID to identify this request in the callback
   * @param achievementId - The achievement identifier configured in TapTap
   */
  unlock(requestId: number, achievementId: string): void {
    this._native.unlock(requestId, achievementId);
  }

  /**
   * Increment progress for a step-based achievement.
   *
   * The result will be delivered via the AchievementIncrement event.
   *
   * @param requestId - A unique ID to identify this request in the callback
   * @param achievementId - The achievement identifier configured in TapTap
   * @param steps - The number of steps to add
   */
  increment(requestId: number, achievementId: string, steps: number): void {
    this._native.increment(requestId, achievementId, steps);
  }

  /**
   * Open the TapTap achievements page.
   */
  showAchievements(): void {
    this._native.showAchievements();
  }
}
