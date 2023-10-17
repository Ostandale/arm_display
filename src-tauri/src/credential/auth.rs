// auth.rs
extern crate google_sheets4 as sheets4;
use sheets4::oauth2::{self, authenticator::Authenticator};
use sheets4::{hyper, hyper_rustls};

use crate::credential::http_client::http_client;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_rustls::HttpsConnector;

pub async fn auth1(
    client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    credentials: &str,
) -> Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
    let secret: oauth2::ServiceAccountKey = oauth2::parse_service_account_key(credentials).unwrap();
    // let secret1: oauth2::ServiceAccountKey = oauth2::read_service_account_key(&config.priv_key)
    //     .await
    //     .expect("secret not found");

    return oauth2::ServiceAccountAuthenticator::with_client(secret, client.clone())
        .build()
        .await
        .expect("could not create an authenticator");
}

pub async fn auth() -> (
    Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    Client<HttpsConnector<HttpConnector>>,
) {
    let credentials_data = yup_oauth2::read_service_account_key("../google_service_acc.json")
        .await
        .expect("サービスアカウントキーの読み込み失敗");
    let client: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> =
        http_client();
    let auth: Authenticator<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> =
        yup_oauth2::ServiceAccountAuthenticator::with_client(credentials_data, client.clone())
            .build()
            .await
            .expect("認証失敗");
    (auth, client)
}
