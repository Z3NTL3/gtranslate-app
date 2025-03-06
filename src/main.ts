import { listen, emit } from '@tauri-apps/api/event';
// import { getCurrentWindow } from "@tauri-apps/api/window";

enum AppEvents {
    StartGlowEffect="start_glow_effect",
    WindowLoaded="window_loaded"
}

window.onload = () => {
    listen(AppEvents.StartGlowEffect, (_) => {
        document.getElementById("logo")?.classList.remove("glow-anim")
        setTimeout(()=> document.getElementById("logo")?.classList.add("glow-anim"), 400)
    })

    // @ts-ignore
    emit(AppEvents.WindowLoaded, {current_window})

    // document.getElementById("header")?.addEventListener("mousedown", (event) => {
    //     if (event.buttons === 1) {
    //         event.detail === 2
    //           ? getCurrentWindow().startDragging() 
    //           : null;
    //       }
    // })
};