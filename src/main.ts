import { listen } from '@tauri-apps/api/event';

enum AppEvents {
    StartGlowEffect="start_glow_effect"
}

window.onload = () => {
    listen(AppEvents.StartGlowEffect, (_) => {
        document.getElementById("logo")?.classList.remove("glow-anim")
        setTimeout(()=> document.getElementById("logo")?.classList.add("glow-anim"), 400)
    })
};