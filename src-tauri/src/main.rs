#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

const UA: &str =  "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 Edg/112.0.0.0";
const URL: &str = "https://www.bing.com/search?q=Bing+AI&showconv=1&FORM=hpcodx";

const INIT_SCRIPT: &str = include_str!("../init.js");

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            tauri::WindowBuilder::new(
                app,
                "main",
                tauri::WindowUrl::External(URL.parse().unwrap()),
            )
            .user_agent(UA)
            .title("Bing Lite")
            .inner_size(800f64, 1000f64)
            .initialization_script(INIT_SCRIPT)
            .build()
            .unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
