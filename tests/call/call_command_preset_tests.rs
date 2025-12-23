mod call_command_preset_tests {
    use crate::common::test_helpers::{clear_config, setup, EndpointBuilder, EnvironmentBuilder, HtrsConfigBuilder, PresetBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use httptest::matchers::{contains, request, url_decoded};
    use httptest::responders::status_code;
    use httptest::{all_of, Expectation, ServerPool};
    use std::error::Error;

    static SERVER_POOL: ServerPool = ServerPool::new(1);

    #[test]
    fn given_unknown_preset_when_call_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_host("foo.com")
                            .with_default()
                    )
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("foo_endpoint")
                            .with_path("/my/path")
                    )
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("call")
            .arg("foo_service")
            .arg("foo_endpoint")
            .arg("--preset")
            .arg("foo_preset")
            .assert()
            .failure()
            .stdout("No preset found with name `foo_preset`\n");
        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_known_endpoint_with_query_param_when_call_with_preset_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let server = SERVER_POOL.get_server();
        server.expect(
            Expectation::matching(all_of![
                request::path("/my/path"),
                request::query(url_decoded(contains(("foo", "bar")))),
            ]).respond_with(status_code(200)),
        );
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_host(server.addr().to_string().as_str())
                            .with_default()
                    )
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("foo_endpoint")
                            .with_path("/my/path")
                            .with_query_param("foo", true)
                    )
            )
            .with_preset(
                PresetBuilder::new()
                    .with_name("foo_preset")
                    .with_value("foo", "bar")
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("call")
            .arg("foo_service")
            .arg("foo_endpoint")
            .arg("--preset")
            .arg("foo_preset")
            .assert()
            .success();
        Ok(())
    }
}