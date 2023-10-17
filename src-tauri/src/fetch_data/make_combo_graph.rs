use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};

//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------
//  todo    コンボグラフ
//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------

pub async fn make_data_combo(fetch_label: Vec<String>, fetch_data: Vec<String>) -> String {
    let mut graph_data: String;

    let js_code_header = String::from("export default [\n");
    let js_code_footer = "]";
    graph_data = js_code_header;

    let data_length = fetch_label.len();
    println!("fetchdata: {:?}", fetch_label);
    for (index, label) in fetch_label.iter().enumerate() {
        let data1 = &fetch_data[index];
        let data2 = &fetch_data[data_length + index];

        graph_data.push_str(&format!("{{\n\tgroup: '{}',\n", label));
        graph_data.push_str(&format!("\tdate: \"{}\",\n", data1));

        if index % 2 == 0 {
            graph_data.push_str(&format!("\tvalue1: {},\n", &data2));
        } else {
            graph_data.push_str(&format!("\tvalue2: {},\n", &data2));
        }

        graph_data.push_str("\t\n},\n");
    }
    graph_data.push_str(js_code_footer);
    graph_data
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
    let option_data_head = r#"
import { ScaleTypes } from "@carbon/charts-svelte";
import { ChartTheme } from "@carbon/charts-svelte";
export default {
    title: '"#
        .to_string();

    let option_data_mid = r#"',
    axes: {
        left: {
          mapsTo: 'value1',
          scaleType: 'linear',
          title: '左縦軸タイトル'
        },
        right: {
          mapsTo: 'value2',
          scaleType: 'linear',
          title: '右縦軸タイトル',
          correspondingDatasets: [
            'グラフB'
          ]
        },
        bottom: {
          mapsTo: 'date',
          scaleType: 'labels',
          title: '"#;

    let option_data_foot = r#"'
        },
    },
    comboChartTypes: [
    {
      type: 'simple-bar',
      correspondingDatasets: [
        'グラフA'
      ]
    },
    {
      type: 'line',
      options: {
        points: {
          radius: 5
        }
      },
      correspondingDatasets: [
        'グラフB'
      ]
    }
  ],
    bars: {
        spacingFactor: 0.15,
        maxWidth: 300
    },
    height: "700px",
    theme: ChartTheme.G90
}
"#;

    option_data_head + &config.sheet_name + option_data_mid + &format_time + option_data_foot
}
