use sheets4::{api::ValueRange, hyper, hyper_rustls, Error, Sheets};
use sheets4::api::{BatchGetValuesResponse};

use crate::config::Config;

pub async fn read(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
) -> Result<(hyper::Response<hyper::Body>, ValueRange), Error> {
    return hub
        .spreadsheets()
        .values_get(&config.sheet_id, &config.deposit_range_input)
        .doit()
        .await;
}

// pub async fn assign_cell_range(hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>, config: Config,
// ) -> Result<(hyper::Response<hyper::Body>, ValueRange), Error> {
//
// }

pub async fn get_whole_sheet(
    hub: &Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    config: &Config,
) -> Result<(hyper::Response<hyper::Body>, BatchGetValuesResponse), Error> {
    return hub
        .spreadsheets()
        .values_batch_get(&config.sheet_id)
        .add_ranges("Sheet1")
        .doit()
        .await;
}
