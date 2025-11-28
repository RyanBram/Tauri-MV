# **Tauri MV**

**A next-generation, lightweight runtime for RPG Maker MV games (Part of Project Ace).**
![Tauri Window](https://raw.githubusercontent.com/RyanBram/Tauri-MV/refs/heads/main/docs/img/Tauri%20Initial%20Window.jpg)

**Tauri MV** is a specialized runner built with **Rust** and **Tauri**. It is designed as a secure, high-performance drop-in replacement for **NW.js**.

While standard Tauri applications bundle web assets into the executable, **Tauri MV** acts as a dynamic container. It reads the local package.json configuration at launch to determine the entry point (typically index.html). This architecture allows it to run **Project Ace** (and standard RPG Maker MV) games natively while keeping the distribution size incredibly small.

## **🎯 Purpose & Context**

**Project Ace** is an enhanced version of the RPG Maker MV engine. Tauri MV was created to solve the distribution bloat and security concerns associated with the default NW.js runner used by the engine.

By switching to Tauri MV, developers can distribute their games with a **\~3 MB** executable instead of the \~200 MB+ overhead required by NW.js.

## **🚀 Key Features**

* **📦 Ultra-Lightweight:** drastically reduced distribution size. The runtime is approximately **3 MB**.  
* **🔄 Drop-in Replacement:** Designed to replace Game.exe (NW.js) without requiring changes to your project's asset structure.  
* **📂 Dynamic Loading:** Unlike standard Tauri apps, this **does not** bundle your game assets inside the binary. It reads package.json at runtime and loads your local www/index.html, making patching and updates as simple as replacing file assets.  
* **⚡ Native Performance:** Utilizes the OS's native web renderer (WebView2 on Windows, WebKit on macOS/Linux) for better performance and memory management compared to Chromium.  
* **🔒 Enhanced Security:** Built on Rust, providing memory safety and a more secure execution environment than the legacy Node.js context used in older NW.js versions.

## **⚙️ Architecture & Behavior**

When you launch TauriMV.exe (or your renamed binary), it performs the following logic:

1. Scans the current working directory.  
2. Parses the standard RPG Maker package.json.  
3. Identifies the entry point (e.g., "main": "index.html").  
4. Launches a native window rendering that entry point with the configured dimensions.

### **Directory Structure**

To run your Project Ace / RPG Maker MV game, your folder structure should look like this:

MyRPGGame/  
├── TauriMV.exe         \<-- The Runner (Replaces Game.exe/nw.exe)  
├── package.json        \<-- Your Project Ace/MV config  
├── index.html          \<-- Entry point  
├── js/  
├── css/  
└── img/

## **📝 Configuration**

Your package.json remains the source of truth. Tauri MV respects the standard fields used in Project Ace/MV projects.

**Example package.json:**

{  
  "name": "My Project Ace Game",  
  "main": "index.html",  
  "js-flags": "--expose-gc",  
  "window": {  
    "title": "Epic RPG Adventure",  
    "toolbar": false,  
    "width": 1280,  
    "height": 720,  
    "icon": "icon/icon.png",  
    "resizable": true,  
    "fullscreen": false  
  }  
}

* **main**: The relative path to your entry HTML file.  
* **window**: Configuration for the initial window state.

## **🆚 Comparison: Tauri MV vs. NW.js**

| Feature | NW.js (Default) | Tauri MV |
| :---- | :---- | :---- |
| **Engine** | Chromium \+ Node.js | System Webview \+ Rust |
| **Distribution Overhead** | Heavy (\~200MB+) | **Tiny (\~3MB)** |
| **Asset Strategy** | External or Bundled (zip) | **External (Dynamic Load)** |
| **Memory Usage** | High | Low |
| **Startup Speed** | Slower | Instant |
| **Security** | V8 Sandbox | Rust Safety |

## **🛠️ Building form Source**

If you wish to modify the runtime logic (e.g., to add custom Rust-based API bindings for Project Ace), follow these steps:

### **Prerequisites**

* [Rust & Cargo](https://www.rust-lang.org/tools/install)  
* [Node.js](https://nodejs.org/) (for Tauri CLI)  
* C++ Build Tools (Visual Studio Build Tools for Windows)

### **Installation & Build**

1. Clone the repository:  
   git clone \[https://github.com/your-repo/tauri-mv.git\](https://github.com/your-repo/tauri-mv.git)  
   cd tauri-mv

2. Install dependencies:  
   npm install

3. Build the release binary:  
   npm run tauri build

The executable will be located in src-tauri/target/release/.

## **⚠️ Compatibility Note**

WebView Dependency:  
Tauri MV relies on the system's WebView.

* **Windows:** Requires [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (Pre-installed on Windows 10/11).  
* **macOS/Linux:** Uses WebKit (Pre-installed).

Unlike NW.js, which ships its own browser engine, Tauri MV uses what is already on the user's computer. This ensures the engine is always patched against security vulnerabilities but implies that rendering may vary slightly depending on the OS version.

## **📄 License**

[MIT License](https://www.google.com/search?q=LICENSE)
