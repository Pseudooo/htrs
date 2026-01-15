
#[cfg(test)]
mod delete_endpoint_tests {
    use crate::common::builders::{EndpointBuilder, HtrsConfigBuilder, ServiceBuilder};
    use crate::common::test_helpers::{clear_config, get_config, setup};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_delete_endpoint_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("delete")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No service could be found with name or alias `foo_service`\n");

        clear_config(&path);
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
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("delete")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No endpoint could be found with name `foo_endpoint`\n");

        clear_config(&path);
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
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("delete")
            .arg("endpoint")
            .arg("foo_endpoint")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config(&path);
        let service = &config.services[0];
        assert_eq!(service.endpoints.len(), 0);

        clear_config(&path);
        Ok(())
    }
}
