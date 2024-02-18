use cucumber::{given, then, when, World};
use hookshot::{parse, Platform, Video};
use lazy_static::lazy_static;
use mockito::{Server, ServerGuard};
use serde::{Deserialize, Serialize};
use serde_json;
use spectral::prelude::*;
use std::fmt;
use url::Url;

extern crate spectral;
lazy_static! {
    static ref MOCK_SERVER_MUTEX: tokio::sync::Mutex<ServerGuard> =
        tokio::sync::Mutex::new(Server::new());
}
#[derive(Deserialize, Serialize, Debug)]
struct ApiResponse {
    greeting: String,
}

// `World` is your shared, likely mutable state.
// Cucumber constructs it via `Default::default()` for each scenario.
#[derive(World, Debug)]
pub struct HookshotWorld {
    proper_video_url: Url,
    url_string: String,
    resulting_platform: Platform,
    asset_id: Option<String>,
    expected_asset_id: Option<String>,
    video: Video,
}

struct ServerWrapper(ServerGuard);

impl fmt::Debug for ServerWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Since ServerGuard does not implement Debug, you cannot print its internals.
        // You can print a placeholder, or any other debug information you see fit.
        write!(f, "ServerGuardWrapper {{ /* ServerGuard instance */ }}")
    }
}

impl Default for HookshotWorld {
    fn default() -> Self {
        HookshotWorld {
            proper_video_url: Url::parse("http://example.com").unwrap(),
            resulting_platform: Platform::Unknown,
            url_string: String::new(),
            asset_id: None,
            expected_asset_id: None,
            video: Video {
                platform: Platform::Unknown,
                asset_id: None,
            },
        }
    }
}
#[given("I have a youtube URL string")]
fn have_a_youtube_url_string(world: &mut HookshotWorld) {
    world.url_string = String::from("https://www.youtube.com/watch?v=crEz8i6oVpI");
    world.expected_asset_id = Some("crEz8i6oVpI".to_string());
}

#[given("I have a youtube URL with no asset id")]
fn have_youtube_url_no_asset_id(world: &mut HookshotWorld) {
    world.url_string = String::from("https://www.youtube.com/watch?");
    world.expected_asset_id = None;
}
#[given("I have a neocities URL string")]
fn have_neocities_url_string(world: &mut HookshotWorld) {
    world.url_string = String::from("https://distantskies.neocities.org/")
}
#[given("I have a proper youtube.com video URL")]
fn have_proper_yt_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from(
        "https://www.youtube.com/crEz8i6oVpI?si=ayJIRnyyVZkyyp7F",
    ))
    .expect("Invalid URL argument given");
}
#[given("I have a proper youtu.be video URL")]
fn have_proper_ytbe_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from(
        "https://youtu.be/crEz8i6oVpI?si=ayJIRnyyVZkyyp7F",
    ))
    .expect("Invalid URL argument given");
}
#[given("I have a proper rumble video URL")]
fn have_proper_rumble_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from(
        "https://www.rumble.com/vn30ur-unipulator-last-blade-2-kizukaseru.html",
    ))
    .expect("Invalid URL argument given");
    world.expected_asset_id = Some("vn30ur-unipulator-last-blade-2-kizukaseru".to_string());
}
#[given("I have a proper Nico Video URL")]
fn have_proper_nico_video_url(world: &mut HookshotWorld) {
    world.proper_video_url =
        Url::parse(&String::from("http://www.nicovideo.jp")).expect("Invalid URL argument given");
}

#[given("I have a proper Bitchute video URL")]
fn have_proper_bitchute_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from(
        "https://www.bitchute.com/video/LUhMTHrgKBg8/",
    ))
    .expect("Invalid URL argument given");
}

#[given("I have a proper Fartstream video URL")]
fn have_proper_fartstream_video_url(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from(
        "https://www.fartstream.com/video/LUhMTHrgKBg8/",
    ))
    .expect("Invalid URL argument given");
}

#[when("I parse it via parse_platform")]
fn parse_via_parse_platform(world: &mut HookshotWorld) {
    let host = world
        .proper_video_url
        .host_str()
        .clone()
        .map(|s| s.to_string());
    world.resulting_platform = parse::parse_platform(host.clone());
}

#[when("I parse it via video_from_url")]
fn parse_via_video_from_url(world: &mut HookshotWorld) {
    world.video = parse::video_from_url(&world.url_string);
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
fn have_yt_url_with_asset_id_as_segment(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from("https://youtu.be/BvYuf4r-8xk"))
        .expect("Invalid URL argument given");
    world.expected_asset_id = Some("BvYuf4r-8xk".to_string());
}

#[given("I have a youtube url where the asset id is part of the query")]
fn have_yt_url_with_asset_id_as_query(world: &mut HookshotWorld) {
    world.proper_video_url =
        Url::parse(&String::from("https://www.youtube.com/watch?v=BvYuf4r-8xk"))
            .expect("Invalid URL argument given");
    world.expected_asset_id = Some("BvYuf4r-8xk".to_string());
}
#[given("I have a youtube URL where there is a query but no asset_id")]
fn have_yt_url_with_query_but_no_asset_id(world: &mut HookshotWorld) {
    world.proper_video_url = Url::parse(&String::from("https://www.youtube.com/watch?t=701s"))
        .expect("Invalid URL argument given");
}

