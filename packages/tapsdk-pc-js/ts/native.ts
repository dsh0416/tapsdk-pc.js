/**
 * Native module loader
 * 
 * This module handles loading the native NAPI-RS addon and provides
 * type-safe exports for the native bindings.
 */

import { createRequire } from 'module';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);

// Load the native addon
// In production, the .node file will be in the same directory
// During development, it might be in the package root
function loadNativeModule() {
  const possiblePaths = [
    join(__dirname, '..', 'tapsdk-pc.win32-x64-msvc.node'),
    join(__dirname, '..', '..', 'tapsdk-pc.win32-x64-msvc.node'),
  ];

  for (const path of possiblePaths) {
    try {
      return require(path);
    } catch {
      continue;
    }
  }

  throw new Error(
    'Failed to load native module. Make sure the native addon is built with `npm run build:native`'
  );
}

export const native = loadNativeModule();
