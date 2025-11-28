// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};

use std::thread;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PackageJson {
    #[serde(default = "default_main")]
    main: String,
    #[serde(default)]
    window: Option<WindowConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct WindowConfig {
    #[serde(default = "default_width")]
    width: u32,
    #[serde(default = "default_height")]
    height: u32,
    #[serde(default)]
    title: Option<String>,
    #[serde(default)]
    position: Option<String>,
    #[serde(default)]
    icon: Option<String>,
}

fn default_main() -> String {
    "index.html".to_string()
}

fn default_width() -> u32 {
    832
}

fn default_height() -> u32 {
    624
}

fn get_exe_dir() -> Result<PathBuf, String> {
    std::env::current_exe()
        .map_err(|e| format!("Failed to get exe path: {}", e))?
        .parent()
        .ok_or("Failed to get exe directory")?
        .to_path_buf()
        .canonicalize()
        .map_err(|e| format!("Failed to canonicalize path: {}", e))
}

fn read_package_json() -> Result<PackageJson, String> {
    let exe_dir = get_exe_dir()?;
    let package_path = exe_dir.join("package.json");
    
    if !package_path.exists() {
        return Err(format!("package.json not found in: {}", exe_dir.display()));
    }
    
    let content = fs::read_to_string(&package_path)
        .map_err(|e| format!("Failed to read package.json: {}", e))?;
    
    serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse package.json: {}", e))
}

fn get_mime_type(path: &str) -> &'static str {
    if path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else if path.ends_with(".js") {
        "application/javascript; charset=utf-8"
    } else if path.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if path.ends_with(".json") {
        "application/json; charset=utf-8"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".gif") {
        "image/gif"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".ogg") {
        "audio/ogg"
    } else if path.ends_with(".m4a") {
        "audio/mp4"
    } else if path.ends_with(".mp3") {
        "audio/mpeg"
    } else if path.ends_with(".wav") {
        "audio/wav"
    } else if path.ends_with(".webm") {
        "video/webm"
    } else if path.ends_with(".mp4") {
        "video/mp4"
    } else {
        "application/octet-stream"
    }
}

fn start_http_server(exe_dir: PathBuf, port: u16) {
    thread::spawn(move || {
        let server = tiny_http::Server::http(format!("127.0.0.1:{}", port)).unwrap();
        println!("HTTP Server running on http://127.0.0.1:{}", port);
        
        for request in server.incoming_requests() {
            let url = request.url().to_string();
            
            // Decode URL (handle %20 for spaces, etc.)
            let decoded_url = urlencoding::decode(&url).unwrap_or(std::borrow::Cow::Borrowed(&url));
            
            let path = if decoded_url == "/" || decoded_url.is_empty() {
                "index.html".to_string()
            } else {
                decoded_url.trim_start_matches('/').to_string()
            };
            
            let file_path = exe_dir.join(&path);
            println!("Request: {} -> {}", url, file_path.display());
            
            match fs::read(&file_path) {
                Ok(content) => {
                    let mime_type = get_mime_type(&path);
                    let response = tiny_http::Response::from_data(content)
                        .with_header(
                            tiny_http::Header::from_bytes(&b"Content-Type"[..], mime_type.as_bytes()).unwrap()
                        )
                        .with_header(
                            tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..]).unwrap()
                        );
                    let _ = request.respond(response);
                }
                Err(e) => {
                    eprintln!("File not found: {} ({})", file_path.display(), e);
                    let response = tiny_http::Response::from_string("404 Not Found")
                        .with_status_code(404);
                    let _ = request.respond(response);
                }
            }
        }
    });
}

