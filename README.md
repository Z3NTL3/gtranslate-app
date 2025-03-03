<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/othneildrew/Best-README-Template">
    <img src="README-assets/google-translate.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">GTranslate App</h3>

  <p align="center">
    GTranslate is a Windows, MacOS and Linux app for quick translations, built with speed in mind.
    <br/>
    <br />
  
  </p>
</div>


> [!WARNING]  
> Currently in **DEVELOPMENT**

# GTranslate

#### What's the difference between using this and Google Translator from my browser
The difference between using Google Translator from your browser is that this app auto starts on boot, has a system tray and hotbind to quickly open up and do your translation duties in.

#### Goals
This project has no goal other than making an arbitrary app in Rust using Tauri.

#### Features
- Keybinds to quickly open
- Autostart on boot
- System Tray
- Self updater
- Supported on Windows, MacOS and Linux
- Easy setup: platform specific popular installer wizards
- Built with Rust, maximising security and performance
- Uses Tokio for maximal performance
- Easy installation: Bundled using the most popular platform specific installation wizards

## Internal crates
- ### Plugins
  - ``tauri-plugin-translator-bindings``
    > Our Tauri plugin providing bindings to ``gtranslate`` crate made by z3ntl3
    > <br> [Source](https://github.com/Z3NTL3/gtranslate-app/tree/main/tauri-plugin-translator-bindings/src)