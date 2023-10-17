use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};

//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------
//  todo    横棒グラフ　弾丸タイプ
//  todo    //-------------------------------------------------------------------
//  todo    //-------------------------------------------------------------------

pub async fn make_data_bullet(fetch_label: Vec<String>, fetch_data: Vec<String>) -> String {
    let mut graph_data: String;

    let js_code_header = String::from("export default [\n");
    let js_code_footer = "]";
    graph_data = js_code_header;

    let data_length = fetch_label.len();
    for (i, label) in fetch_label.iter().enumerate() {
        let data = &fetch_data[i].replace('/', "-");
        let data_range1 = &fetch_data[data_length + i];
        let data_range2 = &fetch_data[data_length * 2 + i];
        let data_range3 = &fetch_data[data_length * 3 + i];
        let data_marker = &fetch_data[data_length * 4 + i];

        graph_data.push_str(&format!("{{\n\tgroup: '{}',\n", label));
        graph_data.push_str("\tranges: [\n");
        graph_data.push_str(&format!("\t\t{},\n", data_range1));
        graph_data.push_str(&format!("\t\t{},\n", data_range2));
        graph_data.push_str(&format!("\t\t{}\n", data_range3));
        graph_data.push_str("\t],\n");

        graph_data.push_str(&format!("\t\tmarker: {},\n", data_marker));

        graph_data.push_str(&format!("\t\tvalue: {}\n", data));

        graph_data.push_str("\n},\n");
    }
    graph_data.push_str(js_code_footer);
    graph_data
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
                extendLinearDomainBy: 'marker',
                title :'"#;

    let option_data_foot = r#"'
            },
        },
        bars: {
            spacingFactor: 0.6,
            maxWidth: 300
        },
        height: "600px",
        theme: ChartTheme.G90
    }
    "#;

    option_data_head + &config.sheet_name + option_data_mid + &format_time + option_data_foot
}