#[tauri::command]
fn get_exe_directory() -> Result<String, String> {
    get_exe_dir().map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn toggle_fullscreen(app: tauri::AppHandle) -> Result<(), String> {
    let windows = app.webview_windows();
    let window = windows.values().next().ok_or("No window")?;
    let is_full = window.is_fullscreen().map_err(|e| e.to_string())?;
    window.set_fullscreen(!is_full).map_err(|e| e.to_string())
}

#[tauri::command]
fn center_window(app: tauri::AppHandle) -> Result<(), String> {
    let windows = app.webview_windows();
    let window = windows.values().next().ok_or("No window")?;

    // Get monitor info
    let monitor = window.current_monitor().map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;

    let monitor_size = monitor.size();
    let monitor_position = monitor.position();

    // Get window size
    let window_size = window.outer_size().map_err(|e| e.to_string())?;

    // Calculate center position accounting for monitor position and taskbar
    let center_x = monitor_position.x + (monitor_size.width as i32 - window_size.width as i32) / 2;
    let center_y = monitor_position.y + (monitor_size.height as i32 - window_size.height as i32) / 2;

    // Set window position
    window.set_position(tauri::PhysicalPosition::new(center_x, center_y))
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn toggle_devtools(app: tauri::AppHandle) -> Result<(), String> {
    let windows = app.webview_windows();
    let window = windows.values().next().ok_or("No window")?;
    
    window.open_devtools();
    Ok(())
}

#[tauri::command]
fn exit_app() {
    std::process::exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Read package.json from exe directory
            let package_config = match read_package_json() {
                Ok(config) => Some(config),
                Err(e) => {
                    println!("No package.json found: {}", e);
                    println!("Showing default landing page...");
                    None
                }
            };

            let windows = app.webview_windows();
            if let Some(window) = windows.values().next() {
                if let Some(config) = package_config {
                    // package.json exists - load external app
                    println!("Loaded package.json:");
                    println!("  Main file: {}", config.main);

                    // Get exe directory for serving files
                    let exe_dir = match get_exe_dir() {
                        Ok(dir) => dir,
                        Err(e) => {
                            eprintln!("Error getting exe directory: {}", e);
                            std::process::exit(1);
                        }
                    };

                    println!("Serving files from: {}", exe_dir.display());

                    // Start embedded HTTP server on port 3000
                    let port = 3000;
                    start_http_server(exe_dir.clone(), port);
                    
                    // Give server time to start
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    
                    // Load main file via HTTP
                    let url = format!("http://127.0.0.1:{}/{}", port, config.main);
                    println!("Loading: {}", url);
                    
                    // Navigate using eval
                    let script = format!(r#"window.location.href = "{}";"#, url);
                    let _ = window.eval(&script);

                    // Apply window config if provided
                    if let Some(win_config) = config.window {
                        println!("DEBUG: Window config detected");
                        println!("DEBUG: Requested size: {}x{}", win_config.width, win_config.height);
                        
                        // Apply title if specified
                        if let Some(title) = &win_config.title {
                            println!("DEBUG: Setting title to: {}", title);
                            let _ = window.set_title(title);
                        }
                        
                        // Get current size before resize
                        if let Ok(current_size) = window.outer_size() {
                            println!("DEBUG: Current outer size BEFORE resize: {}x{}", current_size.width, current_size.height);
                        }
                        if let Ok(current_inner) = window.inner_size() {
                            println!("DEBUG: Current inner size BEFORE resize: {}x{}", current_inner.width, current_inner.height);
                        }
                        
                        // Apply size
                        let requested_size = tauri::Size::Physical(tauri::PhysicalSize {
                            width: win_config.width,
                            height: win_config.height,
                        });
                        
                        if let Err(e) = window.set_size(requested_size) {
                            println!("DEBUG: Error setting size: {}", e);
                        } else {
                            println!("DEBUG: set_size() called successfully");
                        }
                        
                        // Small delay to let resize complete
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        
                        // Check actual size after resize
                        if let Ok(actual_size) = window.outer_size() {
                            println!("DEBUG: Actual outer size AFTER resize: {}x{}", actual_size.width, actual_size.height);
                        }
                        if let Ok(actual_inner) = window.inner_size() {
                            println!("DEBUG: Actual inner size AFTER resize: {}x{}", actual_inner.width, actual_inner.height);
                        }
                        
                        // Apply position if "center"
                        if let Some(pos) = &win_config.position {
                            if pos == "center" {
                                println!("DEBUG: Centering window");
                                // Get monitor info
                                if let Ok(Some(monitor)) = window.current_monitor() {
                                    let monitor_size = monitor.size();
                                    let monitor_position = monitor.position();
                                    
                                    // Calculate center position
                                    let center_x = monitor_position.x + (monitor_size.width as i32 - win_config.width as i32) / 2;
                                    let center_y = monitor_position.y + (monitor_size.height as i32 - win_config.height as i32) / 2;
                                    
                                    println!("DEBUG: Moving to position: ({}, {})", center_x, center_y);
                                    let _ = window.set_position(tauri::PhysicalPosition::new(center_x, center_y));
                                }
                            }
                        }
                    }
                } else {
                    // No package.json - just show bundled landing page (already loaded)
                    println!("Showing bundled landing page");
                }
                
                // Show window after all configuration is complete
                println!("DEBUG: Showing window");
                let _ = window.show();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            toggle_fullscreen,
            center_window,
            toggle_devtools,
            exit_app,
            get_exe_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
