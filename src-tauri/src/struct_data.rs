pub const MAX_GRAPH_NUMBER: i32 = 6;

// config.rs
#[derive(Debug, Clone)]
pub struct FetchSpreadSheetConfig {
    pub priv_key: String,
    pub sheet_id: String,
    pub sheet_name: String,
    pub fetch_range: String,
    pub graph_pattern: &'static str,
    pub save_graph_data_name: &'static str,
    pub update_interval: i32,
}

impl FetchSpreadSheetConfig {
    pub fn new() -> Vec<FetchSpreadSheetConfig> {
        vec![
            FetchSpreadSheetConfig {
                priv_key: String::from("credentials.json"),
                sheet_id: String::from("1CLB6stHi6SdRfjwmuSIhzYUCk4kZXuiv5xXN_nr4JzE"),
                sheet_name: String::from("シート1"),
                fetch_range: String::from("30"),
                graph_pattern: "bar",
                save_graph_data_name: "bar1",
                update_interval: 30,
            },
            FetchSpreadSheetConfig {
                priv_key: String::from("credentials.json"),
                sheet_id: String::from("1CLB6stHi6SdRfjwmuSIhzYUCk4kZXuiv5xXN_nr4JzE"),
                sheet_name: String::from("シート2"),
                fetch_range: String::from("30"),
                graph_pattern: "horibar",
                save_graph_data_name: "bar2",
                update_interval: 20,
            },
            FetchSpreadSheetConfig {
                priv_key: String::from("credentials.json"),
                sheet_id: String::from("1CLB6stHi6SdRfjwmuSIhzYUCk4kZXuiv5xXN_nr4JzE"),
                sheet_name: String::from("シート3"),
                fetch_range: String::from("30"),
                graph_pattern: "floathoribar",
                save_graph_data_name: "bar3",
                update_interval: 30,
            },
            FetchSpreadSheetConfig {
                priv_key: String::from("credentials.json"),
                sheet_id: String::from("1CLB6stHi6SdRfjwmuSIhzYUCk4kZXuiv5xXN_nr4JzE"),
                sheet_name: String::from("シート4"),
                fetch_range: String::from("30"),
                graph_pattern: "bullet",
                save_graph_data_name: "bar4",
                update_interval: 10,
            },
            FetchSpreadSheetConfig {
                priv_key: String::from("credentials.json"),
                sheet_id: String::from("1CLB6stHi6SdRfjwmuSIhzYUCk4kZXuiv5xXN_nr4JzE"),
                sheet_name: String::from("シート5"),
                fetch_range: String::from("30"),
                graph_pattern: "gauge",
                save_graph_data_name: "bar5",
                update_interval: 10,
            },
            FetchSpreadSheetConfig {
                priv_key: String::from("credentials.json"),
                sheet_id: String::from("1CLB6stHi6SdRfjwmuSIhzYUCk4kZXuiv5xXN_nr4JzE"),
                sheet_name: String::from("シート6"),
                fetch_range: String::from("30"),
                graph_pattern: "combo",
                save_graph_data_name: "bar6",
                update_interval: 10,
            },
        ]
    }
}

#[derive(Debug, Clone)]
pub struct GraphData {
    graph_type: String,       //  グラフのタイプ  Bar Lineなど
    data_fetch_interval: i32, //  データ取得、書き込み間隔    0表示しない -1起動時の１回だけ 数字の場合は分数
    sheet_name: String,       //  データを読み込むシート名
    data_file_name: String,   //  書き込むデータ名
    graph_label: Vec<String>,
}

impl GraphData {
    pub fn init(
        graph_type: String,
        data_fetch_interval: i32,
        sheet_name: String,
        data_file_name: String,
        graph_label: Vec<String>,
    ) -> GraphData {
        GraphData {
            graph_type,
            data_fetch_interval,
            sheet_name,
            data_file_name,
            graph_label,
        }
    }
}
