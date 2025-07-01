const { contextBridge, ipcRenderer  } = require('electron/renderer')

contextBridge.exposeInMainWorld('deskmsg', {
    'startNativeServer': () => ipcRenderer.invoke('startNativeServer'),
    'getNativeServerConfig': () => ipcRenderer.invoke('getNativeServerConfig'),
})