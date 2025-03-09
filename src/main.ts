import { listen, emit } from '@tauri-apps/api/event';
import { getCurrentWindow } from "@tauri-apps/api/window";
import { getVersion } from '@tauri-apps/api/app';

enum AppEvents {
    StartGlowEffect="start_glow_effect",
    WindowLoaded="window_loaded"
}

window.onload = async () => {
    listen(AppEvents.StartGlowEffect, (_) => {
        document.getElementById("logo")?.classList.remove("glow-anim")
        setTimeout(()=> document.getElementById("logo")?.classList.add("glow-anim"), 400)
    })

    
    // @ts-ignore
    emit(AppEvents.WindowLoaded, {window: getCurrentWindow().label})

    // document.getElementById("header")?.addEventListener("mousedown", (event) => {
    //     if (event.buttons === 1) {
    //         event.detail === 2
    //           ? getCurrentWindow().startDragging() 
    //           : null;
    //       }
    // })

    const appVersion = await getVersion().catch(null);
    // @ts-ignore
    document.getElementById("app-version").innerText = `v${appVersion}`
};

export default AppEvents;