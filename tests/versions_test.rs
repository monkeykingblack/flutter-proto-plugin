use proto_pdk_test_utils::*;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
pub struct FlutterPluginConfig {
    pub channel: String,
}
mod flutter_toole {
    use super::*;

    generate_resolve_versions_tests!("flutter-test", {
        "3.0" => "3.0.5",
        "3.10" => "3.10.6",
        "3.16.9" => "3.16.9",
    });

    #[tokio::test(flavor = "multi_thread")]
    async fn load_stable_versions() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("flutter-test").await;

        let output = plugin.load_versions(LoadVersionsInput::default()).await;

        assert!(!output.versions.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn load_beta_versions() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.tool_config(FlutterPluginConfig {
                    channel: "beta".into(),
                });
            })
            .await;

        let output = plugin.load_versions(LoadVersionsInput::default()).await;

        assert!(!output.versions.is_empty());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn sets_latest_alias() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("flutter-test").await;

        let output = plugin.load_versions(LoadVersionsInput::default()).await;

        assert!(output.latest.is_some());
        assert!(output.aliases.contains_key("latest"));
        assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn parse_flutter_sdk() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("flutter-test").await;

        assert_eq!(
            plugin
                .parse_version_file(ParseVersionFileInput {
                    content: r#"
                    environment:
                        sdk: ">= 2.0.0 <= 3.5.0"
                        flutter: ">=3.19.0"
                    "#
                    .into(),
                    file: "pubspec.yaml".into(),
                    ..Default::default()
                })
                .await,
            ParseVersionFileOutput {
                version: Some(UnresolvedVersionSpec::parse(">=3.19.0").unwrap()),
            }
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn flutter_environment_not_set() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("flutter-test").await;

        assert_eq!(
            plugin
                .parse_version_file(ParseVersionFileInput {
                    content: r#"
                    environment:
                        sdk: ">= 2.0.0 <= 3.5.0"
                    "#
                    .into(),
                    file: "pubspec.yaml".into(),
                    ..Default::default()
                })
                .await,
            ParseVersionFileOutput { version: None }
        );
    }
}
