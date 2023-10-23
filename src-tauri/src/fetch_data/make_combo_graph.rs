use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};

use serde::ser::{SerializeMap, SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::json;

//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------
//  todo    コンボグラフ
//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------

//  JSON用構造体
#[derive(Serialize, Deserialize, Clone)]
struct MyData1 {
    group: String,
    date: String,
    value1: i32,
}
#[derive(Serialize, Deserialize, Clone)]
struct MyData2 {
    group: String,
    date: String,
    value2: i32,
}

pub async fn make_data_combo(fetch_label: Vec<String>, fetch_data: Vec<String>) -> String {
    let mut graph_data: Vec<serde_json::Value> = Vec::new();

    let data_length = fetch_label.len();
    for (index, label) in fetch_label.iter().enumerate() {
        let data1 = &fetch_data[index];
        let data2 = &fetch_data[data_length + index];

        let json_obj = if index % 2 == 0 {
            let data = MyData1 {
                group: label.to_string(),
                date: data1.to_string(),
                value1: data2.parse::<i32>().unwrap(),
            };
            serde_json::to_value(&data).unwrap()
        } else {
            let data = MyData2 {
                group: label.to_string(),
                date: data1.to_string(),
                value2: data2.parse::<i32>().unwrap(),
            };
            serde_json::to_value(&data).unwrap()
        };
        graph_data.push(json_obj);
    }
    serde_json::to_string(&graph_data).unwrap()
}

//  todo    //-------------------------------------------------------------------
//  *   コンボのオプション作成
//  todo    //-------------------------------------------------------------------

pub async fn make_option_combo(config: &FetchSpreadSheetConfig, update_epoch_time: u64) -> String {
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
          "mapsTo": "value1",
          "scaleType": "linear",
          "title": "左縦軸タイトル"
        },
        "right": {
          "mapsTo": "value2",
          "scaleType": "linear",
          "title": "右縦軸タイトル",
          "correspondingDatasets": [
            "グラフB"
          ]
        },
        "bottom": {
          "mapsTo": "date",
          "scaleType": "labels",
          "title": ""#;

    let option_data_foot = r#""
        }
    },
    "comboChartTypes": [
    {
      "type": "simple-bar",
      "correspondingDatasets": [
        "グラフA"
      ]
    },
    {
      "type": "line",
      "options": {
        "points": {
          "radius": 5
        }
      },
      "correspondingDatasets": [
        "グラフB"
      ]
    }
  ],
    "bars": {
        "spacingFactor": 0.15,
        "maxWidth": 300
    },
    "height": "700px",
    "theme": "g90"
}
"#;

    option_data_head + &config.sheet_name + option_data_mid + &format_time + option_data_foot
}
