extern crate google_sheets4 as sheets4;
use sheets4::Sheets;
// use std::collections::HashMap;
// use eframe::egui;

mod auth;
mod config;
mod http_client;
mod sheets;

mod table;
use table::Table;
use table::Display;

#[tokio::main]
async fn main() {
    let config = config::Config::new();
    let client = http_client::http_client();
    let auth = auth::auth(&config, client.clone()).await;
    let hub = Sheets::new(client.clone(), auth);
    let mut table = Table::default();

    // let result = sheets::read(&hub, &config).await;
    let result = sheets::get_whole_sheet(&hub, &config).await;
    match result {
        Err(e) => println!("{}", e),
        Ok((_, spreadsheet)) => {
            // let totals = HashMap::<String, i32>::new();
            match &spreadsheet.value_ranges {
                Some(value_ranges) => {
                    let mut data = Vec::new();
                    for value_range in value_ranges {
                        for inner_vec in value_range.values.clone().unwrap_or_else(Vec::new) {
                            let row_data: Vec<String> = inner_vec
                                .iter()
                                .map(|v| v.to_string())
                                .collect();
                            data.push(row_data);
                        }
                    }
                    table.update_data(data);
                },
                None => {
                    println!("Error");
                }
            }
            eframe::run_native(
                "Google Sheet Display",
                eframe::NativeOptions::default(),
                Box::new(|_cc| Box::new(Display { table })),
            ).expect("Error creating GUI");
        }
    }
}