use std::time::Instant;

use trafiklab::{
    Request,
    resrobot::{Location, RouteRequest},
};

#[test]
#[cfg(feature = "stress")]
fn resrobot_route_build_url_stress_test() {
    let request = RouteRequest::new(
        "API_KEY".to_string(),
        Location::Id("10".to_string()),
        Location::Id("11".to_string()),
    )
    .with_search_for_arrival(true)
    .with_count_after(3)
    .with_count_before(3)
    .with_max_transfers(1);

    const ITERATIONS: u32 = 10_000_000;
    // warm-up to avoid measuring one-time allocations
    let _ = request.build_url();

    let start = Instant::now();
    for _ in 0..ITERATIONS {
        let _ = request.build_url().unwrap();
    }
    let total = start.elapsed();

    let avg = total / ITERATIONS;

    println!("Total: {:?} for {ITERATIONS} iterations", total);
    println!("Average: {:?} per call", avg);
    let url = request.build_url().unwrap();
    println!("{}", url.as_str())
    // assert_eq!(url, url::Url::from_str("https://hello.com").unwrap())
}
