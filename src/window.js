export const { getCurrentWindow } = window.__TAURI__.window;

export const appWindow = getCurrentWindow();

export function minimize() {
    appWindow.minimize()
}

export function toggleMaximize() {
    appWindow.toggleMaximize()
}

export function close() {
    appWindow.close()
}
