# Flutter plugin

[Flutter](https://flutter.dev/) WASM plugin for [proto](https://github.com/moonrepo/proto).

> Note: Support flutter version larger than `3.0.0`

## Installation

Add the following to `.prototools`.

```toml
[plugins]
flutter = "github://monkeykingblack/flutter-proto-plugin"
```

## Configuration

Flutter plugin can be configured with a `.prototools` file.

- `channel` (string) - The channel to download Flutter archives from. Support `stable`, `beta`, `dev` value

```toml
[tools.flutter]
channel = "stable"
```


Starting from version `v0.42` of Proto, [the results of HTTP requests are cached for 12 hours](https://moonrepo.dev/blog/proto-v0.42#other-changes). Therefore, to fetch version information for Flutter from different channels, you need to disable the caching feature of Proto.

```shell
PROTO_CACHE=off proto versions flutter
```

## Hooks

Flutter plugin does not support hooks.

## Contributing

Build the plugin:

```shell
cargo build --target wasm32-wasi
```

Test the plugin by running `proto` commands.

```shell
proto install flutter-test
```
