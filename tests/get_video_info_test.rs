use spectral::prelude::*;

use url::{Url};

use hookshot::grok_url_arg;

#[test]
fn grok_url_string() {
    let video_url = String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg");
    let result = grok_url_arg(video_url.clone());
    assert_that!(video_url.parse::<Url>()).is_ok();
}

