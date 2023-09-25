use indicatif::MultiProgress;

pub mod cmd;
pub mod config;
pub mod utils;
pub mod vault;

pub struct CliConf {
    pub term_ctx: MultiProgress,
    pub api_endpoint: String,
    pub http: reqwest::blocking::Client,
    pub no_interaction: bool,
}
