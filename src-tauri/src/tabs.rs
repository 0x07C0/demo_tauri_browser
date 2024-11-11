use std::sync::Mutex;

use nanoid::nanoid;
use tauri::{
  menu::{MenuId, MenuItem},
  webview::PageLoadPayload,
  AppHandle, LogicalPosition, LogicalSize, Manager, Webview, WebviewBuilder, WebviewUrl, Window,
};

use crate::state::BrowserState;

pub fn open_new_tab(
  app: &AppHandle,
  window: &Window,
  tabs_menu_id: &MenuId,
  current_webview: &str,
) -> Result<(MenuId, String), Box<dyn std::error::Error>> {
  let webview_id = nanoid!();
  window.add_child(
    WebviewBuilder::new(&webview_id, WebviewUrl::App(Default::default())),
    LogicalPosition::new(0., 0.),
    LogicalSize::new(600., 600.),
  )?;

  switch_tab(window, current_webview, &webview_id)?;

  let menu = window.remove_menu()?.ok_or("Menu doesn't exist")?;
  let item = MenuItem::new(app, "New Tab", true, None::<&str>)?;
  let tab_id = item.id().clone();

  menu
    .get(tabs_menu_id)
    .ok_or("Tabs menu item doesn't exist")?
    .as_submenu()
    .ok_or("Received item isn't a submenu")?
    .append(&item)?;
  window.set_menu(menu)?;

  Ok((tab_id, webview_id))
}

pub fn switch_tab(
  window: &Window,
  current_webview: &str,
  new_webview: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  window
    .get_webview(current_webview)
    .ok_or("Current webview doesn't exist")?
    .hide()?;
  window
    .get_webview(new_webview)
    .ok_or("New webview doesn't exist")?
    .show()?;

  Ok(())
}

pub fn rename_selected_tab(
  window: &Window,
  tabs_menu_id: &MenuId,
  tab_id: &MenuId,
  new_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
  let menu = window.remove_menu()?.ok_or("Menu doesn't exist")?;

  let entry = menu
    .get(tabs_menu_id)
    .ok_or("Tabs menu item doesn't exist")?
    .as_submenu()
    .ok_or("Received item isn't a submenu")?
    .get(tab_id)
    .ok_or("Tab doesn't exist")?;
  let entry = entry.as_menuitem().ok_or("Invalid menu item type")?;

  entry.set_text(new_name)?;

  window.set_menu(menu)?;
  Ok(())
}

pub fn on_page_load(webview: &Webview, payload: &PageLoadPayload) {
  let window = webview.window();
  let state = webview.state::<Mutex<BrowserState>>();
  let state_lock = state.lock().unwrap();
  let tabs_menu_id = &state_lock.tabs_menu_item;
  let new_name = payload.url().as_str();
  let tab_id = &state_lock.current_tab_id;

  if let Err(e) = rename_selected_tab(&window, tabs_menu_id, tab_id, new_name) {
    eprintln!("Failed renaming selected tab {tab_id:?} to a new name: {e}");
  }
}
