use cucumber::{given, then, when, World};
use hookshot::{parse_platform, asset_id, Platform};
use url::Url;


// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, World)]
pub struct HookshotWorld {
    proper_video_url: Url,
    resulting_platform: Platform,
    asset_id : Option<String>,
    expected_asset_id: Option<String>
}

impl Default for HookshotWorld {
    fn default() -> Self {
        HookshotWorld {
            proper_video_url: Url::parse("http://example.com").unwrap(), // Provide a sensible default URL
            resulting_platform: Platform::Unknown,
            asset_id: None,
            expected_asset_id: None
        }
    }
}
#[given("I have a proper youtube.com video URL")]
fn have_proper_yt_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from("https://www.youtube.com/K1Yh9eIiY2w?si=ayJIRnyyVZkyyp7F")).expect("Invalid URL argument given");
}
#[given("I have a proper youtu.be video URL")]
fn have_proper_ytbe_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from("https://youtu.be/K1Yh9eIiY2w?si=ayJIRnyyVZkyyp7F")).expect("Invalid URL argument given");
}
#[given("I have a proper rumble video URL")]
fn have_proper_rumble_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from("https://www.rumble.com/vn30ur-unipulator-last-blade-2-kizukaseru.html")).expect("Invalid URL argument given");
    world.expected_asset_id = Some("vn30ur-unipulator-last-blade-2-kizukaseru".to_string());
}
#[given("I have a proper Nico Video URL")]
fn have_proper_nico_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from("http://www.nicovideo.jp")).expect("Invalid URL argument given");
}

#[given("I have a proper Bitchute video URL")]
fn have_proper_bitchute_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from("https://www.bitchute.com/video/LUhMTHrgKBg8/")).expect("Invalid URL argument given");
}

#[given("I have a proper Fartstream video URL")]
fn have_proper_fartstream_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from("https://www.fartstream.com/video/LUhMTHrgKBg8/")).expect("Invalid URL argument given");
}

#[when("I parse it via parse_platform")]
fn parse_via_parse_platform(world: &mut HookshotWorld) {
    let host = world.proper_video_url.host_str().clone().map(|s| s.to_string());
    world.resulting_platform = parse_platform(host.clone());
}
#[then("the resulting platform should be the Youtube variant of the enum")]
fn result_should_be_yt_enum(world: &mut HookshotWorld) {
    assert_eq!(world.resulting_platform, Platform::YouTube);
}
#[then("the resulting platform should be the Rumble variant of the enum")]
fn result_should_be_rumble_enum(world: &mut HookshotWorld) {
    assert_eq!(world.resulting_platform, Platform::Rumble);
}
#[then("the resulting platform should be the Nico variant of the enum")]
fn result_should_be_nico_enum(world: &mut HookshotWorld) {
    assert_eq!(world.resulting_platform, Platform::NicoVideo);
}
#[then("the resulting platform should be the BitChute variant of the enum")]
fn result_should_be_bitchute_enum(world: &mut HookshotWorld) {
    assert_eq!(world.resulting_platform, Platform::BitChute);
}
#[then("the resulting platform should be the Unknown variant of the enum")]
fn result_should_be_unknown_enum(world: &mut HookshotWorld) {
    assert_eq!(world.resulting_platform, Platform::Unknown);
}

#[given("I have a youtube url where the asset id is part of the url segment")]
fn have_yt_url_with_asset_id_as_segment(world: &mut HookshotWorld){
    world.proper_video_url = Url::parse(&String::from("https://youtu.be/BvYuf4r-8xk")).expect("Invalid URL argument given");
    world.expected_asset_id = Some("BvYuf4r-8xk".to_string());
}

#[given("I have a youtube url where the asset id is part of the query")]
fn have_yt_url_with_asset_id_as_query(world: &mut HookshotWorld){
    world.proper_video_url = Url::parse(&String::from("https://www.youtube.com/watch?v=BvYuf4r-8xk")).expect("Invalid URL argument given");
    world.expected_asset_id = Some("BvYuf4r-8xk".to_string());
}
#[given("I have a youtube URL where there is a query but no asset_id")]
fn have_yt_url_with_query_but_no_asset_id(world: &mut HookshotWorld){
    world.proper_video_url = Url::parse(&String::from("https://www.youtube.com/watch?t=701s")).expect("Invalid URL argument given");
}

#[when("I parse it with asset_id")]
fn parse_it_with_asset_id(world: &mut HookshotWorld){
    world.asset_id = asset_id(&world.proper_video_url)
}

#[then("I should have the correct video id as string")]
fn have_correct_video_id(world: &mut HookshotWorld){
    assert_eq!(world.asset_id, world.expected_asset_id);
}
#[then("I should have a None result for the asset_id")]
fn asset_id_should_be_none(world: &mut HookshotWorld){
    assert!(world.asset_id.is_none());
}

// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(HookshotWorld::run("tests/features/lib/parse_video_url_string.feature"));
}
