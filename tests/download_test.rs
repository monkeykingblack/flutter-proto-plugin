use proto_pdk_test_utils::*;

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
                download_name: Some("flutter_linux_3.24.5-stable.tar.xz".into()),
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
                download_name: Some("flutter_macos_3.24.5-stable.zip".into()),
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
                download_name: Some("flutter_macos_arm64_3.24.5-stable.zip".into()),
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
                download_name: Some("flutter_macos_3.24.5-stable.zip".into()),
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
                .get("dart")
                .unwrap()
                .exe_path,
            Some("bin/dart".into())
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_windows_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("flutter-test", |config| {
                config.host(HostOS::Windows, HostArch::Arm64);
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
            Some("bin/flutter.exe".into())
        );

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
                .get("dart")
                .unwrap()
                .exe_path,
            Some("bin/dart.exe".into())
        );
    }
}
