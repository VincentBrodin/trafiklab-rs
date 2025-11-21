use std::str::FromStr;

use trafiklab::{
    Request,
    resrobot::{Location, RouteRequest},
};

#[test]
fn resrobot_route_build_url() {
    let url = RouteRequest::new(
        "access_id".to_string(),
        Location::Id("10".to_string()),
        Location::Id("11".to_string()),
    )
    .search_for_arrival(true)
    .build_url()
    .unwrap();
    assert_eq!(url, url::Url::from_str("https://hello.com").unwrap())
}
