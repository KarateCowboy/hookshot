use spectral::prelude::*;

use url::{Host, Url};

use hookshot::grok_url_arg;

#[test]
fn grok_url_string() {
    let video_url = String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg");
    let result = grok_url_arg(video_url.clone());

    let expected_host = String::from("www.youtube.com");
    let found_host = result.host().expect("There was no host");
    match found_host {
        Host::Domain(val) => assert_that!(val).is_equal_to(&expected_host),
        _ => {
            throw_error!("Host value returned was not of type Domain");
        }
    }
    assert_that(&result.host()).is_some();
    // assert_that(&result.host().unwrap()).is_equal_to(&host_name);
    // assert_that!(video_url.parse::<Url>()).is_ok();
}
