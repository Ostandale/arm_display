use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};

//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------
//  todo    縦棒グラフ
//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------

pub async fn make_data_bar(fetch_label: Vec<String>, fetch_data: Vec<String>) -> String {
    let mut graph_data: String;

    let js_code_header = String::from("export default [\n");
    let js_code_footer = "]";
    graph_data = js_code_header;

    for (label, data) in fetch_label.iter().zip(fetch_data.iter()) {
        graph_data.push_str(&format!("{{\n\tgroup: '{}',\n", label));
        graph_data.push_str(&format!("\tvalue: {}\n}},\n", data));
    }
    graph_data.push_str(js_code_footer);
    graph_data
}

//  todo    //-------------------------------------------------------------------
//  *   縦棒グラフ
//  todo    //-------------------------------------------------------------------

pub async fn make_option_bar(config: &FetchSpreadSheetConfig, update_epoch_time: u64) -> String {
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
                mapsTo: 'value'
            },
            bottom: {
                scaleType: ScaleTypes.LABELS,
                mapsTo: 'group',
                title: '"#;

    let option_data_foot = r#"'
            },
        },
        bars: {
            spacingFactor: 0.6,
            maxWidth: 100
        },
        height: "400px",
        theme: ChartTheme.G90
    }
    "#;

    option_data_head + &config.sheet_name + option_data_mid + &format_time + option_data_foot
}
