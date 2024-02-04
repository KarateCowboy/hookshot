use url::Url;

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
    return match url.path_segments().map(|c| c.collect::<Vec<&str>>()) {
        Some(segment_vec) => match find_youtube_id(segment_vec) {
            Some(id) => Some(id),
            None => {
                match url.query() {
                    Some(query) => find_param_value(&query, "v"),
                    None => None
                }
            }
        },
        None => None
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
    println!("{:?}", v);
    return v
        .into_iter()
        .find(|s| {
            s.len() == 11
                && s.chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        })
        .map(|s| s.to_string());
}
