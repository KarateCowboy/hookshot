use spectral::prelude::*;
use url::Url;

use hookshot::{parse, Platform, Video};

extern crate spectral;


#[test]
fn grok_url_string() {
    let video_url = String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg");
    parse::grok_url_arg(video_url.clone());
    asserting!("URL should parse correctly")
        .that(&video_url.parse::<Url>())
        .is_ok();
}

#[test]
fn parse_yt_asset_id_in_query() {
    let video_url =
        parse::grok_url_arg(String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg"));
    let asset_id = parse::asset_id(&video_url);
    asserting!("Asset ID should match")
        .that(&asset_id)
        .is_equal_to(&Some(String::from("Fi5eYLA0uAg")));
}

#[test]
fn parse_yt_asset_id_in_url_segment() {
    let video_url = parse::grok_url_arg(String::from(
        "https://youtu.be/Fi5eYLA0uAg?si=TMmFVQ6R7Zi8_6XV",
    ));
    let asset_id = parse::asset_id(&video_url);
    asserting!("Asset ID should match")
        .that(&asset_id)
        .is_equal_to(&Some(String::from("Fi5eYLA0uAg")));
}

#[test]
fn builds_a_video_object_from_url_string() {
    let video_url = String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg");
    let result_video: Video = parse::video_from_url(&video_url);
    asserting!("Video object should have YouTube platform")
        .that(&result_video.platform)
        .is_equal_to(&Platform::YouTube);
}
