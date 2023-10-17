// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod credential;
mod tauri_command;
mod test;

mod fetch_data;
mod tauri_state;

mod config;
mod struct_data;

use crate::credential::auth::auth;

use struct_data::FetchSpreadSheetConfig;
use struct_data::GraphData;

use tauri_command::greet::greet;

use google_sheets4::Sheets;

use fetch_data::parse_spreadsheet_data::parse_spreadsheet_data;

use std::time::{SystemTime, UNIX_EPOCH};

use config::DATA_REFRESH_INTERVAL;
use config::SPREADSHEET_ID;
use struct_data::MAX_GRAPH_NUMBER;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tokio::main]
async fn main() {
    //  *   GCP 認証手続き
    let (auth, client) = auth().await;
    let mut sheet = Sheets::new(client.clone(), auth);

    let thread = tokio::spawn(async move {
        let spreadsheet_config = FetchSpreadSheetConfig::new();
        let mut last_update: Vec<u64> = vec![0u64; MAX_GRAPH_NUMBER as usize];
        loop {
            for (last_update_time, config) in last_update.iter_mut().zip(spreadsheet_config.iter())
            {
                //  todo    スプレッドシートやデータベースから定期的にデータを取得して各種data.jsを書き込むルーチンを作成する
                //  *   スプレッドシートからラベル名と数値を読み込みデータファイルを書き込む
                let now = SystemTime::now();
                let since_epoch = now.duration_since(UNIX_EPOCH).expect("時間変換失敗");
                let epoch_time = since_epoch.as_secs();
                if *last_update_time == 0
                    || (*last_update_time > 0
                        && config.update_interval != -1
                        && (*last_update_time + config.update_interval as u64) < epoch_time)
                {
                    *last_update_time = epoch_time;

                    let result =
                        parse_spreadsheet_data(&sheet, SPREADSHEET_ID, config, *last_update_time)
                            .await;
                    let mut graph_list: Vec<GraphData> = Vec::new();
                }
            }
            println!("last update: {:?}", last_update);
            tokio::time::sleep(std::time::Duration::from_secs(DATA_REFRESH_INTERVAL)).await;
        }
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
