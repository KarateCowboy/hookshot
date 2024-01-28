use url::{Host, Url};


pub struct Video {
    pub site_id: String,
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
    // my_url.
    return Video {
        site_id: String::from("blarg"),
    };
}
pub fn parse_platform(host_str: Option<String>) -> Platform {
    match host_str {
        Some(val) => {
            let val_lower = val.to_lowercase();
            match val_lower.as_str() {
                "youtube.com" => Platform::YouTube,
                "rumble.com" => Platform::Rumble,
                "nicovideo.jp" => Platform::NicoVideo,
                "bitchute.com" => Platform::BitChute,
                _ => Platform::Unknown,
            }
        }
        None => Platform::Unknown,
    }
}
