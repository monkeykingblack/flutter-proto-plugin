# Flutter plugin

[Flutter](https://flutter.dev/) WASM plugin for [proto](https://github.com/moonrepo/proto).

## Installation

Add the following to `.prototools`.

```toml
[plugins]
flutter = "github://monkeykingblack/flutter-proto-plugin"
```

## Configuration

Flutter plugin does not support configuration.

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