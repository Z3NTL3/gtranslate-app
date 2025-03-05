import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from "@tauri-apps/api/window";

enum AppEvents {
    StartGlowEffect="start_glow_effect"
}

window.onload = () => {
    listen(AppEvents.StartGlowEffect, (_) => {
        document.getElementById("logo")?.classList.remove("glow-anim")
        setTimeout(()=> document.getElementById("logo")?.classList.add("glow-anim"), 400)
    })

    document.getElementById("header")?.addEventListener("mousedown", (event) => {
        if (event.buttons === 1) {
            event.detail === 2
              ? getCurrentWindow().startDragging() 
              : null;
          }
    })
};