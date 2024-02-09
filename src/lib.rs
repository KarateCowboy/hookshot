use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApiResponse {
    items: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
    snippet: Snippet,
}

#[derive(Deserialize)]
struct Snippet {
    title: String,
    description: String,
    channel_name: String,
    published_at: String,
}

#[derive(Debug, Clone)]
pub struct Video {
    pub platform: Platform,
    pub asset_id: Option<String>
}
#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub enum Platform {
    #[default]
    YouTube,
    Rumble,
    NicoVideo,
    BitChute,
    Unknown,
}

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let video_id = "K1Yh9eIiY2w"; // Extract this from the URL
//     let api_key = "YOUR_API_KEY"; // Use your actual API key here
//     let url = format!("https://www.googleapis.com/youtube/v3/videos?id={}&key={}&part=snippet", video_id, api_key);

//     let response = reqwest::get(url)
//         .await?
//         .json::<ApiResponse>()
//         .await?;

//     if let Some(item) = response.items.first() {
//         println!("Title: {}", item.snippet.title);
//         println!("Description: {}", item.snippet.description);
//         println!("Channel: {}", item.snippet.channelTitle);
//         println!("Upload Date: {}", item.snippet.publishedAt);
//     } else {
//         println!("No metadata found for video ID: {}", video_id);
//     }

//     Ok(())
// }
pub fn metadata(video: &Video) -> Result<Video, Error> {
  return Ok(video.clone());
}
pub mod parse {
    use crate::{Platform, Video};
    use url::Url;
    pub fn grok_url_arg(url_str: String) -> Url {
        let my_url = Url::parse(&url_str).expect("Invalid URL argument given");
        my_url
    }
    pub fn parse_platform(host_str: Option<String>) -> Platform {
        match host_str {
            Some(val) => {
                let val_lower = val.to_lowercase();
                match val_lower.as_str() {
                    "www.youtube.com" => Platform::YouTube,
                    "youtu.be" => Platform::YouTube,
                    "www.rumble.com" => Platform::Rumble,
                    "www.nicovideo.jp" => Platform::NicoVideo,
                    "www.bitchute.com" => Platform::BitChute,
                    _ => Platform::Unknown,
                }
            }
            None => Platform::Unknown,
        }
    }
    pub fn video_from_url(video_url: &str) -> Video {
        let my_url = grok_url_arg(video_url.to_owned().clone());
        let host_val = my_url.host_str().clone();
        return Video {
            platform: parse_platform(host_val.map(|s| s.to_string())),
            asset_id: asset_id(&my_url)
        };
    }
    pub fn asset_id(url: &Url) -> Option<String> {
        let platform = parse_platform(url.host_str().map(|s| s.to_string()));
        match platform {
            Platform::YouTube => parse_yt_asset_id(&url),
            Platform::Rumble => parse_rumble_asset_id(&url),
            _ => None,
        }
    }

    fn parse_yt_asset_id(url: &Url) -> Option<String> {
        return match url.path_segments().map(|c| c.collect::<Vec<&str>>()) {
            Some(segment_vec) => match find_youtube_id(segment_vec) {
                Some(id) => Some(id),
                None => match url.query() {
                    Some(query) => find_param_value(&query, "v"),
                    None => None,
                },
            },
            None => None,
        };
    }

    fn parse_rumble_asset_id(url: &Url) -> Option<String> {
        return match url.path_segments().map(|c| c.collect::<Vec<&str>>()) {
            Some(segment_vec) => match find_rumble_id(segment_vec) {
                Some(id) => Some(id),
                None => match url.query() {
                    Some(query) => find_param_value(&query, "v"),
                    None => None,
                },
            },
            None => None,
        };
    }

    fn find_param_value(query: &str, param: &str) -> Option<String> {
        return query.split('&').find_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) if key == param => Some(value.to_string()),
                _ => None,
            }
        });
    }
    fn find_youtube_id(v: Vec<&str>) -> Option<String> {
        return v
            .into_iter()
            .find(|s| {
                s.len() == 11
                    && s.chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
            })
            .map(|s| s.to_string());
    }
    fn find_rumble_id(v: Vec<&str>) -> Option<String> {
        return v
            .into_iter()
            .find(|s| {
                s.chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
            })
            .map(|s| s.to_string().replace(".html", ""));
    }
}
