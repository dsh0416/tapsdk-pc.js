/**
 * TapTap PC SDK - Leaderboard functionality
 */

import { native } from './native.js';
import type {
  LeaderboardScoreItem,
  LoadMyCenteredScoresRequest,
  LoadMyScoresRequest,
  LoadScoresRequest,
  ShowLeaderboardRequest,
} from './types.js';

/**
 * Leaderboard API
 */
export class Leaderboard {
  private readonly _native: ReturnType<typeof native.Leaderboard.get>;

  private constructor(nativeInstance: ReturnType<typeof native.Leaderboard.get>) {
    this._native = nativeInstance;
  }

  /**
   * Get the leaderboard singleton instance.
   *
   * @returns Leaderboard instance
   * @throws Error if SDK is not initialized
   */
  static get(): Leaderboard {
    const nativeInstance = native.Leaderboard.get();
    return new Leaderboard(nativeInstance);
  }

  /**
   * Submit scores to up to five leaderboards.
   *
   * The result will be delivered via the LeaderboardSubmitScores event.
   */
  submitScores(requestId: number, items: LeaderboardScoreItem[]): void {
    this._native.submitScores(requestId, items);
  }

  /**
   * Load leaderboard scores.
   *
   * The result will be delivered via the LeaderboardLoadScores event.
   */
  loadScores(requestId: number, request: LoadScoresRequest): void {
    this._native.loadScores(requestId, request);
  }

  /**
   * Load the current user's score.
   *
   * The result will be delivered via the LeaderboardLoadMyScores event.
   */
  loadMyScores(requestId: number, request: LoadMyScoresRequest): void {
    this._native.loadMyScores(requestId, request);
  }

  /**
   * Load scores near the current user.
   *
   * The result will be delivered via the LeaderboardLoadMyCenteredScores event.
   */
  loadMyCenteredScores(requestId: number, request: LoadMyCenteredScoresRequest): void {
    this._native.loadMyCenteredScores(requestId, request);
  }

  /**
   * Open the TapTap leaderboards page.
   */
  showLeaderboards(request: ShowLeaderboardRequest): void {
    this._native.showLeaderboards(request);
  }
}
