use cucumber::{given, when, then, World};
use hookshot::{Platform, parse_platform};

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
    proper_yt_url: Option<String>,
    resulting_platform: Platform

}

#[given("I have a proper youtube video URL")]
fn have_proper_yt_video_url(world: &mut HookshotWorld){
   world.proper_yt_url = Some(String::from("www.youtube.com/watch?v=Fi5eYLA0uAg"));
}

#[when("I parse it via parse_platform")]
fn parse_via_parse_platform(world: &mut HookshotWorld){
    world.resulting_platform = parse_platform(world.proper_yt_url.clone());
}

#[then("the resulting platform should be the Youtube variant of the enum")]
fn result_should_be_yt_enum(world: &mut HookshotWorld){
    assert_eq!(world.resulting_platform, Platform::YouTube);
}
// This runs before everything else, so you can setup things here.
fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(HookshotWorld::run("tests/features/lib/thing.feature"));
}
