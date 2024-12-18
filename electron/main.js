/**
 * Author: David Yue
 * Email: davidyue5819@gmail.com
 */
const { app, BrowserWindow } = require("electron");
const path = require("path");
const process = require("process");

function createWindow() {
    const win = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            nodeIntegration: true,
            enableRemoteModule: true,
            contextIsolation: false
        }
    });

    win.loadFile("index.html");
}

app.whenReady().then(() => {
    createWindow();

    app.on("activate", () => {
        if (BrowserWindow.getAllWindows().length == 0) {
            createWindow()
        }
    });
});


app.on('window-all-closed', () => {
    if (process.platform !== "darwin") {
        app.quit()
    }
})