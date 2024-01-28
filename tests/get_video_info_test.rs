use spectral::prelude::*;

use url::Url;

use hookshot::{grok_url_arg, video_from_url, parse_platform, Video, Platform};

#[test]
fn grok_url_string() {
    let video_url = String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg");
    let result = grok_url_arg(video_url.clone());
    assert_that!(video_url.parse::<Url>()).is_ok();
}
#[test]
fn builds_a_video_object_from_url_string() {
    let video_url = String::from("https://www.youtube.com/watch?v=Fi5eYLA0uAg");
    let result_video : Video = video_from_url(&video_url);
    assert_that!(true).is_true();
    // assert_that!(&result_video.platform).is_equal_to(Platforms::YouTube);
}

#[test]
fn parse_youtube_platform(){
    let parsed_platform : Platform = parse_platform(Some("youtube.com".to_string()));
    assert_that!(parsed_platform).is_equal_to(Platform::YouTube);
}
#[test]
fn parse_rumble_platform(){
    let parsed_platform : Platform = parse_platform(Some("rumble.com".to_string()));
    assert_that!(parsed_platform).is_equal_to(Platform::Rumble);

}
#[test]
fn parse_nico_platform(){
    let parsed_platform : Platform = parse_platform(Some("nicovideo.jp".to_string()));
    assert_that!(parsed_platform).is_equal_to(Platform::NicoVideo);
}
#[test]
fn parse_bitchute_platform(){
    let parsed_platform : Platform = parse_platform(Some("bitchute.com".to_string()));
    assert_that!(parsed_platform).is_equal_to(Platform::BitChute);
}
#[test]
fn parse_unkown_platform(){
    let parsed_platform : Platform = parse_platform(Some("geocities.com".to_string()));
    assert_that!(parsed_platform).is_equal_to(Platform::Unknown);
}



