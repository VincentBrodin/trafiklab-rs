mod request;
use std::fmt::{Display, Write};

pub use request::RouteRequest;

#[derive(Default)]
pub enum Language {
    #[default]
    Swedish,
    English,
    German,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Swedish => f.write_str("sv"),
            Language::English => f.write_str("en"),
            Language::German => f.write_str("de"),
        }
    }
}

pub struct Coordinate {
    pub latitude: f32,
    pub longitude: f32,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:.6}+{:.6}", self.latitude, self.longitude))
    }
}

pub enum Location {
    Id(String),
    Coordinate(Coordinate),
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Id(id) => f.write_str(id),
            Location::Coordinate(coordinate) => f.write_str(&coordinate.to_string()),
        }
    }
}
