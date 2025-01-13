use proto_pdk_test_utils::*;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
struct FlutterPluginConfig {
    pub channel: String,
}

mod flutter_tool {

    use super::*;

    generate_download_install_tests!("flutter-test", "3.24.5");

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_linux_arm64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::Linux, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.24.5").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                archive_prefix: Some("flutter".into()),
                download_url: "https://storage.googleapis.com/flutter_infra_release/releases/stable/linux/flutter_linux_3.24.5-stable.tar.xz".into(),
                ..Default::default()
            }
        )
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_macos_x64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::MacOS, HostArch::X64);
            })
            .await;
        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.24.5").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                archive_prefix: Some("flutter".into()),
                download_url: "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_3.24.5-stable.zip".into(),
                ..Default::default()
            }
        )
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn supports_macos_arm64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::MacOS, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.24.5").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                archive_prefix: Some("flutter".into()),
                download_url: "https://storage.googleapis.com/flutter_infra_release/releases/stable/macos/flutter_macos_arm64_3.24.5-stable.zip".into(),
                ..Default::default()
            }
        )
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn support_windows_x64() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::Windows, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .download_prebuilt(DownloadPrebuiltInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.24.5").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await,
            DownloadPrebuiltOutput {
                archive_prefix: Some("flutter".into()),
                download_url: "https://storage.googleapis.com/flutter_infra_release/releases/stable/windows/flutter_windows_3.24.5-stable.zip".into(),
                ..Default::default()
            }
        )
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_unix_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::Linux, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("3.24.5").unwrap(),
                        ..Default::default()
                    },
                })
                .await
                .exes
                .get("flutter")
                .unwrap()
                .exe_path,
            Some("bin/flutter".into())
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_windows_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::Linux, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: ToolContext {
                        version: VersionSpec::parse("1.2.0").unwrap(),
                        ..Default::default()
                    },
                })
                .await
                .exes
                .get("flutter")
                .unwrap()
                .exe_path,
            Some("bin/flutter".into())
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn download_from_beta_channel() {
        let channel = "beta";
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter_test", |config| {
                config.host(HostOS::Windows, HostArch::Arm64);
                config.tool_config(FlutterPluginConfig {
                    channel: channel.into(),
                });
            })
            .await;

        assert_eq!(
            plugin.download_prebuilt(DownloadPrebuiltInput {
                context: ToolContext {
                    version: VersionSpec::parse("3.22.0-0.3.pre").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            }).await,
            DownloadPrebuiltOutput {
                archive_prefix: Some("flutter".into()),
                download_url: "https://storage.googleapis.com/flutter_infra_release/releases/beta/windows/flutter_windows_3.22.0-0.3.pre-beta.zip".into(),
                ..Default::default()
            }
        )
    }
}
