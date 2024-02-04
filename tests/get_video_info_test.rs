use spectral::prelude::*;
use url::Url;

use hookshot::{grok_url_arg, parse_asset_id, parse_platform, video_from_url, Platform, Video};

// #[test]
// fn grok_url_string() {
//     let video_url = String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg");
//     grok_url_arg(video_url.clone());
//     assert_that!(video_url.parse::<Url>()).is_ok();
// }

// #[test]
// fn parse_youtube_platform() {
//     let parsed_platform: Platform = parse_platform(Some("www.youtube.com".to_string()));
//     assert_that!(parsed_platform).is_equal_to(Platform::YouTube);
// }
// #[test]
// fn parse_rumble_platform() {
//     let parsed_platform: Platform = parse_platform(Some("www.rumble.com".to_string()));
//     assert_that!(parsed_platform).is_equal_to(Platform::Rumble);
// }
// #[test]
// fn parse_nico_platform() {
//     let parsed_platform: Platform = parse_platform(Some("www.nicovideo.jp".to_string()));
//     assert_that!(parsed_platform).is_equal_to(Platform::NicoVideo);
// }
// #[test]
// fn parse_bitchute_platform() {
//     let parsed_platform: Platform = parse_platform(Some("www.bitchute.com".to_string()));
//     assert_that!(parsed_platform).is_equal_to(Platform::BitChute);
// }
// #[test]
// fn parse_unkown_platform() {
//     let parsed_platform: Platform = parse_platform(Some("www.geocities.com".to_string()));
//     assert_that!(parsed_platform).is_equal_to(Platform::Unknown);
// }

// #[test]
// fn parse_yt_asset_id_in_query() {
//     let video_url = grok_url_arg(String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg"));
//     let asset_id = parse_asset_id(&video_url);
//     assert_that!(asset_id).is_equal_to(Some(String::from("Fi5eYLA0uAg")));
// }
// #[test]
// fn parse_yt_asset_id_in_url_segment() {
//     let video_url = grok_url_arg(String::from(
//         "https://youtu.be/Fi5eYLA0uAg?si=TMmFVQ6R7Zi8_6XV",
//     ));
//     let asset_id = parse_asset_id(&video_url);
//     assert_that!(asset_id).is_equal_to(Some(String::from("Fi5eYLA0uAg")));
// }

// #[test]
// fn builds_a_video_object_from_url_string() {
//     let video_url = String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg");
//     let result_video: Video = video_from_url(&video_url);
//     assert_that!(result_video.platform).is_equal_to(Platform::YouTube);
// }

#[macro_use]
extern crate spectral;

#[test]
fn grok_url_string() {
    let video_url = String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg");
    grok_url_arg(video_url.clone());
    asserting!("URL should parse correctly")
        .that(&video_url.parse::<Url>())
        .is_ok();
}
#[test]
fn it_parses_the_youtube_url() {
    let parsed_platform: Platform = parse_platform(Some("www.youtube.com".to_string()));
    asserting!("Platform should be YouTube")
        .that(&parsed_platform)
        .is_equal_to(&Platform::YouTube);
}

#[test]
fn parse_rumble_platform() {
    let parsed_platform: Platform = parse_platform(Some("www.rumble.com".to_string()));
    asserting!("Platform should be Rumble")
        .that(&parsed_platform)
        .is_equal_to(&Platform::Rumble);
}

#[test]
fn parse_nico_platform() {
    let parsed_platform: Platform = parse_platform(Some("www.nicovideo.jp".to_string()));
    asserting!("Platform should be NicoVideo")
        .that(&parsed_platform)
        .is_equal_to(&Platform::NicoVideo);
}

#[test]
fn parse_bitchute_platform() {
    let parsed_platform: Platform = parse_platform(Some("www.bitchute.com".to_string()));
    asserting!("Platform should be BitChute")
        .that(&parsed_platform)
        .is_equal_to(&Platform::BitChute);
}

#[test]
fn parse_unknown_platform() {
    let parsed_platform: Platform = parse_platform(Some("www.geocities.com".to_string()));
    asserting!("Platform should be Unknown")
        .that(&parsed_platform)
        .is_equal_to(&Platform::Unknown);
}

#[test]
fn parse_yt_asset_id_in_query() {
    let video_url = grok_url_arg(String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg"));
    let asset_id = parse_asset_id(&video_url);
    asserting!("Asset ID should match")
        .that(&asset_id)
        .is_equal_to(&Some(String::from("Fi5eYLA0uAg")));
}

#[test]
fn parse_yt_asset_id_in_url_segment() {
    let video_url = grok_url_arg(String::from(
        "https://youtu.be/Fi5eYLA0uAg?si=TMmFVQ6R7Zi8_6XV",
    ));
    let asset_id = parse_asset_id(&video_url);
    asserting!("Asset ID should match")
        .that(&asset_id)
        .is_equal_to(&Some(String::from("Fi5eYLA0uAg")));
}

#[test]
fn builds_a_video_object_from_url_string() {
    let video_url = String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg");
    let result_video: Video = video_from_url(&video_url);
    asserting!("Video object should have YouTube platform")
        .that(&result_video.platform)
        .is_equal_to(&Platform::YouTube);
}
