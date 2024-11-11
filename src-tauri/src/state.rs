use std::collections::HashMap;

use tauri::menu::MenuId;

use crate::menu::MenuItems;

#[derive(Debug)]
pub struct BrowserState {
  pub current_tab_id: MenuId,
  pub current_webview: String,
  pub tabs: HashMap<MenuId, String>,
  pub tabs_menu_item: MenuId,
  pub menu_items: HashMap<String, MenuItems>,
}

impl Default for BrowserState {
  fn default() -> Self {
    Self {
      current_tab_id: Default::default(),
      current_webview: "main".to_string(),
      tabs: Default::default(),
      tabs_menu_item: Default::default(),
      menu_items: Default::default(),
    }
  }
}
