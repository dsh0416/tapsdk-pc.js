/**
 * TapTap PC SDK - Online game functionality
 */

import { native } from './native.js';
import type {
  OnlineGameGetRoomListRequest,
  OnlineGameJoinRoomRequest,
  OnlineGameRoomRequest,
  OnlineGameSendCustomMessageRequest,
  OnlineGameUpdateRoomPropertiesRequest,
} from './types.js';

/**
 * Online game API
 */
export class OnlineGame {
  private readonly _native: ReturnType<typeof native.OnlineGame.get>;

  private constructor(nativeInstance: ReturnType<typeof native.OnlineGame.get>) {
    this._native = nativeInstance;
  }

  /**
   * Get the online game singleton instance.
   *
   * @returns OnlineGame instance
   * @throws Error if SDK is not initialized
   */
  static get(): OnlineGame {
    const nativeInstance = native.OnlineGame.get();
    return new OnlineGame(nativeInstance);
  }

  connect(requestId: number): void {
    this._native.connect(requestId);
  }

  disconnect(requestId: number): void {
    this._native.disconnect(requestId);
  }

  createRoom(requestId: number, request: OnlineGameRoomRequest): void {
    this._native.createRoom(requestId, request);
  }

  matchRoom(requestId: number, request: OnlineGameRoomRequest): void {
    this._native.matchRoom(requestId, request);
  }

  getRoomList(requestId: number, request: OnlineGameGetRoomListRequest = {}): void {
    this._native.getRoomList(requestId, request);
  }

  joinRoom(requestId: number, request: OnlineGameJoinRoomRequest): void {
    this._native.joinRoom(requestId, request);
  }

  leaveRoom(requestId: number): void {
    this._native.leaveRoom(requestId);
  }

  updatePlayerCustomStatus(requestId: number, status: number): void {
    this._native.updatePlayerCustomStatus(requestId, status);
  }

  updatePlayerCustomProperties(requestId: number, properties: string): void {
    this._native.updatePlayerCustomProperties(requestId, properties);
  }

  updateRoomProperties(requestId: number, request: OnlineGameUpdateRoomPropertiesRequest): void {
    this._native.updateRoomProperties(requestId, request);
  }

  sendCustomMessage(requestId: number, request: OnlineGameSendCustomMessageRequest): void {
    this._native.sendCustomMessage(requestId, request);
  }

  kickRoomPlayer(requestId: number, playerId: string): void {
    this._native.kickRoomPlayer(requestId, playerId);
  }

  startFrameSync(requestId: number): void {
    this._native.startFrameSync(requestId);
  }

  sendFrameInput(requestId: number, data: string): void {
    this._native.sendFrameInput(requestId, data);
  }

  stopFrameSync(requestId: number): void {
    this._native.stopFrameSync(requestId);
  }
}
