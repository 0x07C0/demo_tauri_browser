pub mod commands;
pub mod menu;
pub mod state;
pub mod tabs;

use commands::visit::visit;
use menu::{build_menu, on_menu_event};
use state::BrowserState;
use std::sync::Mutex;
use tabs::on_page_load;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .manage(Mutex::new(BrowserState::default()))
    .menu(build_menu)
    .on_menu_event(on_menu_event)
    .invoke_handler(tauri::generate_handler![visit])
    .on_page_load(on_page_load)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
