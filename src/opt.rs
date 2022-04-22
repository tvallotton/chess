

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(short = "r", long, default_value = "256")]
    pub recursion_limit: u64,
    #[structopt(short = "t", long, default_value = "500")]
    pub time_limit: u64,
    #[structopt(short = "b", long)]
    pub play_as_black: bool,
    #[structopt(short = "s", long, default_value = "./settings.json")]
    pub settings_path: String,
}

pub fn options() -> Opt {
    Opt::from_args()
}
