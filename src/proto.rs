use std::collections::HashMap;

use crate::config::FlutterPluginConfig;

use extism_pdk::*;
use proto_pdk::*;
use schematic::SchemaBuilder;
use yaml_rust2::YamlLoader;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

static NAME: &str = "Flutter";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        config_schema: Some(SchemaBuilder::build_root::<FlutterPluginConfig>()),
        minimum_proto_version: Some(Version::new(0, 42, 0)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        self_upgrade_commands: vec!["upgrade".into()],
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/flutter/flutter")?
        .into_iter()
        .collect::<Vec<_>>();
    Ok(Json(LoadVersionsOutput::from(tags)?))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec!["pubspec.yaml".into()],
        ignore: vec!["build".into(), "android".into(), "ios".into(), "web".into()],
    }))
}

#[plugin_fn]
pub fn parse_version_file(
    Json(input): Json<ParseVersionFileInput>,
) -> FnResult<Json<ParseVersionFileOutput>> {
    let mut output = ParseVersionFileOutput::default();

    if input.file == "pubspec.yaml" {
        if let Ok(pubspecs) = YamlLoader::load_from_str(&input.content) {
            let pubspec = &pubspecs[0];
            let sdks = &pubspec["environment"];
            if let Some(flutter) = sdks["flutter"].as_str() {
                output.version = Some(UnresolvedVersionSpec::parse(flutter)?)
            }
        }
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64, HostArch::Arm64],
        ],
    )?;

    let version = &input.context.version;

    if version.is_canary() {
        return Err(plugin_err!(PluginError::UnsupportedCanary {
            tool: NAME.into()
        }));
    }

    let channel = if version.to_string().contains("-pre") {
        "beta"
    } else {
        "stable"
    };

    let os = match env.os {
        HostOS::Linux => "linux",
        HostOS::Windows => "windows",
        HostOS::MacOS => "macos",
        _ => unreachable!(),
    };

    let prefix = if env.os.is_mac() && env.arch == HostArch::Arm64 {
        format!("flutter_macos_arm64_{version}-{channel}")
    } else {
        format!("flutter_{os}_{version}-{channel}")
    };

    let filename = if env.os.is_linux() {
        format!("{prefix}.tar.xz")
    } else {
        format!("{prefix}.zip")
    };

    let host = get_tool_config::<FlutterPluginConfig>()?.dist_url;

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some("flutter".into()),
        download_url: host
            .replace("{channel}", &channel)
            .replace("{os}", &os)
            .replace("{file}", &filename),
        download_name: Some(filename),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([
            (
                "flutter".into(),
                ExecutableConfig::new_primary(env.os.get_file_name("bin/flutter", "bat")),
            ),
            (
                "dart".into(),
                ExecutableConfig::new(env.os.get_file_name("bin/dart", "bat")),
            ),
        ]),
        globals_lookup_dirs: vec!["$HOME/.pub-cache/bin".into()],
        ..LocateExecutablesOutput::default()
    }))
}
