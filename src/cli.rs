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
}
