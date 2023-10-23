use crate::config::DATA_FILE_PATH;
use tokio::fs;

#[tauri::command]
pub async fn fetch_spreadsheet_data(file_name: &str) -> Result<(String, String), String> {
    let data_file_path = format!("{}data/data_{}", DATA_FILE_PATH, file_name);
    let options_file_path = format!("{}data/options_{}", DATA_FILE_PATH, file_name);

    let result = tokio::fs::read_to_string(data_file_path).await;
    let data_file = match result {
        Ok(v) => serde_json::to_string(&v).unwrap(),
        Err(e) => return Err("err".to_string()),
    };
    let result = tokio::fs::read_to_string(options_file_path).await;
    let options_file = match result {
        Ok(v) => serde_json::to_string(&v).unwrap(),
        Err(e) => return Err("err".to_string()),
    };

    Ok((data_file, options_file))
}
