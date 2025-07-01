const { contextBridge } = require('electron/renderer')

contextBridge.exposeInMainWorld('deskmsg', {
    //TODO:
})