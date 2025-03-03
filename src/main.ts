import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

// todo this is just test
window.addEventListener("DOMContentLoaded", () => {
  getCurrentWindow().show();

  invoke('plugin:translator-bindings|translate', {
    translations_options: {
        source_lang: "nl",
        target_lang: "tr",
        query: "ik ga hardlopen"
      }
  }).then(console.log).catch(console.error)
});
