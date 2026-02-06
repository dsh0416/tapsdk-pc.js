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

const MODULE_NAME = 'tapsdk-pc';

/**
 * Get the native binding filename(s) for the current platform.
 * napi-rs uses the pattern: {name}.{platform}-{arch}-{abi}.node
 * Returns an array to support fallbacks (e.g. linux gnu then musl).
 */
function getNativeBindingNames(): string[] {
  const platform = process.platform;
  const arch = process.arch;

  switch (platform) {
    case 'win32':
      if (arch === 'x64') return [`${MODULE_NAME}.win32-x64-msvc.node`];
      if (arch === 'arm64') return [`${MODULE_NAME}.win32-arm64-msvc.node`];
      if (arch === 'ia32') return [`${MODULE_NAME}.win32-ia32-msvc.node`];
      break;
    case 'darwin':
      if (arch === 'x64') return [`${MODULE_NAME}.darwin-x64.node`];
      if (arch === 'arm64') return [`${MODULE_NAME}.darwin-arm64.node`];
      break;
    case 'linux':
      if (arch === 'x64') return [`${MODULE_NAME}.linux-x64-gnu.node`, `${MODULE_NAME}.linux-x64-musl.node`];
      if (arch === 'arm64') return [`${MODULE_NAME}.linux-arm64-gnu.node`, `${MODULE_NAME}.linux-arm64-musl.node`];
      if (arch === 'arm') return [`${MODULE_NAME}.linux-arm-gnueabihf.node`];
      break;
    case 'freebsd':
      if (arch === 'x64') return [`${MODULE_NAME}.freebsd-x64.node`];
      break;
  }

  throw new Error(
    `Unsupported platform: ${platform}-${arch}. The native addon is not built for this target.`
  );
}

// Load the native addon
// In production, the .node file will be in the same directory
// During development, it might be in the package root
function loadNativeModule() {
  const bindingNames = getNativeBindingNames();
  const searchDirs = [join(__dirname, '..'), join(__dirname, '..', '..')];

  for (const dir of searchDirs) {
    for (const name of bindingNames) {
      const path = join(dir, name);
      try {
        return require(path);
      } catch {
        continue;
      }
    }
  }

  throw new Error(
    `Failed to load native module for ${process.platform}-${process.arch}. Make sure the native addon is built with \`npm run build:native\``
  );
}

export const native = loadNativeModule();
