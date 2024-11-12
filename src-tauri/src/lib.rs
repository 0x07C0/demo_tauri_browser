pub mod commands;
pub mod menu;
pub mod navbar;
pub mod state;
pub mod tabs;
pub mod update;

use commands::back::back;
use commands::forward::forward;
use commands::visit::visit;
use menu::{build_menu, on_menu_event};
use state::BrowserState;
use std::sync::Mutex;
use tabs::on_page_load;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_updater::Builder::new().build())
    .manage(Mutex::new(BrowserState::default()))
    .menu(build_menu)
    .setup(|app| {
      navbar::setup(app)?;
      let handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
        update::setup(handle).await.unwrap();
      });
      Ok(())
    })
    .on_menu_event(on_menu_event)
    .invoke_handler(tauri::generate_handler![visit, back, forward])
    .on_page_load(on_page_load)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
