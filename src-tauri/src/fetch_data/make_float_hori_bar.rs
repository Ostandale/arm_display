use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};

//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------
//  todo    横棒グラフ　フロートタイプ
//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------

pub async fn make_data_float_hori_bar(fetch_label: Vec<String>, fetch_data: Vec<String>) -> String {
    let mut graph_data: String;

    let js_code_header = String::from("export default [\n");
    let js_code_footer = "]";
    graph_data = js_code_header;

    let data_length = fetch_label.len();
    for (i, label) in fetch_label.iter().enumerate() {
        let data = &fetch_data[i].replace('/', "-");
        let data2 = &fetch_data[data_length + i].replace('/', "-");
        graph_data.push_str(&format!("{{\n\tgroup: '{}',\n", label));
        graph_data.push_str(&format!("\tdate: \"{}\",\n", data));
        graph_data.push_str("\tvalue: [\n");

        graph_data.push_str(&format!(
            "\t\t{:?},\n",
            &data[data.len() - 2..].parse::<i32>().unwrap()
        ));

        graph_data.push_str(&format!(
            "\t\t{}\n",
            &data2[data2.len() - 2..].parse::<i32>().unwrap()
        ));

        graph_data.push_str("\t]\n},\n");
    }
    graph_data.push_str(js_code_footer);
    graph_data
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
    let option_data_head = r#"
    import { ScaleTypes } from "@carbon/charts-svelte";
    import { ChartTheme } from "@carbon/charts-svelte";
    export default {
        title: '"#
        .to_string();

    let option_data_mid = r#"',
        axes: {
            left: {
                mapsTo: 'group',
                scaleType: ScaleTypes.LABELS,
            },
            bottom: {
                mapsTo: 'value',
                title: '"#;

    let option_data_foot = r#"'
            },
        },
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
