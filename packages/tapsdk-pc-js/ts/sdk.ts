/**
 * TapTap PC SDK - Main SDK class
 */

import { native } from './native.js';
import type { TapEvent, TapSdkEvents } from './types.js';

type TapSdkEventName = keyof TapSdkEvents;
type TapSdkEventListener<K extends TapSdkEventName> = (...args: TapSdkEvents[K]) => void;

/**
 * TapTap PC SDK wrapper for Node.js
 *
 * Events are automatically pushed from a background thread.
 * Use the `on('event', callback)` pattern to receive them.
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
 * // Listen for events
 * sdk.on('event', (event) => {
 *   if (event.eventId === EventId.SYSTEM_STATE_CHANGED) {
 *     if (event.state === SystemState.PLATFORM_SHUTDOWN) {
 *       sdk.shutdown();
 *       process.exit(0);
 *     }
 *   }
 * });
 * ```
 */
export class TapSdk {
  private readonly _native: InstanceType<typeof native.TapSdk>;
  private readonly _listeners: {
    [K in TapSdkEventName]?: Set<TapSdkEventListener<K>>;
  } = {};

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
   * Initialize the SDK and start the background event loop.
   *
   * Events will be emitted via the 'event' event.
   *
   * @param pubKey - The public key from TapTap developer center
   * @throws Error if SDK initialization fails
   */
  constructor(pubKey: string) {
    this._native = new native.TapSdk(pubKey, (event: TapEvent) => {
      this.emit('event', event);
    });
  }

  /**
   * Register an event listener.
   *
   * @param eventName - Event name
   * @param listener - Event listener callback
   * @returns This instance for chaining
   */
  on<K extends TapSdkEventName>(eventName: K, listener: TapSdkEventListener<K>): this {
    const listeners =
      (this._listeners[eventName] as Set<TapSdkEventListener<K>> | undefined) ??
      new Set<TapSdkEventListener<K>>();
    listeners.add(listener);
    this._listeners[eventName] = listeners as (typeof this._listeners)[K];
    return this;
  }

  /**
   * Remove an event listener.
   *
   * @param eventName - Event name
   * @param listener - Listener to remove
   * @returns This instance for chaining
   */
  off<K extends TapSdkEventName>(eventName: K, listener: TapSdkEventListener<K>): this {
    const listeners = this._listeners[eventName] as Set<TapSdkEventListener<K>> | undefined;
    listeners?.delete(listener);
    if (listeners?.size === 0) {
      delete this._listeners[eventName];
    }
    return this;
  }

  /**
   * Register a one-time event listener.
   *
   * @param eventName - Event name
   * @param listener - Event listener callback
   * @returns This instance for chaining
   */
  once<K extends TapSdkEventName>(eventName: K, listener: TapSdkEventListener<K>): this {
    const wrapped: TapSdkEventListener<K> = (...args) => {
      this.off(eventName, wrapped);
      listener(...args);
    };
    return this.on(eventName, wrapped);
  }

  /**
   * Remove all listeners, or all listeners for one event.
   *
   * @param eventName - Optional event name
   * @returns This instance for chaining
   */
  removeAllListeners<K extends TapSdkEventName>(eventName?: K): this {
    if (eventName) {
      delete this._listeners[eventName];
    } else {
      for (const key of Object.keys(this._listeners) as TapSdkEventName[]) {
        delete this._listeners[key];
      }
    }
    return this;
  }

  private emit<K extends TapSdkEventName>(eventName: K, ...args: TapSdkEvents[K]): void {
    const listeners = this._listeners[eventName] as Set<TapSdkEventListener<K>> | undefined;
    if (!listeners || listeners.size === 0) {
      return;
    }
    for (const listener of listeners) {
      listener(...args);
    }
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
   * Shut down the SDK and stop the background event loop.
   *
   * This releases all resources. The SDK instance cannot be used after this.
   */
  shutdown(): void {
    this._native.shutdown();
    this.removeAllListeners();
  }
}
