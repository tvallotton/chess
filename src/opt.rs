use std::str::FromStr;

use crate::parameters::Params;

use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opt {
    pub settings: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    // #[structopt(short = "r", long, default_value = "256")]
    pub recursion_limit: u64,

    // #[structopt(short = "t", long, default_value = "500")]
    pub time_limit: u128,

    // #[structopt(short = "b", long)]
    // pub play_as_black: bool,
    // #[structopt(long)]
    pub memory_limit: usize,

    pub white_params: Params,

    pub black_params: Params,

    pub absolute_params: Params,

    pub nocapture: bool,
}

impl FromStr for Settings {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
