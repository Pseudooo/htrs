
#[cfg(test)]
mod delete_endpoint_tests {
    use crate::common::test_helpers::{get_config, setup, EndpointBuilder, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_delete_endpoint_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("delete")
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
    fn given_delete_endpoint_command_with_unknown_endpoint_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("delete")
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
    fn given_delete_endpoint_command_with_known_endpoint_then_should_succeed() -> Result<(), Box<dyn Error>> {
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
            .arg("delete")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 0);
        Ok(())
    }
}
