use crate::credential::sheets::read_init_file;
use crate::struct_data::FetchSpreadSheetConfig;

use crate::fetch_data::make_bar_graph::{make_data_bar, make_option_bar};
use crate::fetch_data::make_bullet_graph::{make_data_bullet, make_option_bullet};
use crate::fetch_data::make_combo_graph::{make_data_combo, make_option_combo};
use crate::fetch_data::make_float_hori_bar::{
    make_data_float_hori_bar, make_option_float_hori_bar,
};
use crate::fetch_data::make_gauge_graph::{make_data_gauge, make_option_gauge};
use crate::fetch_data::make_hori_bar_graph::make_option_horibar;

use google_sheets4::Sheets;
use std::io::Error;
use tokio::io::ErrorKind;

//  !   //-------------------------------------------------------------------
//  !   //-------------------------------------------------------------------
//  *   スプレッドシートからデータを取得してグラフデータを作成する
//  !   //-------------------------------------------------------------------
//  !   //-------------------------------------------------------------------
pub async fn parse_spreadsheet_data(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    spreadsheet_id: &str,
    config: &FetchSpreadSheetConfig,
    update_epoch_time: u64,
    data_path: &String,
) -> core::result::Result<(), Error> {
    let mut results: Vec<std::result::Result<(), std::io::Error>> = Vec::new();

    //  スプレッドシートからグラフのラベル名となるデータを文字列ベクタとして取得
    let fetch_label = fetch_graph_label(hub, spreadsheet_id, config).await;
    //  スプレッドシートからグラフのデータを数値ベクタとして取得
    let fetch_data = fetch_graph_data(hub, spreadsheet_id, config).await;
    //  ラベルと数値のベクタから実際に使用するグラフのデータ形式に変換したファイル内容＝文字列ベクタを取得
    let data_file = convert_spreadsheet_data(fetch_label, fetch_data, config).await;
    //  グラフ用オプションファイルの作成
    let option_file = make_option_file_data(config, update_epoch_time).await;
    //
    let result = write_javascript_data_file(data_file, option_file, config, data_path).await;
    results.push(result);

    if results.iter().any(|result| result.is_err()) {
        return Err(Error::new(ErrorKind::Other, "書き込みエラーが発生しました"));
    }

    Ok(())
}

//  !   //-------------------------------------------------------------------
//  *   指定したシートの１行目から文字列としてラベルを取得する
//  !   //-------------------------------------------------------------------
pub async fn fetch_graph_label(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    spreadsheet_id: &str,
    config: &FetchSpreadSheetConfig,
) -> Vec<String> {
    let sheet_range = format!("{}!A1:Z1", config.sheet_name);
    let result = read_init_file(hub, spreadsheet_id, &sheet_range).await;

    match result {
        Ok((_, value_range)) => {
            let vec_values: Vec<String> = value_range.values.map_or(Vec::new(), |values| {
                values
                    .iter()
                    .flat_map(|row| {
                        row.iter().map(|value| {
                            let cell_content = value.to_string().replace('\"', "");
                            if cell_content.is_empty() {
                                String::from("")
                            } else {
                                cell_content
                            }
                        })
                    })
                    .collect()
            });
            vec_values
        }
        Err(err) => {
            eprintln!("ラベル名取得エラー： {}", err);
            Vec::<String>::new()
        }
    }
}

//  !   //-------------------------------------------------------------------
//  *   指定したシートの２行目以降から数値としてデータを取得する
//  !   //-------------------------------------------------------------------
pub async fn fetch_graph_data(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    spreadsheet_id: &str,
    config: &FetchSpreadSheetConfig,
) -> Vec<String> {
    let sheet_range = format!("{}!A2:Z{}", config.sheet_name, config.fetch_range);
    let result = read_init_file(hub, spreadsheet_id, &sheet_range).await;

    match result {
        Ok((_, value_range)) => {
            let vec_values: Vec<String> = value_range.values.map_or(Vec::new(), |values| {
                values
                    .iter()
                    .flat_map(|row| {
                        row.iter().map(|value| {
                            let cell_content = value.to_string().replace('\"', "");
                            if cell_content.is_empty() {
                                String::from("")
                            } else {
                                cell_content
                            }
                        })
                    })
                    .collect()
            });
            vec_values
        }
        Err(err) => {
            eprintln!("データ取得エラー： {}", err);
            Vec::<String>::new()
        }
    }
}

