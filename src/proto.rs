use std::{collections::HashMap, u32};

use crate::{release_response::ReleaseResponse, FlutterPluginConfig};

use extism_pdk::*;
use proto_pdk::*;
use schematic::SchemaBuilder;
use yaml_rust2::YamlLoader;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
    fn to_virtual_path(path: String) -> String;
}

static NAME: &str = "Flutter";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        default_version: Some(UnresolvedVersionSpec::Alias("stable".into())),
        config_schema: Some(SchemaBuilder::build_root::<FlutterPluginConfig>()),
        minimum_proto_version: Some(Version::new(0, 42, 0)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        self_upgrade_commands: vec!["upgrade".into(), "channel".into()],
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let env = get_host_environment()?;
    let channel = get_tool_config::<FlutterPluginConfig>()?.channel;

    let os = match env.os {
        HostOS::Windows => "windows",
        HostOS::MacOS => "macos",
        HostOS::Linux => "linux",
        _ => unreachable!(),
    };

    let response: ReleaseResponse = fetch_json(format!(
        "https://storage.googleapis.com/flutter_infra_release/releases/releases_{os}.json"
    ))?;

    let arch = match env.arch {
        HostArch::Arm64 => "arm64",
        _ => "x64",
    };

    let releases = response
        .releases
        .into_iter()
        .filter(|r| {
            r.version
                .split('.')
                .next()
                .and_then(|major| major.parse::<u32>().ok())
                .map_or(false, |major| major >= 3)
        })
        .filter(|r| r.channel == channel)
        .filter(|r| match &r.dart_sdk_arch {
            Some(dark_arch) => dark_arch == arch,
            None => true,
        })
        .map(|r| r.version.to_owned())
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(releases)?))
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

    if version.as_version().unwrap().major < 3 {
        return Err(plugin_err!(PluginError::Message(format!(
            "{} plugin supported version from 3.0.0",
            NAME
        ))));
    }

    let os = match env.os {
        HostOS::Linux => "linux",
        HostOS::Windows => "windows",
        HostOS::MacOS => "macos",
        _ => unreachable!(),
    };

    let arch = match env.arch {
        HostArch::Arm64 => "arm64",
        _ => "x64",
    };

    let config = get_tool_config::<FlutterPluginConfig>()?;
    let host = config.dist_url;
    let channel = config.channel;

    let response: ReleaseResponse = fetch_json(format!(
        "https://storage.googleapis.com/flutter_infra_release/releases/releases_{os}.json"
    ))?;

    let release = response
        .releases
        .iter()
        .filter(|r| r.channel == channel)
        .find(|row| {
            let version_match = row.version == version.to_string();
            if !env.os.is_mac() {
                return version_match;
            }
            version_match && row.dart_sdk_arch.as_deref() == Some(arch)
        });

    match release {
        Some(r) => Ok(Json(DownloadPrebuiltOutput {
            archive_prefix: Some(format!("{}", NAME.to_lowercase())),
            download_url: host.replace("{archive}", &r.archive),
            ..Default::default()
        })),
        None => Err(plugin_err!(PluginError::Message(format!(
            "{} plugin not found version on channel {}",
            NAME, channel
        )))),
    }
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
