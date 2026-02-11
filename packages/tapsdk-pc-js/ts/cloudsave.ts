/**
 * TapTap PC SDK - Cloud Save functionality
 */

import { native } from './native.js';
import type { CreateSaveRequest, UpdateSaveRequest } from './types.js';

/**
 * Cloud save API
 *
 * @example
 * ```typescript
 * import { TapSdk, CloudSave, EventId } from 'tapsdk-pc';
 *
 * const sdk = new TapSdk('your_public_key');
 * const cloudSave = CloudSave.get();
 *
 * // Listen for cloud save events
 * sdk.on('event', (event) => {
 *   if (event.eventId === EventId.CLOUD_SAVE_LIST) {
 *     console.log(`Found ${event.saves.length} saves`);
 *   }
 * });
 *
 * // List saves
 * cloudSave.list(1); // requestId = 1
 * ```
 */
export class CloudSave {
  private readonly _native: ReturnType<typeof native.CloudSave.get>;

  private constructor(nativeInstance: ReturnType<typeof native.CloudSave.get>) {
    this._native = nativeInstance;
  }

  /**
   * Get the cloud save singleton instance
   *
   * @returns CloudSave instance
   * @throws Error if SDK is not initialized
   */
  static get(): CloudSave {
    const nativeInstance = native.CloudSave.get();
    return new CloudSave(nativeInstance);
  }

  /**
   * Request the list of cloud saves
   *
   * The result will be delivered via the CloudSaveList event.
   *
   * @param requestId - A unique ID to identify this request in the callback
   */
  list(requestId: number): void {
    this._native.list(requestId);
  }

  /**
   * Create a new cloud save
   *
   * The result will be delivered via the CloudSaveCreate event.
   *
   * @param requestId - A unique ID to identify this request in the callback
   * @param request - The create request parameters
   */
  create(requestId: number, request: CreateSaveRequest): void {
    this._native.create(requestId, request);
  }

  /**
   * Update an existing cloud save
   *
   * The result will be delivered via the CloudSaveUpdate event.
   *
   * @param requestId - A unique ID to identify this request in the callback
   * @param request - The update request parameters
   */
  update(requestId: number, request: UpdateSaveRequest): void {
    this._native.update(requestId, request);
  }

  /**
   * Delete a cloud save
   *
   * The result will be delivered via the CloudSaveDelete event.
   *
   * @param requestId - A unique ID to identify this request in the callback
   * @param uuid - The unique ID of the cloud save to delete
   */
  delete(requestId: number, uuid: string): void {
    this._native.delete(requestId, uuid);
  }

  /**
   * Get the data file for a cloud save
   *
   * The result will be delivered via the CloudSaveGetData event.
   *
   * @param requestId - A unique ID to identify this request in the callback
   * @param uuid - The unique ID of the cloud save
   * @param fileId - The file ID of the cloud save (from CloudSaveInfo)
   */
  getData(requestId: number, uuid: string, fileId: string): void {
    this._native.getData(requestId, uuid, fileId);
  }

  /**
   * Get the cover image for a cloud save
   *
   * The result will be delivered via the CloudSaveGetCover event.
   *
   * @param requestId - A unique ID to identify this request in the callback
   * @param uuid - The unique ID of the cloud save
   * @param fileId - The file ID of the cloud save (from CloudSaveInfo)
   */
  getCover(requestId: number, uuid: string, fileId: string): void {
    this._native.getCover(requestId, uuid, fileId);
  }
}
