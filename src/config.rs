use std::env;

pub struct Config {
    pub private_key: String,
    pub sheet_id: String,
    pub deposit_range_input: String,
    pub deposit_range_output: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv::dotenv().ok();

        let private_key = env::var("PRIVATE_KEY_PATH").expect("PRIVATE_KEY_PATH must be set.");
        let sheet_id = env::var("SHEET_ID").expect("SHEET_ID must be set.");
        let deposit_range_input = env::var("DEPOSIT_RANGE_INPUT").expect("DEPOSIT_RANGE_INPUT must be set.");
        let deposit_range_output = env::var("DEPOSIT_RANGE_OUTPUT").expect("DEPOSIT_RANGE_OUTPUT must be set.");


        Config {
            private_key,
            sheet_id,
            deposit_range_input,
            deposit_range_output,
        }
    }
}