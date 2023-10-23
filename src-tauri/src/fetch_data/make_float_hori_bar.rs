use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json;

//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------
//  todo    横棒グラフ　フロートタイプ
//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------

//  JSON用構造体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct JsonData {
    group: String,
    date: String,
    value: Vec<i32>,
}

pub async fn make_data_float_hori_bar(fetch_label: Vec<String>, fetch_data: Vec<String>) -> String {
    let mut graph_data: Vec<JsonData> = Vec::new();

    let data_length = fetch_label.len();
    for (i, label) in fetch_label.iter().enumerate() {
        let data1 = &fetch_data[i].replace('/', "-");
        let data2 = &fetch_data[data_length + i].replace('/', "-");

        let data1_num = data1[data1.len() - 2..].parse::<i32>().unwrap();
        let data2_num = data2[data1.len() - 2..].parse::<i32>().unwrap();

        let data = JsonData {
            group: label.to_string(),
            date: data1.to_string(),
            value: vec![data1_num, data2_num],
        };
        graph_data.push(data);
    }

    serde_json::to_string(&graph_data).unwrap()
}

//  todo    //-------------------------------------------------------------------
//  *   横棒グラフ　フロートタイプのオプション作成
//  todo    //-------------------------------------------------------------------

pub async fn make_option_float_hori_bar(
    config: &FetchSpreadSheetConfig,
    update_epoch_time: u64,
) -> String {
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
                "title": ""#;

    let option_data_foot = r#""
            }
        },
        "bars": {
            "spacingFactor": 0.15,
            "maxWidth": 300
        },
        "height": "700px",
        "theme": "g90"
    }
    "#;

    let option_str =
        option_data_head + option_data_mid + &config.sheet_name + &format_time + option_data_foot;
    serde_json::to_string(&option_str).unwrap()
}
