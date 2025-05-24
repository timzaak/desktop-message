const { loadBinding } = require('@napi-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'deskmsg_napi' is the name of the NAPI module (binary file name without .node extension)
 * 'deskmsg-napi' is the package name
 */
let nativeModule;
try {
  nativeModule = loadBinding(__dirname, 'deskmsg_napi', 'deskmsg-napi');
} catch (e) {
  // Attempt to load for a different platform if common path fails (e.g. for yarn PnP)
  if (process.platform === 'win32') {
    nativeModule = require('.\\deskmsg_napi.node');
  } else {
    nativeModule = require('./deskmsg_napi.node');
  }
}


// Export the functions from the native module
// The names should match the `js_name` attribute or the Rust function name if js_name is not used.
module.exports = {
  startServer: nativeModule.startServer,
  getConfig: nativeModule.getConfig,
  discovery: nativeModule.discovery,
  syncTest: nativeModule.syncTest, // Assuming syncTest was kept in lib.rs
}

// Optional: You can also perform any initial checks or configurations here
// For example, checking if the module loaded correctly:
if (nativeModule && typeof nativeModule.startServer !== 'function') {
  console.warn('Native addon deskmsg-napi did not load correctly. Functions might be missing.');
} else if (!nativeModule) {
  console.error('Failed to load native addon deskmsg-napi.');
}
