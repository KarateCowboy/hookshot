use cucumber::{given, then, when, World};
use hookshot::{parse_platform, Platform};
use url::Url;

// These `Cat` definitions would normally be inside your project's code,
// not test code, but we create them here for the show case.
#[derive(Debug, Default)]
struct Cat {
    pub hungry: bool,
}

impl Cat {
    fn feed(&mut self) {
        self.hungry = false;
    }
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(Debug, Default, World)]
pub struct HookshotWorld {
    proper_video_url: Option<String>,
    resulting_platform: Platform,
}
#[given("I have a proper youtube video URL")]
fn have_proper_yt_video_url(world: &mut HookshotWorld) {
    let my_url = Url::parse(&String::from("http://www.youtube.com")).expect("Invalid URL argument given");
    let host = my_url.host_str().clone().map(|s| s.to_string());
    world.proper_video_url = host;
}
#[given("I have a proper rumble video URL")]
fn have_proper_rumble_video_url(world: &mut HookshotWorld) {
    let my_url = Url::parse(&String::from("https://www.rumble.com/vn30ur-unipulator-last-blade-2-kizukaseru.html")).expect("Invalid URL argument given");
    let host = my_url.host_str().clone().map(|s| s.to_string());
    world.proper_video_url = host;
}
#[when("I parse it via parse_platform")]
fn parse_via_parse_platform(world: &mut HookshotWorld) {
    world.resulting_platform = parse_platform(world.proper_video_url.clone());
}
#[then("the resulting platform should be the Youtube variant of the enum")]
fn result_should_be_yt_enum(world: &mut HookshotWorld) {
    assert_eq!(world.resulting_platform, Platform::YouTube);
}
#[then("the resulting platform should be the Rumble variant of the enum")]
fn result_should_be_rumble_enum(world: &mut HookshotWorld) {
    assert_eq!(world.resulting_platform, Platform::Rumble);
}
// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(HookshotWorld::run("tests/features/lib/thing.feature"));
}
