use trafiklab::{
    Request,
    resrobot::{Location, RouteRequest},
};

#[test]
fn resrobot_route_build_url() {
    let url = RouteRequest::new(
        "API_KEY".to_string(),
        Location::Id("10".to_string()),
        Location::Id("11".to_string()),
    )
    .with_search_for_arrival(true)
    .with_count_after(3)
    .with_count_before(3)
    .with_max_transfers(1)
    .build_url()
    .unwrap();
    println!("{}", url.as_str())
    // assert_eq!(url, url::Url::from_str("https://hello.com").unwrap())
}
