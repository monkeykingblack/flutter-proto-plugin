use proto_pdk_test_utils::*;

mod flutter_tool {
    use super::*;

    #[cfg(not(windows))]
    generate_shims_test!("flutter-test", ["flutter", "dart"]);
}
