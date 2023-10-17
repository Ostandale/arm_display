// sheets.rs
extern crate google_sheets4 as sheets4;
use sheets4::{
    api::{
        BatchUpdateSpreadsheetRequest, CopySheetToAnotherSpreadsheetRequest, Request, Sheet,
        SheetProperties, Spreadsheet, SpreadsheetProperties, UpdateSheetPropertiesRequest,
        UpdateSpreadsheetPropertiesRequest, UpdateValuesResponse, ValueRange,
    },
    hyper, hyper_rustls, Error, FieldMask, Sheets,
};

use std::{collections::HashMap, str::FromStr};

use crate::struct_data::FetchSpreadSheetConfig;

pub async fn make_new_sheet(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &FetchSpreadSheetConfig,
    sheet_name: String,
) -> Result<
    (
        hyper::Response<hyper::Body>,
        sheets4::api::BatchUpdateSpreadsheetResponse,
    ),
    Error,
> {
    let title = sheet_name;

    // Requestオブジェクトを作成してベクタに格納
    let request = Request {
        add_sheet: Some(sheets4::api::AddSheetRequest {
            properties: Some(sheets4::api::SheetProperties {
                title: Some(title),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };
    let requests = vec![request];

    // リクエストを送信
    let response = BatchUpdateSpreadsheetRequest {
        requests: Some(requests),
        include_spreadsheet_in_response: None,
        response_ranges: None,
        response_include_grid_data: None,
    };

    hub.spreadsheets()
        .batch_update(response, &config.sheet_id)
        .doit()
        .await
}

pub async fn copy_and_rename(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &FetchSpreadSheetConfig,
    origin_sheet_name: &String,
    new_sheet_name: String,
) -> Result<
    (
        sheets4::hyper::Response<sheets4::hyper::Body>,
        sheets4::api::BatchUpdateSpreadsheetResponse,
    ),
    Box<dyn std::error::Error>,
> {
    let mut sheet_id_tab1: i32 = 0;

    let spreadsheet = hub.spreadsheets().get(&config.sheet_id).doit().await?;
    let sheet_props: Vec<Sheet> = spreadsheet.1.sheets.unwrap();

    //  タブ１　今月　のシートIDを取得　まぁ０なんですけどね
    for sheet in &sheet_props {
        if let Some(props) = &sheet.properties {
            if let Some(title) = &props.title {
                if title == origin_sheet_name {
                    sheet_id_tab1 = props.sheet_id.unwrap();
                    break;
                }
            }
        }
    }
    println!("sheet_id= {}", sheet_id_tab1);
    //  タブをコピーするリクエストを作成
    let request = CopySheetToAnotherSpreadsheetRequest {
        destination_spreadsheet_id: Some(config.sheet_id.clone()),
        ..Default::default()
    };

    //  タブをコピー実行
    let result = hub
        .spreadsheets()
        .sheets_copy_to(request, &config.sheet_id, sheet_id_tab1)
        .doit()
        .await?;
    println!("\nコピーしたタイトル、ID = {:?}\n", result);

    let change_sheet_id = result.1.sheet_id;

    let field_mask = FieldMask::from_str("title")?;

    // let mut update_properties_request = UpdateSheetPropertiesRequest {
    //     fields: Some(field_mask),
    //     properties: Some(SpreadsheetProperties {
    //         title: Some("ABC".to_string()),
    //         ..Default::default()
    //     }),
    //     ..Default::default()
    // };

    let update_properties_request = UpdateSheetPropertiesRequest {
        fields: Some(field_mask),
        properties: Some(SheetProperties {
            sheet_id: change_sheet_id,
            title: Some("ABC".to_string()),
            ..Default::default()
        }),
    };
    println!(
        "update_properties_request = {:?}\n",
        update_properties_request
    );

    let mut request = Request {
        update_sheet_properties: Some(update_properties_request),
        ..Default::default()
    };

    let req_vec = vec![request];

    let update_request = BatchUpdateSpreadsheetRequest {
        requests: Some(req_vec),
        ..Default::default()
    };

    let res = hub
        .spreadsheets()
        .batch_update(update_request, &config.sheet_id)
        .doit()
        .await?;

    println!("res = {:?}\n", res);
    //  タブ名変更のリクエスト作成

    Ok(res)
}

pub async fn read(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &FetchSpreadSheetConfig,
    read_line: i32,
) -> Result<(hyper::Response<hyper::Body>, ValueRange), Error> {
    let fetch_range = &config.fetch_range;
    let result = hub
        .spreadsheets()
        .values_get(&config.sheet_id, &fetch_range)
        .doit()
        .await;
    result
}

pub async fn read_init_file(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    spreadsheet_id: &str,
    range: &String,
) -> Result<(hyper::Response<hyper::Body>, ValueRange), Error> {
    let result = hub
        .spreadsheets()
        .values_get(spreadsheet_id, range)
        .doit()
        .await;
    result
}

pub async fn update(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &FetchSpreadSheetConfig,
) -> Result<(hyper::Response<hyper::Body>, UpdateValuesResponse), Error> {
    let req = ValueRange {
        major_dimension: Some("ROWS".to_string()),
        range: Some("cash_db!E3:F3".to_string()),
        values: Some(vec![vec![
            serde_json::value::Value::String("123".to_string()),
            serde_json::value::Value::String("890".to_string()),
        ]]),
    };

    hub.spreadsheets()
        .values_update(req, &config.sheet_id, "A4:K30")
        .value_input_option("RAW")
        .doit()
        .await
}

pub async fn get_spreadsheet_tab_info(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &FetchSpreadSheetConfig,
) -> Result<HashMap<String, i32>, Error> {
    let mut result_vec: HashMap<String, i32> = HashMap::new();

    let spreadsheet = hub.spreadsheets().get(&config.sheet_id).doit().await?;
    let sheet_props: Vec<Sheet> = spreadsheet.1.sheets.unwrap();

    //  タブとID番号をハッシュマップに保存
    for sheet in &sheet_props {
        if let Some(props) = &sheet.properties {
            if let Some(title) = &props.title {
                if let Some(id) = &props.sheet_id {
                    result_vec.insert(
                        props.title.as_ref().unwrap().to_string(),
                        *props.sheet_id.as_ref().unwrap(),
                    );
                }
            }
        }
    }

    Ok(result_vec)
}
