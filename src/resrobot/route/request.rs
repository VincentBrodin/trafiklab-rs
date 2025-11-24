use chrono::Utc;

use crate::Request;
use crate::resrobot::{Language, Location};

pub struct RouteRequest {
    access_id: String,
    language: Language,
    search_for_arrival: bool,
    origin: Location,
    destination: Location,
    datetime: chrono::DateTime<Utc>,
    count_before: u8,
    count_after: u8,
    max_transfer: Option<u8>,
}

impl RouteRequest {
    pub fn new(access_id: String, origin: Location, destination: Location) -> Self {
        Self {
            access_id: access_id,
            origin,
            destination,
            language: Default::default(),
            search_for_arrival: false,
            datetime: chrono::Utc::now(),
            count_after: 5,
            count_before: 0,
            max_transfer: None,
        }
    }

    pub fn with_search_for_arrival(mut self, value: bool) -> Self {
        self.search_for_arrival = value;
        self
    }

    pub fn with_time(mut self, value: chrono::DateTime<Utc>) -> Self {
        self.datetime = value;
        self
    }

    pub fn with_count_after(mut self, value: u8) -> Self {
        // the sum of count after and count before can't be more then 6
        assert!(self.count_before + value <= 6);
        self.count_after = value;
        self
    }
    pub fn with_count_before(mut self, value: u8) -> Self {
        // the sum of count after and count before can't be more then 6
        assert!(value + self.count_after <= 6);
        self.count_before = value;
        self
    }

    pub fn with_max_transfers(mut self, value: u8) -> Self {
        // has to be between 1 and 3
        assert!((1..4).contains(&value));
        self.max_transfer = Some(value);
        self
    }
}

impl Request for RouteRequest {
    type Output = String;

    fn send(self) -> impl Future<Output = Result<Self::Output, crate::Error>> + Send {
        println!("SENT");
        async move { Ok("test".to_string()) }
    }

    fn build_url(&self) -> Result<reqwest::Url, crate::Error> {
        // Gotten from https://www.trafiklab.se/api/our-apis/resrobot-v21/route-planner/
        const PARAM_COUNT: usize = 14;
        let mut params: Vec<(&str, &str)> = Vec::with_capacity(PARAM_COUNT);
        params.push(("format", "json"));
        params.push(("accessId", &self.access_id));
        let lang = self.language.to_string();
        params.push(("lang", &lang));
        let origin = self.origin.to_string();
        params.push(("orign", &origin));
        let dest = self.origin.to_string();
        params.push(("dest", &dest));
        let url = reqwest::Url::parse_with_params("https://api.resrobot.se/v2.1/trip", params)?;
        Ok(url)
    }
}
