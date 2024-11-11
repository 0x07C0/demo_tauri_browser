use std::collections::HashMap;

use tauri::menu::MenuId;

use crate::menu::MenuItems;

#[derive(Debug, Default)]
pub struct BrowserState {
  pub current_tab_id: MenuId,
  pub current_webview: String,
  pub tabs: HashMap<MenuId, String>,
  pub tabs_menu_item: MenuId,
  pub menu_items: HashMap<String, MenuItems>,
}
