use crate::{
    Request,
    resrobot::{Language, Location},
};

pub struct RouteRequest {
    access_id: String,
    language: Language,
    search_for_arrival: bool,
    origin: Location,
    destination: Location,
}

impl RouteRequest {
    pub fn new(access_id: String, origin: Location, destination: Location) -> Self {
        Self {
            access_id: access_id,
            origin,
            destination,
            language: Default::default(),
            search_for_arrival: false,
        }
    }

    pub fn search_for_arrival(mut self, value: bool) -> Self {
        self.search_for_arrival = value;
        self
    }
}

impl Request for RouteRequest {
    type Output = String;

    fn send(self) -> impl Future<Output = Result<Self::Output, crate::Error>> + Send {
        println!("SENT");
        async move { Ok("test".to_string()) }
    }

    fn build_url(self) -> Result<reqwest::Url, crate::Error> {
        let url = reqwest::Url::parse("https://hello.com")?;
        Ok(url)
    }
}
