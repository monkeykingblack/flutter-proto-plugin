use proto_pdk_test_utils::*;

mod flutter_toole {
    use super::*;

    generate_resolve_versions_tests!("flutter-test", {
        "0.4" => "0.4.0",
        "0.5.1" => "0.5.1",
        "1.1.0" => "1.1.0",
    });

    #[tokio::test(flavor = "multi_thread")]
    async fn load_versions_from_git() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("flutter-test").await;

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
