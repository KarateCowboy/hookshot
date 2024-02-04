use cucumber::{given, then, when, World};
use hookshot::{parse_platform, Platform};
use url::Url;


// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, World)]
pub struct HookshotWorld {
    proper_video_url: Url, //Option<String>,
    resulting_platform: Platform,
}

impl Default for HookshotWorld {
    fn default() -> Self {
        HookshotWorld {
            proper_video_url: Url::parse("http://example.com").unwrap(), // Provide a sensible default URL
            resulting_platform: Platform::Unknown
            // initialize other fields
        }
    }
}
#[given("I have a proper youtube video URL")]
fn have_proper_yt_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from("http://www.youtube.com")).expect("Invalid URL argument given");
}
#[given("I have a proper rumble video URL")]
fn have_proper_rumble_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from("https://www.rumble.com/vn30ur-unipulator-last-blade-2-kizukaseru.html")).expect("Invalid URL argument given");
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
// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(HookshotWorld::run("tests/features/lib/thing.feature"));
}
