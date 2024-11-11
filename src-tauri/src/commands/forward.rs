use std::sync::Mutex;

use tauri::{AppHandle, Manager};

use crate::BrowserState;

#[tauri::command]
pub fn forward(app: AppHandle) -> Result<(), String> {
  let state = app.state::<Mutex<BrowserState>>();
  let state_lock = state.lock().unwrap();
  let webview = app
    .get_webview(&state_lock.current_webview)
    .ok_or("Current webview doesn't exist")?;
  webview
    .eval("history.forward()")
    .map_err(|e| e.to_string())?;
  Ok(())
}
