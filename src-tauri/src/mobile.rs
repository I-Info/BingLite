const UA: &str =  "Mozilla/5.0 (iPhone; CPU OS 16_4) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.4 Mobile/15E148 Safari/605.1.15 BingSapphire/1.0.401121001";

#[tauri::mobile_entry_point]
fn main() {
    crate::AppBuilder::new()
        .setup(|app| {
            tauri::WindowBuilder::new(
                app,
                "main",
                tauri::WindowUrl::External(crate::URL.parse().unwrap()),
            )
            .user_agent(UA)
            .initialization_script(crate::INIT_SCRIPT)
            .build()
            .unwrap();
            Ok(())
        })
        .run(|_, _| {});
}
