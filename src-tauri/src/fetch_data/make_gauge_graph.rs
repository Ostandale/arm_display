use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------
//  todo    ゲージタイプ
//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------

//  JSON用構造体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct JsonData {
    group: String,
    value: i32,
}

pub async fn make_data_gauge(fetch_label: Vec<String>, fetch_data: Vec<String>) -> String {
    let mut graph_data: Vec<JsonData> = Vec::new();

    for (label, data) in fetch_label.iter().zip(fetch_data.iter()) {
        let check_number = data.parse::<i32>();
        match check_number {
            Ok(number) => {
                let data = JsonData {
                    group: label.to_string(),
                    value: number,
                };
                graph_data.push(data);
            }
            Err(e) => {
                eprint!("パース失敗 : {}", e);
            }
        }
    }
    serde_json::to_string(&graph_data).unwrap()
}

//  todo    //-------------------------------------------------------------------
//  *   ゲージのオプション作成
//  todo    //-------------------------------------------------------------------

pub async fn make_option_gauge(config: &FetchSpreadSheetConfig, update_epoch_time: u64) -> String {
    let japan_time = Utc
        .timestamp_opt(update_epoch_time as i64, 0)
        .unwrap()
        .with_timezone(&chrono::offset::FixedOffset::east_opt(9 * 3600).unwrap());
    let format_time = japan_time
        .format(" 最終更新時間 %Y-%m-%d %H:%M:%S")
        .to_string();
    let option_data_head = r#"{
        "title": ""#
        .to_string();

    let option_data_foot = r#"",
        "resizable": true,
        "height": "600px",
        "width": "100%",
        "gauge": {
            "status": "warning",
            "type": "full"
        },
        "bars": {
            "spaceingFactor": 0.15,
            "maxWidth": 50
        },
        "theme": "g90"
    }
    "#;

    let option_str = option_data_head + &config.sheet_name + &format_time + option_data_foot;
    serde_json::to_string(&option_str).unwrap()
}
