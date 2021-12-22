use std::fmt::{Display, Formatter, Result};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub beatmapsets: Vec<Beatmap>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Beatmap {
    artist: String,
    creator: String,
    title: String,
    pub id: usize 
}

impl Display for Beatmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} - {} ({})", self.artist, self.title, self.creator)
    }
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub location: String
}
