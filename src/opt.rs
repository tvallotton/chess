use crate::parameters::Params;

use serde::Deserialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Deserialize)]
pub struct Opt {
    #[structopt(short = "r", long, default_value = "256")]
    pub recursion_limit: u64,
    
    #[structopt(short = "t", long, default_value = "500")]
    pub time_limit: u128,
    
    #[structopt(short = "b", long)]
    pub play_as_black: bool,
    
    #[structopt(long)]
    pub memory_limit: usize, 

    #[structopt(short = "s", long, default_value = "./settings.json")]
    pub settings_path: String,

    #[structopt(short = "wp", long)]
    pub white_params: Params,

    #[structopt(short = "bp", long)]
    pub black_params: Params,

    #[structopt(long)]
    pub nocapture: bool,
}

pub fn options() -> Opt {
    Opt::from_args()
}
