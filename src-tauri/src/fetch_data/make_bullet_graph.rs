use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------
//  todo    横棒グラフ　弾丸タイプ
//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------

//  JSON用構造体
#[derive(Debug, Serialize, Deserialize, Clone)]
struct JsonData {
    group: String,
    ranges: Vec<i32>,
    marker: i32,
    value: i32,
}

pub async fn make_data_bullet(fetch_label: Vec<String>, fetch_data: Vec<String>) -> String {
    let mut graph_data: Vec<JsonData> = Vec::new();

    let data_length = fetch_label.len();
    for (i, label) in fetch_label.iter().enumerate() {
        let data_value = fetch_data[i].replace('/', "-").parse::<i32>().unwrap();
        let data_range1 = fetch_data[data_length + i].parse::<i32>().unwrap();
        let data_range2 = fetch_data[data_length * 2 + i].parse::<i32>().unwrap();
        let data_range3 = fetch_data[data_length * 3 + i].parse::<i32>().unwrap();
        let data_marker = fetch_data[data_length * 4 + i].parse::<i32>().unwrap();

        let data = JsonData {
            group: label.to_string(),
            ranges: vec![data_range1, data_range2, data_range3],
            marker: data_marker,
            value: data_value,
        };

        graph_data.push(data);
    }
    serde_json::to_string(&graph_data).unwrap()
}
//  todo    //-------------------------------------------------------------------
//  *   横棒グラフ　弾丸タイプのオプション作成
//  todo    //-------------------------------------------------------------------

pub async fn make_option_bullet(config: &FetchSpreadSheetConfig, update_epoch_time: u64) -> String {
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

    let option_data_mid = r#"",
        "axes": {
            "left": {
                "mapsTo": "group",
                "scaleType": "labels"
            },
            "bottom": {
                "mapsTo": "value",
                "extendLinearDomainBy": "marker",
                "title" :""#;

    let option_data_foot = r#""
            }
        },
        "bars": {
            "spacingFactor": 0.6,
            "maxWidth": 300
        },
        "height": "600px",
        "theme": "g90"
    }
    "#;

    let option_str =
        option_data_head + &config.sheet_name + option_data_mid + &format_time + option_data_foot;
    serde_json::to_string(&option_str.trim()).unwrap()
}
