/**
 * Tests for TapTap PC SDK Node.js bindings
 *
 * These tests verify:
 * 1. The native module loads correctly
 * 2. All exports are available
 * 3. Functions are callable and return expected values
 * 4. SDK fails gracefully without TapTap client
 */

import { describe, it, expect } from 'vitest';
import { native } from './native.js';
import { TapSdk } from './sdk.js';
import { CloudSave } from './cloudsave.js';
import { EventId, SystemState } from './types.js';

describe('Native Module Loading', () => {
  it('should load the native module', () => {
    expect(native).toBeDefined();
    expect(native.TapSdk).toBeDefined();
    expect(native.CloudSave).toBeDefined();
  });

  it('should have event_id namespace', () => {
    expect(native.event_id).toBeDefined();
    expect(native.event_id.SYSTEM_STATE_CHANGED).toBe(1);
    expect(native.event_id.AUTHORIZE_FINISHED).toBe(2002);
  });

  it('should have system_state namespace', () => {
    expect(native.system_state).toBeDefined();
    expect(native.system_state.PLATFORM_ONLINE).toBe(1);
    expect(native.system_state.PLATFORM_SHUTDOWN).toBe(3);
  });
});

describe('EventId Constants', () => {
  it('should have correct values', () => {
    expect(EventId.UNKNOWN).toBe(0);
    expect(EventId.SYSTEM_STATE_CHANGED).toBe(1);
    expect(EventId.AUTHORIZE_FINISHED).toBe(2002);
    expect(EventId.GAME_PLAYABLE_STATUS_CHANGED).toBe(4001);
    expect(EventId.DLC_PLAYABLE_STATUS_CHANGED).toBe(4002);
    expect(EventId.CLOUD_SAVE_LIST).toBe(6001);
    expect(EventId.CLOUD_SAVE_CREATE).toBe(6002);
    expect(EventId.CLOUD_SAVE_UPDATE).toBe(6003);
    expect(EventId.CLOUD_SAVE_DELETE).toBe(6004);
    expect(EventId.CLOUD_SAVE_GET_DATA).toBe(6005);
    expect(EventId.CLOUD_SAVE_GET_COVER).toBe(6006);
  });

  it('should match native module constants', () => {
    expect(EventId.SYSTEM_STATE_CHANGED).toBe(native.event_id.SYSTEM_STATE_CHANGED);
    expect(EventId.AUTHORIZE_FINISHED).toBe(native.event_id.AUTHORIZE_FINISHED);
    expect(EventId.CLOUD_SAVE_LIST).toBe(native.event_id.CLOUD_SAVE_LIST);
  });
});

describe('SystemState Constants', () => {
  it('should have correct values', () => {
    expect(SystemState.UNKNOWN).toBe(0);
    expect(SystemState.PLATFORM_ONLINE).toBe(1);
    expect(SystemState.PLATFORM_OFFLINE).toBe(2);
    expect(SystemState.PLATFORM_SHUTDOWN).toBe(3);
  });

  it('should match native module constants', () => {
    expect(SystemState.PLATFORM_ONLINE).toBe(native.system_state.PLATFORM_ONLINE);
    expect(SystemState.PLATFORM_SHUTDOWN).toBe(native.system_state.PLATFORM_SHUTDOWN);
  });
});

describe('TapSdk Static Methods', () => {
  it('should have isInitialized method', () => {
    expect(TapSdk.isInitialized).toBeInstanceOf(Function);
  });

  it('should have restartAppIfNecessary method', () => {
    expect(TapSdk.restartAppIfNecessary).toBeInstanceOf(Function);
  });

  it('should report SDK as not initialized', () => {
    expect(TapSdk.isInitialized()).toBe(false);
  });

  it('should return false for restartAppIfNecessary when not in TapTap', () => {
    const result = TapSdk.restartAppIfNecessary('test_client_id');
    expect(result).toBe(false);
  });
});

describe('TapSdk Initialization', () => {
  it('should fail initialization without TapTap client', () => {
    expect(() => new TapSdk('test_public_key')).toThrow();
  });

  it('should have descriptive error message', () => {
    try {
      new TapSdk('test_public_key');
      expect.fail('Should have thrown');
    } catch (error) {
      expect(error).toBeInstanceOf(Error);
      expect((error as Error).message).toBeTruthy();
      expect((error as Error).message.length).toBeGreaterThan(0);
    }
  });
});

describe('CloudSave', () => {
  it('should have get factory method', () => {
    expect(CloudSave.get).toBeInstanceOf(Function);
  });

  it('should fail to get CloudSave without SDK initialization', () => {
    expect(() => CloudSave.get()).toThrow();
  });

  it('should have descriptive error for uninitialized state', () => {
    try {
      CloudSave.get();
      expect.fail('Should have thrown');
    } catch (error) {
      expect(error).toBeInstanceOf(Error);
      expect((error as Error).message).toContain('not initialized');
    }
  });
});

describe('TapSdk Prototype Methods', () => {
  const methods = [
    'getClientId',
    'authorize',
    'getOpenId',
    'isGameOwned',
    'isDlcOwned',
    'showDlcStore',
    'shutdown',
  ] as const;

  it.each(methods)('should have %s method on prototype', (method) => {
    expect(TapSdk.prototype[method]).toBeInstanceOf(Function);
  });
});

describe('CloudSave Prototype Methods', () => {
  const methods = [
    'list',
    'create',
    'update',
    'delete',
    'getData',
    'getCover',
  ] as const;

  it.each(methods)('should have %s method on prototype', (method) => {
    expect(CloudSave.prototype[method]).toBeInstanceOf(Function);
  });
});

describe('Type Exports', () => {
  it('should export EventId as const object', () => {
    // TypeScript ensures this at compile time, but verify runtime behavior
    expect(Object.isFrozen(EventId) || Object.keys(EventId).length > 0).toBe(true);
  });

  it('should export SystemState as const object', () => {
    expect(Object.keys(SystemState).length).toBe(4);
  });
});

describe('Error Handling', () => {
  it('should throw Error instance for SDK init failure', () => {
    try {
      new TapSdk('invalid');
    } catch (error) {
      expect(error).toBeInstanceOf(Error);
    }
  });

  it('should throw Error instance for CloudSave.get failure', () => {
    try {
      CloudSave.get();
    } catch (error) {
      expect(error).toBeInstanceOf(Error);
    }
  });
});
