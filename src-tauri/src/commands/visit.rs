use std::sync::Mutex;

use tauri::{AppHandle, Manager, Url};

use crate::BrowserState;

#[tauri::command]
pub fn visit(app: AppHandle, url_or_search: &str) -> Result<(), String> {
  let state = app.state::<Mutex<BrowserState>>();
  let state_lock = state.lock().unwrap();
  let mut webview = app
    .get_webview(&state_lock.current_webview)
    .ok_or("Current webview not found")?;

  let url = match Url::parse(url_or_search) {
    Ok(url) => url,
    Err(_) => {
      let mut url = Url::parse("https://www.google.com/search").map_err(|e| e.to_string())?;
      url.set_query(Some(&format!("q={url_or_search}")));
      url
    }
  };
  webview.navigate(url).map_err(|e| e.to_string())?;

  Ok(())
}
