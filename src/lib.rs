use url::{Url, Host};

pub fn grok_url_arg(url_str : String) -> Url {
    let my_url = Url::parse(&url_str).expect("Invalid URL argument given");
    my_url
}
