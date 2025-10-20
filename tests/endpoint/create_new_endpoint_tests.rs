mod create_new_endpoint_tests {
    use crate::common::test_helpers::{get_config, setup, EndpointBuilder, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_new_endpoint_command_with_no_args_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("endpoint")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_without_service_arg_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("/path")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("/path")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("Unable to find service with name or alias `foo_service`\n");
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_known_service_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("/path")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "foo_endpoint");
        assert_eq!(endpoint.path_template, "/path");
        assert_eq!(endpoint.query_parameters.len(), 0);
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_known_service_and_existing_endpoint_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("foo_endpoint")
                            .with_path("/path")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("/path")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("Service `foo_service` already has an endpoint named `foo_endpoint`\n");
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_known_service_and_single_query_param_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("/path")
            .arg("--service")
            .arg("foo_service")
            .arg("--query")
            .arg("queryA")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "foo_endpoint");
        assert_eq!(endpoint.path_template, "/path");
        assert_eq!(endpoint.query_parameters.len(), 1);
        assert_eq!(endpoint.query_parameters[0], "queryA");
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_known_service_and_multiple_query_params_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("/path")
            .arg("--service")
            .arg("foo_service")
            .arg("--query")
            .arg("queryA")
            .arg("--query")
            .arg("queryB")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "foo_endpoint");
        assert_eq!(endpoint.path_template, "/path");
        assert_eq!(endpoint.query_parameters.len(), 2);
        assert_eq!(endpoint.query_parameters[0], "queryA");
        assert_eq!(endpoint.query_parameters[1], "queryB");
        Ok(())
    }
}
