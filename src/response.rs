use serde::Deserialize;
use std::fmt::{Display, Formatter, Result};

const ILLEGAL_CHARS: &[char] = &['\\', '?', '<', '>', '/', ':', '*', '|', '"'];

#[derive(Deserialize, Debug)]
pub struct Response {
    pub beatmapsets: Vec<Beatmap>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Beatmap {
    artist: String,
    creator: String,
    title: String,
    pub id: usize,
}

impl Beatmap {
    // This is needed because filenames can't have some special characters on windows
    pub fn sanitized_name(&self) -> String {
        let formatted_name = format!("{}", self);
        let mut sanitized_name = formatted_name.clone();

        for c in formatted_name.chars() {
            if ILLEGAL_CHARS.contains(&c) {
                sanitized_name = formatted_name.replace(c, "_");
            }
        }

        sanitized_name
    }
}

impl Display for Beatmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} - {} ({})", self.artist, self.title, self.creator)
    }
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub location: String,
}
