use tauri::{App, AppHandle, RunEvent};

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

pub const URL: &str = "https://www.bing.com/search?q=Bing+AI&showconv=1&FORM=hpcodx";
pub const INIT_SCRIPT: &str = include_str!("../../dist/bundle.js");

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }

    pub fn run<F: FnMut(&AppHandle, RunEvent) + 'static>(self, callback: F) {
        let setup = self.setup;
        let app = tauri::Builder::default()
            .setup(move |app| {
                if let Some(setup) = setup {
                    (setup)(app)?;
                }

                Ok(())
            })
            .plugin(tauri_plugin_shell::init())
            .build(tauri::generate_context!())
            .expect("error while running tauri application");

        app.run(callback)
    }
}
