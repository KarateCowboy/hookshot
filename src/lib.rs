use url::{Host, Url};

pub struct Video {
    pub platform: Platform,
    // pub asset_id: String
}

#[derive(PartialEq, Eq, Debug)]
pub enum Platform {
    YouTube,
    Rumble,
    NicoVideo,
    BitChute,
    Unknown,
}

pub fn grok_url_arg(url_str: String) -> Url {
    let my_url = Url::parse(&url_str).expect("Invalid URL argument given");
    my_url
}

pub fn video_from_url(video_url: &str) -> Video {
    let my_url = grok_url_arg(video_url.to_owned().clone());
    let host_val = my_url.host_str().clone();
    println!("{:?}", host_val);
    return Video {
        platform: parse_platform(host_val.map(|s| s.to_string())),
        // asset_id: parse_asset_id()
    };
}
pub fn parse_platform(host_str: Option<String>) -> Platform {
    match host_str {
        Some(val) => {
            let val_lower = val.to_lowercase();
            match val_lower.as_str() {
                "www.youtube.com" => Platform::YouTube,
                "www.rumble.com" => Platform::Rumble,
                "www.nicovideo.jp" => Platform::NicoVideo,
                "www.bitchute.com" => Platform::BitChute,
                _ => Platform::Unknown,
            }
        }
        None => Platform::Unknown,
    }
}

pub fn parse_asset_id(url: &Url) -> Option<String> {
    return match url.query() {
        Some(v) => Some(String::from("found")),
        None => None,
    };
}
