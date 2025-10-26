mod create_new_endpoint_tests {
    use crate::common::test_helpers::{get_config, setup, EndpointBuilder, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use rstest::rstest;
    use std::error::Error;

    #[test]
    fn given_new_endpoint_command_with_known_service_when_execute_then_should_create_endpoint() -> Result<(), Box<dyn Error>> {
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
            .arg("/my/path")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "foo_endpoint");
        assert_eq!(endpoint.path_template, "/my/path");
        assert_eq!(endpoint.query_parameters.len(), 0);
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_single_optional_query_param_when_execute_then_should_create_endpoint_with_param() -> Result<(), Box<dyn Error>> {
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
            .arg("/my/path")
            .arg("--service")
            .arg("foo_service")
            .arg("--query")
            .arg("param")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "foo_endpoint");
        assert_eq!(endpoint.path_template, "/my/path");
        assert_eq!(endpoint.query_parameters.len(), 1);
        let query_param = &endpoint.query_parameters[0];
        assert_eq!(query_param.name, "param");
        assert_eq!(query_param.required, false);
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_multiple_optional_query_params_when_execute_then_should_create_endpoint_with_params() -> Result<(), Box<dyn Error>> {
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
            .arg("/my/path")
            .arg("--service")
            .arg("foo_service")
            .arg("--query")
            .arg("param1")
            .arg("--query")
            .arg("param2")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "foo_endpoint");
        assert_eq!(endpoint.path_template, "/my/path");
        assert_eq!(endpoint.query_parameters.len(), 2);
        let query_param1 = &endpoint.query_parameters[0];
        assert_eq!(query_param1.name, "param1");
        assert_eq!(query_param1.required, false);
        let query_param2 = &endpoint.query_parameters[1];
        assert_eq!(query_param2.name, "param2");
        assert_eq!(query_param2.required, false);
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_single_required_query_param_when_execute_then_should_create_endpoint_with_param() -> Result<(), Box<dyn Error>> {
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
            .arg("/my/path")
            .arg("--service")
            .arg("foo_service")
            .arg("--query")
            .arg("*param")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "foo_endpoint");
        assert_eq!(endpoint.path_template, "/my/path");
        assert_eq!(endpoint.query_parameters.len(), 1);
        let query_param = &endpoint.query_parameters[0];
        assert_eq!(query_param.name, "param");
        assert_eq!(query_param.required, true);
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_multiple_required_query_params_when_execute_then_should_create_endpoint_with_params() -> Result<(), Box<dyn Error>> {
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
            .arg("/my/path")
            .arg("--service")
            .arg("foo_service")
            .arg("--query")
            .arg("*param1")
            .arg("--query")
            .arg("*param2")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "foo_endpoint");
        assert_eq!(endpoint.path_template, "/my/path");
        assert_eq!(endpoint.query_parameters.len(), 2);
        let query_param1 = &endpoint.query_parameters[0];
        assert_eq!(query_param1.name, "param1");
        assert_eq!(query_param1.required, true);
        let query_param2 = &endpoint.query_parameters[1];
        assert_eq!(query_param2.name, "param2");
        assert_eq!(query_param2.required, true);
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_unknown_service_when_execute_then_should_error() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("unknown_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("/my/path")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("Unable to find service with name or alias `foo_service`\n");
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_existing_endpoint_name_when_execute_then_should_error() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("foo_endpoint")
                            .with_path("/my/path")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("/my/path")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("Service `foo_service` already has an endpoint named `foo_endpoint`\n");
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_blank_name_when_execute_then_should_error() -> Result<(), Box<dyn Error>> {
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
            .arg("")
            .arg("/my/path")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("Endpoint name cannot be empty\n");
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_blank_path_when_execute_then_should_error() -> Result<(), Box<dyn Error>> {
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
            .arg("")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("Endpoint path cannot be empty\n");
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_duplicate_path_params_when_execute_then_should_error() -> Result<(), Box<dyn Error>> {
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
            .arg("/{param}/path/{param}")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_blank_query_param_when_execute_then_should_error() -> Result<(), Box<dyn Error>> {
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
            .arg("/my/path")
            .arg("--service")
            .arg("foo_service")
            .arg("--query")
            .arg("")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_new_endpoint_command_with_blank_required_query_param_when_execute_then_should_error() -> Result<(), Box<dyn Error>> {
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
            .arg("/my/path")
            .arg("--service")
            .arg("foo_service")
            .arg("--query")
            .arg("*")
            .assert()
            .failure();
        Ok(())
    }

    #[rstest]
    #[case("paramA", "paramA")]
    #[case("paramA", "*paramA")]
    fn given_new_endpoint_command_with_duplicate_query_params_when_execute_then_should_error(
        #[case] param_a: &str,
        #[case] param_b: &str,
    ) -> Result<(), Box<dyn Error>> {
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
            .arg("/my/path")
            .arg("--service")
            .arg("foo_service")
            .arg("--query")
            .arg(param_a)
            .arg("--query")
            .arg(param_b)
            .assert()
            .failure();
        Ok(())
    }
}
