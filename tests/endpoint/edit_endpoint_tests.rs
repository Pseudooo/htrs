
#[cfg(test)]
mod edit_endpoint_tests {
    use crate::common::test_helpers::{get_config, setup, EndpointBuilder, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_edit_endpoint_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No service could be found with name or alias `foo_service`\n");
        Ok(())
    }

    #[test]
    fn given_edit_endpoint_command_with_unknown_endpoint_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No endpoint could be found with name `foo_endpoint` for service `foo_service`\n");
        Ok(())
    }

    #[test]
    fn given_edit_endpoint_command_with_known_endpoint_when_edit_name_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("foo_endpoint")
                            .with_path("/path/")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("--service")
            .arg("foo_service")
            .arg("--new-name")
            .arg("new_endpoint")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "new_endpoint");
        assert_eq!(endpoint.path_template, "/path/");
        assert_eq!(endpoint.query_parameters.len(), 0);
        Ok(())
    }

    #[test]
    fn given_edit_endpoint_command_with_known_endpoint_when_edit_path_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("foo_endpoint")
                            .with_path("/old/path")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("--service")
            .arg("foo_service")
            .arg("--new-path")
            .arg("/new/path")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "foo_endpoint");
        assert_eq!(endpoint.path_template, "/new/path");
        assert_eq!(endpoint.query_parameters.len(), 0);
        Ok(())
    }

    #[test]
    fn given_edit_endpoint_command_with_known_endpoint_when_add_query_param_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("foo_endpoint")
                            .with_path("/path")
                            .with_query_param("existing_param")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("--service")
            .arg("foo_service")
            .arg("--new-query")
            .arg("new_param")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "foo_endpoint");
        assert_eq!(endpoint.path_template, "/path");
        assert_eq!(endpoint.query_parameters.len(), 2);
        assert_eq!(endpoint.query_parameters[0], "existing_param");
        assert_eq!(endpoint.query_parameters[1], "new_param");
        Ok(())
    }

    #[test]
    fn given_edit_endpoint_command_with_known_endpoint_when_remove_query_param_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("foo_endpoint")
                            .with_path("/path")
                            .with_query_param("param1")
                            .with_query_param("param2")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("--service")
            .arg("foo_service")
            .arg("--del-query")
            .arg("param1")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 1);
        let endpoint = &service.endpoints[0];
        assert_eq!(endpoint.name, "foo_endpoint");
        assert_eq!(endpoint.path_template, "/path");
        assert_eq!(endpoint.query_parameters.len(), 1);
        assert_eq!(endpoint.query_parameters[0], "param2");
        Ok(())
    }
}
