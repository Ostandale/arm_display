use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};

//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------
//  todo    ゲージタイプ
//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------

pub async fn make_data_gauge(fetch_label: Vec<String>, fetch_data: Vec<String>) -> String {
    let mut graph_data: String;

    let js_code_header = String::from("export default [\n");
    let js_code_footer = "]";
    graph_data = js_code_header;

    for (index, _label) in fetch_label.iter().enumerate() {
        let data = &fetch_data[index];

        graph_data.push_str("{\n");
        if index == 0 {
            graph_data.push_str("\tgroup: \"value\",\n");
        } else {
            graph_data.push_str("\tgroup: \"delta\",\n");
        }
        graph_data.push_str(&format!("\tvalue: {}\n", data));

        graph_data.push_str("\n},\n");
    }
    graph_data.push_str(js_code_footer);
    graph_data
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
    let option_data_head = r#"
    import { ChartTheme } from "@carbon/charts-svelte";
    export default {
        title: '"#
        .to_string();

    let option_data_foot = r#"',
        resizable: true,
        height: "600px",
        gauge: {
            status: 'warning',
            type: 'full'
        },
        theme: ChartTheme.G90
    }
    "#;

    option_data_head + &config.sheet_name + &format_time + option_data_foot
}
