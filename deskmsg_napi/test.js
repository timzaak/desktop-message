// deskmsg_napi/test.js
const path = require('path');

// Determine the path to the built .node file
// This might need adjustment based on your build output directory structure
// Assuming index.js is in the root of deskmsg_napi, and it correctly loads the .node file
const { startServer, getConfig, discovery, syncTest } = require('./index.js');

async function main() {
  console.log('Starting N-API module test...');

  // Test syncTest
  try {
    const syncResult = syncTest(10);
    console.log(`syncTest(10) => ${syncResult}`);
    if (syncResult !== 110) {
      console.error('syncTest failed: Expected 110');
    }
  } catch (e) {
    console.error('Error calling syncTest:', e);
  }

  // Test startServer
  const serverConfig = {
    mqtt_address: '0.0.0.0:1883',
    http_address: '0.0.0.0:8080',
    basic_path: '/mqtt'
  };
  try {
    console.log(`Attempting to start server with config: ${JSON.stringify(serverConfig)}`);
    await startServer(JSON.stringify(serverConfig)); // NAPI function expects a JSON string
    console.log('Server started successfully (or was already running).');

    // Test getConfig
    try {
      const currentConfig = await getConfig(); // NAPI function returns a JSON string
      console.log(`Current server config: ${currentConfig}`);
      const parsedConfig = JSON.parse(currentConfig);
      if (parsedConfig.mqtt_address !== serverConfig.mqtt_address) {
        console.warn('getConfig returned MQTT address different from initial config.');
      }
    } catch (e) {
      console.error('Error calling getConfig:', e);
    }

  } catch (e) {
    console.error('Error starting server:', e.message);
    if (e.message.includes('Server is already initialized')) {
        console.log('Server was already running, which is acceptable for this test.');
        // If server was already running, we can still try getConfig
        try {
            const currentConfig = await getConfig();
            console.log(`Current server config (server was already running): ${currentConfig}`);
        } catch (e_cfg) {
            console.error('Error calling getConfig (server was already running):', e_cfg);
        }
    } else {
        // If it's another error, then it's a more serious issue
        console.error('Server startup failed for a reason other than already initialized.');
        return; // Exit if server couldn't start and wasn't already running
    }
  }

  // Test discovery
  const serviceName = '_mqtt._tcp.local'; // Example service name
  const discoverySeconds = 3;
  try {
    console.log(`Attempting discovery for service '${serviceName}' for ${discoverySeconds} seconds...`);
    const discoveryResult = await discovery(serviceName, discoverySeconds); // NAPI function returns a JSON string
    console.log(`Discovery result: ${discoveryResult}`);
    const parsedResult = JSON.parse(discoveryResult);
    if (Array.isArray(parsedResult)) {
        console.log(`Found ${parsedResult.length} services.`);
    }
  } catch (e) {
    console.error('Error calling discovery:', e);
  }
  
  console.log('N-API module test finished.');
}

main().catch(console.error);
