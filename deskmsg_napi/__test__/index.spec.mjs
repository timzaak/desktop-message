import test from 'ava'

import {getConfig, startServer} from "../index.js";

test('start server napi', async (t) => {
    startServer({
        mqttAddress: '127.0.0.1:0',
        httpAddress: '127.0.0.1:0',
        basicPath: '',
        httpAuthToken: 'default_token_from_main'
    })
    //
    // await new Promise((resolve) => {
    //     setTimeout(resolve, 1000*5)
    // })
    const v = getConfig()
    console.log(v)
    t.pass()
})

