mod request;
pub use request::RouteRequest;

#[derive(Default)]
pub enum Language {
    #[default]
    Swedish,
    English,
    German,
}

pub struct Coordinate {
    pub latitude: f32,
    pub longitude: f32,
}

pub enum Location {
    Id(String),
    Coordinate(Coordinate),
}
