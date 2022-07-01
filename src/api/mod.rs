pub struct YouTubeConfig {
    api_key: String,
    region: String,
    proxy_region: Option<String>,
}

impl std::default::Default for YouTubeConfig {
    fn default() -> Self {
        YouTubeConfig {
            api_key: String::from("AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8"),
            region: String::from("US"),
            proxy_region: None,
        }
    }
}
