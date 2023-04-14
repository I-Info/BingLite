#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[cfg(mobile)]
pub fn main() {
    // Mobile entry point is in src-tauri/src/mobile.rs.
    // Keep this empty.
}

#[cfg(desktop)]
const UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36 Edg/110.0.1587.41";

#[cfg(desktop)]
fn main() {
    app::AppBuilder::new()
        .setup(|app| {
            tauri::WindowBuilder::new(
                app,
                "main",
                tauri::WindowUrl::External(app::URL.parse().unwrap()),
            )
            .user_agent(UA)
            .title("Bing Lite")
            .inner_size(800f64, 1000f64)
            .initialization_script(app::INIT_SCRIPT)
            .build()
            .unwrap();
            Ok(())
        })
        .run();
}
