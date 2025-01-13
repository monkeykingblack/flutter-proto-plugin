use crate::channel::Channel;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Release {
    pub channel: Channel,
    pub version: String,
    pub dart_sdk_arch: Option<String>,
    pub archive: String,
}

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseResponse {
    pub releases: Vec<Release>,
}
