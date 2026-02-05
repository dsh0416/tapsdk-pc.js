/**
 * TapTap PC SDK - Main SDK class
 */

import { native } from './native.js';
import type { TapEvent } from './types.js';

/**
 * TapTap PC SDK wrapper for Node.js
 *
 * @example
 * ```typescript
 * import { TapSdk, EventId, SystemState } from 'tapsdk-pc';
 *
 * // Check if restart is needed
 * if (TapSdk.restartAppIfNecessary('your_client_id')) {
 *   process.exit(0);
 * }
 *
 * // Initialize the SDK
 * const sdk = new TapSdk('your_public_key');
 *
 * // Check ownership
 * if (!sdk.isGameOwned()) {
 *   console.log('User does not own this game');
 *   process.exit(1);
 * }
 *
 * // Poll for events in your game loop
 * const events = sdk.runCallbacks();
 * for (const event of events) {
 *   if (event.eventId === EventId.SYSTEM_STATE_CHANGED) {
 *     if (event.state === SystemState.PLATFORM_SHUTDOWN) {
 *       sdk.shutdown();
 *       process.exit(0);
 *     }
 *   }
 * }
 * ```
 */
export class TapSdk {
  private readonly _native: InstanceType<typeof native.TapSdk>;

  /**
   * Check if the app needs to restart (call before init)
   *
   * If this returns true, TapTap will relaunch the game - exit immediately.
   *
   * @param clientId - The client ID from TapTap developer center
   * @returns true if app needs restart, false otherwise
   */
  static restartAppIfNecessary(clientId: string): boolean {
    return native.TapSdk.restartAppIfNecessary(clientId);
  }

  /**
   * Check if the SDK is initialized
   *
   * @returns true if SDK is initialized, false otherwise
   */
  static isInitialized(): boolean {
    return native.TapSdk.isInitialized();
  }

  /**
   * Initialize the SDK
   *
   * @param pubKey - The public key from TapTap developer center
   * @throws Error if SDK initialization fails
   */
  constructor(pubKey: string) {
    this._native = new native.TapSdk(pubKey);
  }

  /**
   * Get the client ID
   *
   * @returns The client ID or null if not available
   */
  getClientId(): string | null {
    return this._native.getClientId();
  }

  /**
   * Poll for events from the SDK
   *
   * Call this regularly (e.g., in your game loop) to receive events.
   *
   * @returns Array of events that occurred since the last poll
   */
  runCallbacks(): TapEvent[] {
    return this._native.runCallbacks();
  }

  /**
   * Request user authorization
   *
   * @param scopes - Permission scopes to request (e.g., "public_profile")
   * @throws Error if authorization request fails
   */
  authorize(scopes: string): void {
    this._native.authorize(scopes);
  }

  /**
   * Get the current user's OpenID
   *
   * @returns The user's OpenID or null if not available
   */
  getOpenId(): string | null {
    return this._native.getOpenId();
  }

  /**
   * Check if the user owns the current game
   *
   * @returns true if user owns the game, false otherwise
   */
  isGameOwned(): boolean {
    return this._native.isGameOwned();
  }

  /**
   * Check if the user owns a specific DLC
   *
   * @param dlcId - The DLC identifier
   * @returns true if user owns the DLC, false otherwise
   */
  isDlcOwned(dlcId: string): boolean {
    return this._native.isDlcOwned(dlcId);
  }

  /**
   * Show the store page for a specific DLC
   *
   * @param dlcId - The DLC identifier
   * @returns true if store page opened, false otherwise
   */
  showDlcStore(dlcId: string): boolean {
    return this._native.showDlcStore(dlcId);
  }

  /**
   * Shut down the SDK
   *
   * This releases all resources. The SDK instance cannot be used after this.
   */
  shutdown(): void {
    this._native.shutdown();
  }
}
