mod call_command_tests {
    use crate::common::builders::{EndpointBuilder, EnvironmentBuilder, HtrsConfigBuilder, ServiceBuilder};
    use crate::common::test_helpers::{clear_config, setup};
    use assert_cmd::Command;
    use httptest::matchers::{contains, request, url_decoded};
    use httptest::responders::status_code;
    use httptest::{all_of, Expectation, ServerPool};
    use std::error::Error;

    static SERVER_POOL: ServerPool = ServerPool::new(1);

    #[test]
    fn given_known_endpoint_with_no_params_when_call_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let mut server = SERVER_POOL.get_server();
        server.expect(
            Expectation::matching(all_of![
                request::method("GET"),
                request::path("/my/path"),
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
                    )
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("call")
            .arg("foo_service")
            .arg("foo_endpoint")
            .assert()
            .success();

        clear_config(&path);
        server.verify_and_clear();
        Ok(())
    }

    #[test]
    fn given_known_endpoint_with_path_param_when_call_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let mut server = SERVER_POOL.get_server();
        server.expect(
            Expectation::matching(all_of![
                request::method("GET"),
                request::path("/my/foo/path"),
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
                            .with_path("/my/{param}/path")
                    )
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("call")
            .arg("foo_service")
            .arg("foo_endpoint")
            .arg("--param")
            .arg("foo")
            .assert()
            .success();

        clear_config(&path);
        server.verify_and_clear();
        Ok(())
    }

    #[test]
    fn given_known_endpoint_with_required_param_when_call_then_should_fail() -> Result<(), Box<dyn Error>> {
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
                            .with_query_param("foo", true)
                    )
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("call")
            .arg("foo_service")
            .arg("foo_endpoint")
            .assert()
            .failure();

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_known_endpoint_with_query_param_when_call_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let mut server = SERVER_POOL.get_server();
        server.expect(
            Expectation::matching(all_of![
                request::path("/my/path"),
                request::query(url_decoded(contains(("foo", "bar"))))
            ]).respond_with(status_code(200))
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
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("call")
            .arg("foo_service")
            .arg("foo_endpoint")
            .arg("--foo")
            .arg("bar")
            .assert()
            .success();

        clear_config(&path);
        server.verify_and_clear();
        Ok(())
    }
}