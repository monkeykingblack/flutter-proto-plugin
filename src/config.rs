use crate::channel::Channel;

#[derive(Debug, schematic::Schematic, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct FlutterPluginConfig {
    pub dist_url: String,
    pub channel: Channel,
}

impl Default for FlutterPluginConfig {
    fn default() -> Self {
        Self {
            dist_url: "https://storage.googleapis.com/flutter_infra_release/releases/{archive}"
                .into(),
            channel: Channel::Stable,
        }
    }
}