//  !   //-------------------------------------------------------------------
//  todo
//  *   スプレッドシートのデータとラベルを受け取ってグラフ用のデータに成形する
//  !   //-------------------------------------------------------------------
async fn convert_spreadsheet_data(
    fetch_label: Vec<String>,
    fetch_data: Vec<String>,
    config: &FetchSpreadSheetConfig,
) -> String {
    match config.graph_pattern {
        "bar" => make_data_bar(fetch_label, fetch_data).await,
        "horibar" => make_data_bar(fetch_label, fetch_data).await,
        "floathoribar" => make_data_float_hori_bar(fetch_label, fetch_data).await,
        "bullet" => make_data_bullet(fetch_label, fetch_data).await,
        "gauge" => make_data_gauge(fetch_label, fetch_data).await,
        "combo" => make_data_combo(fetch_label, fetch_data).await,
        "line" => String::from(""),
        "circle" => String::from(""),

        &_ => String::from(""),
    }
}

//  !   //-------------------------------------------------------------------
//  todo
//  *   オプションファイルの作成
//  !   //-------------------------------------------------------------------
async fn make_option_file_data(config: &FetchSpreadSheetConfig, update_epoch_time: u64) -> String {
    match config.graph_pattern {
        "bar" => make_option_bar(config, update_epoch_time).await,
        "horibar" => make_option_horibar(config, update_epoch_time).await,
        "floathoribar" => make_option_float_hori_bar(config, update_epoch_time).await,
        "bullet" => make_option_bullet(config, update_epoch_time).await,
        "gauge" => make_option_gauge(config, update_epoch_time).await,
        "combo" => make_option_combo(config, update_epoch_time).await,
        "line" => String::from(""),
        "circle" => String::from(""),

        &_ => String::from("value"),
    }
}

//  !   //-------------------------------------------------------------------
//  *   グラフ用データファイルの書き込み処理
//  !   //-------------------------------------------------------------------
async fn write_javascript_data_file(
    graph_data: String,
    option_data: String,
    config: &FetchSpreadSheetConfig,
    data_path: &String,
) -> std::result::Result<(), Error> {
    //  データファイルの書き込み
    let result_data_write = write_file(
        &format!("data_{}", config.save_graph_data_name),
        &graph_data,
        data_path,
    )
    .await;
    let result_option_write = write_file(
        &format!("options_{}", config.save_graph_data_name),
        &option_data,
        data_path,
    )
    .await;

    if result_data_write.is_ok() && result_option_write.is_ok() {
        Ok(())
    } else if result_data_write.is_err() {
        result_data_write
    } else {
        result_option_write
    }
}

//  !   //-------------------------------------------------------------------
//  *   実際にデータを書き込む
//  !   //-------------------------------------------------------------------
async fn write_file(file_name: &str, data: &str, data_path: &String) -> Result<(), Error> {
    let file_path = format!("{}{}.js", data_path, file_name);
    let result = tokio::fs::write(file_path, data).await;

    if result.is_ok() {
        println!("成功");
    } else if let Err(e) = &result {
        eprintln!("ファイル書き込みエラー： {}", e);
    }
    result
}

//  todo   //-------------------------------------------------------------------
//  todo   //-------------------------------------------------------------------
//  todo   //-------------------------------------------------------------------
//  todo   //-------------------------------------------------------------------
//  todo   //-------------------------------------------------------------------
//  todo   //-------------------------------------------------------------------
