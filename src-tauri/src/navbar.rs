use std::sync::Mutex;

use nanoid::nanoid;
use tauri::{App, Manager, PhysicalPosition, PhysicalSize, WebviewBuilder, WebviewUrl};

use crate::{state::BrowserState, tabs::new_tab};

pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
  let state = app.state::<Mutex<BrowserState>>();
  let mut state_lock = state.lock().unwrap();

  let window = app.get_window("main").ok_or("No default window")?;

  let size = window.inner_size()?;

  let webview_id = nanoid!();
  let webview = window.get_webview("main").ok_or("No default webview")?;
  webview.close()?;

  window.add_child(
    WebviewBuilder::new("main", WebviewUrl::App("nav.html".into())).auto_resize(),
    PhysicalPosition::new(0, 0),
    PhysicalSize::new(size.width, 70),
  )?;

  new_tab(&window, size, &webview_id)?;
  let default_tab = state_lock
    .tabs
    .iter_mut()
    .find(|(_, w)| w.as_str().eq("main"))
    .ok_or("Didn't insert a default tab in menu builder")?;
  *default_tab.1 = webview_id.clone();
  state_lock.current_webview = webview_id;

  Ok(())
}
