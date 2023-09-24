pub mod cmd;
pub mod config;
pub mod utils;
pub mod vault;

use indicatif::MultiProgress;
pub struct CliConf {
    pub term_ctx: MultiProgress,
    pub api_endpoint: String,
    pub http: reqwest::blocking::Client,
}
