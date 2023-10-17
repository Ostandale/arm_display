use crate::struct_data::FetchSpreadSheetConfig;
use chrono::{TimeZone, Utc};

//  todo    //-------------------------------------------------------------------
//  *   横棒グラフのオプション作成
//  todo    //-------------------------------------------------------------------

pub async fn make_option_horibar(
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
        height: "600px",
        theme: ChartTheme.G90
    }
    "#;

    option_data_head + &config.sheet_name + option_data_mid + &format_time + option_data_foot
}
