import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

window.addEventListener("DOMContentLoaded", () => {
  getCurrentWindow().show();

  invoke('translator-bindings:translations|translate', {
      source_lang: "nl",
      target_lang: "tr",
      query: "ik ga hardlopen"
  }).then(console.log).catch(console.error)
});
