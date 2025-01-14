# Flutter plugin

[Flutter](https://flutter.dev/) WASM plugin for [proto](https://github.com/moonrepo/proto).

> Note: Support flutter version larger than `3.0.0`

## Installation

Add the following to `.prototools`.

```toml
[plugins]
flutter = "github://monkeykingblack/flutter-proto-plugin"
```

Flutter proto flugin does not export global package bin to your `PATH`. You can do by yourself or run command mannualy with `dart pub global activate`. For more information please visit [Dart docs](https://dart.dev/tools/pub/cmd/pub-global#running-a-script-from-your-path)

## Configuration

Flutter plugin can be configured with a `.prototools` file.

- `channel` (string) - The channel to download Flutter archives from. Support `stable`, `beta`, `dev` value

```toml
[tools.flutter]
channel = "stable"
```

> NOTE: Starting from version `v0.42` of Proto, **the results of HTTP requests are cached for 12 hours** ([docs](https://moonrepo.dev/blog/proto-v0.42#other-changes)). Therefore, to get versions for Flutter from different channels, you need to disable the caching feature of Proto.
> 
> ```shell
> PROTO_CACHE=off proto versions flutter
> ```

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