#[when("I parse it with asset_id")]
fn parse_it_with_asset_id(world: &mut HookshotWorld) {
    world.asset_id = parse::asset_id(&world.proper_video_url)
}

#[then("I should have the correct video id as string")]
fn have_correct_video_id(world: &mut HookshotWorld) {
    assert_eq!(world.asset_id, world.expected_asset_id);
}
#[then("I should have a None result for the asset_id")]
fn asset_id_should_be_none(world: &mut HookshotWorld) {
    assert!(world.asset_id.is_none());
}
#[then("the video object should have the parsed properties")]
fn video_has_parsed_properties(world: &mut HookshotWorld) {
    asserting!("platform is youtube variant")
        .that(&world.video.platform)
        .is_equal_to(Platform::YouTube);
    asserting!("asset_id is correctly set")
        .that(&world.video.asset_id)
        .is_equal_to(&world.expected_asset_id);
}

#[then("the video object should have no asset id")]
fn no_asset_id_on_video_object(world: &mut HookshotWorld) {
    asserting!("The video.asset_id is None")
        .that(&world.video.asset_id)
        .is_equal_to(&world.expected_asset_id);
}
#[then("the video object should have no asset id and an unknown platform")]
fn unknown_platform_on_video(world: &mut HookshotWorld) {
    asserting!("The platform is Unknown")
        .that(&world.video.platform)
        .is_equal_to(Platform::Unknown);
    asserting!("The asset_id is None")
        .that(&world.video.asset_id)
        .is_equal_to(None);
}

#[given("I have a youtube Video for which I contact the API")]
async fn have_yt_video(world: &mut HookshotWorld) {
    let mut server = MOCK_SERVER_MUTEX.lock().await;
    server
        .mock("GET", "/greetings")
        // .match_header("content-type", "application/json")
        // .match_body(mockito::Matcher::PartialJsonString(
        //     "{\"greeting\": \"hello\"}".to_string(),
        // ))
        .with_body("{\"greeting\": \"hello\"}")
        .create();
    world.video = Video {
        platform: Platform::YouTube,
        asset_id: Some("crEz8i6oVpI".to_string()),
    }
}

#[when("I fetch the metadata for it")]
async fn fetch_metadata(world: &mut HookshotWorld) {
    let server = MOCK_SERVER_MUTEX.lock().await;
    let url = format!("{}/greetings", server.url());

    let response = reqwest::get(&url).await;
    // match response {
    //     Ok(response_val) => {
    //         println!("{:?}", response_val);
    //         let content = response_val.json::<ApiResponse>().await;
    //         match content {
    //             Ok(content_body) => println!("here is the content body {:?}", content_body),
    //             Err(e) => {
    //                 println!("there was a content error {:?}", e);
    //                 println!("blarg");
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         println!("there was an error {:?}", e);
    //         println!("stuff");
    //     }
    // }
    // match response {
    //     Ok(response_val) => {
    //         println!("Response: {:?}", response_val);

    //         // Ensure we have a successful status code before attempting to parse JSON
    //         if response_val.status().is_success() {
    //             match response_val.json::<ApiResponse>().await {
    //                 Ok(content_body) => println!("Here is the content body: {:?}", content_body),
    //                 Err(e) => {
    //                     println!("There was a content error: {:?}", e);
    //                 }
    //             }
    //         } else {
    //             println!("Response was not successful: {:?}", response_val.status());
    //         }
    //     }
    //     Err(e) => {
    //         println!("There was an error making the request: {:?}", e);
    //     }
    // }
    match response {
        Ok(response_val) => {
            // Print the status code and headers
            println!("Response status: {}", response_val.status());
            for (key, value) in response_val.headers().iter() {
                println!("{}: {:?}", key, value);
            }

            // Check if the response was successful
            if response_val.status().is_success() {
                // Attempt to get the response text for debugging purposes
                let text = response_val.text().await;
                match text {
                    Ok(body) => {
                        println!("Response body text: {}", body);
                        // Attempt to parse the response body text as JSON
                        let json_result: Result<ApiResponse, _> = serde_json::from_str(&body);
                        match json_result {
                            Ok(content_body) => println!("Parsed JSON content: {:?}", content_body),
                            Err(e) => println!("JSON parsing error: {:?}", e),
                        }
                    }
                    Err(e) => println!("Error getting response text: {:?}", e),
                }
            } else {
                println!("Response was not successful: {}", response_val.status());
            }
        }
        Err(e) => {
            println!("There was an error making the request: {:?}", e);
        }
    }
}
// This runs before everything else, so you can setup things here.
#[tokio::main]
async fn main() {
    // let _server = MOCK_SERVER_MUTEX.lock().unwrap();
    HookshotWorld::run("tests/features/lib/populate_metadata.feature").await;
    // HookshotWorld::run("tests/features/lib/parse_video_url_string.feature").await;
    // HookshotWorld::run("tests/features/lib/make_video.feature").await;
}
