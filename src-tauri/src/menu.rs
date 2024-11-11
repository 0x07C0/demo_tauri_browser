use std::sync::Mutex;
use strum::AsRefStr;
use tauri::{
  menu::{Menu, MenuEvent, MenuItem, Submenu},
  AppHandle, Manager, State, Wry,
};

use crate::{
  tabs::{open_new_tab, switch_tab},
  BrowserState,
};

#[derive(Debug, Clone, Copy, AsRefStr)]
pub enum MenuItems {
  New,
  Back,
  Forward,
  Refresh,
}

impl MenuItems {
  pub fn tauri_menu_item(
    self,
    state: &State<'_, Mutex<BrowserState>>,
    handle: &AppHandle,
    accelerator: &str,
  ) -> tauri::Result<MenuItem<Wry>> {
    let item = MenuItem::new(handle, self, true, Some(accelerator))?;
    state
      .lock()
      .unwrap()
      .menu_items
      .insert(item.id().0.clone(), self);
    Ok(item)
  }
}

pub fn on_menu_event(app: &AppHandle, event: MenuEvent) {
  let Some(window) = app.get_window("main") else {
    eprintln!("Received a menu event while window is destroyed");
    return;
  };
  let state = app.state::<Mutex<BrowserState>>();
  let mut state = state.lock().unwrap();

  if let Some(webview_id) = state.tabs.get(event.id()) {
    if let Err(e) = switch_tab(&window, &state.current_webview, webview_id) {
      eprintln!("Failed switching tabs: {e}");
    }
    state.current_webview = webview_id.clone();
    state.current_tab_id = event.id().clone();
  }

  let Some(current_webview) = window.get_webview(&state.current_webview) else {
    eprintln!("Current webview doesn't exist");
    return;
  };

  if let Some(menu_action) = state.menu_items.get(&event.id().0) {
    match menu_action {
      MenuItems::New => {
        if let Ok((tab_id, webview_id)) =
          open_new_tab(app, &window, &state.tabs_menu_item, &state.current_webview)
            .inspect_err(|e| eprintln!("Failed opening a new tab: {e}"))
        {
          state.tabs.insert(tab_id.clone(), webview_id.clone());
          state.current_tab_id = tab_id;
          state.current_webview = webview_id;
        }
      }
      MenuItems::Back => current_webview.eval("history.back()").unwrap(),
      MenuItems::Forward => current_webview.eval("history.forward()").unwrap(),
      MenuItems::Refresh => current_webview.eval("location.reload()").unwrap(),
    }
  }
}

pub fn build_menu(handle: &AppHandle) -> tauri::Result<Menu<Wry>> {
  let state = handle.state::<Mutex<BrowserState>>();
  let mut state_lock = state.lock().unwrap();
  let default_tab = MenuItem::new(handle, "New Tab", true, None::<&str>)?;
  state_lock.current_tab_id = default_tab.id().clone();
  state_lock
    .tabs
    .insert(default_tab.id().clone(), "main".to_string());
  let tabs_item = Submenu::with_items(handle, "Tabs", true, &[&default_tab])?;
  state_lock.tabs_menu_item = tabs_item.id().clone();
  drop(state_lock);
  Menu::with_items(
    handle,
    &[
      &Submenu::with_items(
        handle,
        "Page",
        true,
        &[
          &MenuItems::New.tauri_menu_item(&state, handle, "CmdOrCtrl+N")?,
          &MenuItems::Back.tauri_menu_item(&state, handle, "CmdOrCtrl+B")?,
          &MenuItems::Forward.tauri_menu_item(&state, handle, "CmdOrCtrl+F")?,
          &MenuItems::Refresh.tauri_menu_item(&state, handle, "CmdOrCtrl+R")?,
        ],
      )?,
      &tabs_item,
    ],
  )
}
