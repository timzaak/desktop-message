const { app,ipcMain, BrowserWindow } = require('electron')
const path = require('path')
const { startServer, getConfig } = require('deskmsg_napi')

const createWindow = () => {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            preload: path.join(__dirname, 'preload.js')
        }
    })
    win.webContents.openDevTools()
    win.loadFile('index.html')
}

app.whenReady().then(() => {

    ipcMain.handle('startNativeServer', () => {
        startServer({
            mqttAddress: '127.0.0.1:0',
            httpAddress: '127.0.0.1:0',
            basicPath: '',
            httpAuthToken: '123456'
        })
    })
    ipcMain.handle('getNativeServerConfig',() => {
        return getConfig()
    })
    createWindow()
    app.on('activate', () => {
        if (BrowserWindow.getAllWindows().length === 0) createWindow()
    })
})
app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        app.quit()
    }
})