use proto_pdk_test_utils::*;

mod flutter_tule {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn register_metadata() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox.create_plugin("flutter-test").await;

        let metadata = plugin
            .register_tool(ToolMetadataInput {
                id: "flutter-test".into(),
            })
            .await;

        assert_eq!(metadata.name, "Flutter");
        assert_eq!(
            metadata.default_version,
            Some(UnresolvedVersionSpec::parse("stable").unwrap())
        )
    }
}
