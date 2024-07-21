// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate reqwest;

use tauri::async_runtime::block_on;

async fn get_ip(url: String) -> Result<(), Box<dyn std::error::Error>> {
  let resp = reqwest::get(url)
      .await?
      .json::<serde_json::Value>()
      .await?;
  println!("{:#?}", resp);
  Ok(())
}

fn main() {
  let _ = block_on(get_ip("http://ip-api.com/json/?lang=zh-CN".into()));
  let _ = block_on(get_ip("https://ip.useragentinfo.com/json".into()));
  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
