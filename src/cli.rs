use std::{str::FromStr, num::ParseIntError};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "oszdl",
    about = "A CLI beatmap downloader for the rhythm game osu!",
    author = "Shinobu#2469",
    version = "1.0"
)]
pub struct Cli {
    #[structopt[default_value=""]]
    pub query: String,

    #[structopt[name="mode", short="m", help="osu! game mode\n1 - std\n2 - taiko\n3 - catch\n4 - mania"]]
    pub mode: Option<Mode>,

    #[structopt[name="recommended", short="r", help="Recommended difficulty"]]
    pub recommended_difficulty: bool
}

#[derive(Debug)]
pub enum Mode {
    Standard,
    Taiko,
    Catch,
    Mania,
    Any
}

impl FromStr for Mode {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Mode::Standard),
            "1" => Ok(Mode::Taiko),
            "2" => Ok(Mode::Catch),
            "3" => Ok(Mode::Mania),
            _ => Ok(Mode::Any)
        }
    }

    type Err = ParseIntError;
}
