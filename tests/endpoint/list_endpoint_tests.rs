
#[cfg(test)]
mod list_endpoint_tests {
    use crate::common::builders::{EndpointBuilder, HtrsConfigBuilder, ServiceBuilder};
    use crate::common::test_helpers::{clear_config, setup};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_list_endpoint_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>>{
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("list")
            .arg("endpoint")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No service could be found with name or alias `foo_service`\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_list_endpoint_command_with_known_service_and_no_endpoints_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("list")
            .arg("endpoint")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success()
            .stdout("No endpoints defined\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_list_endpoint_command_with_known_service_and_single_endpoint_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("endpoint1")
                            .with_path("/path")
                    )
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("endpoint2")
                            .with_path("/path")
                    )
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("list")
            .arg("endpoint")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success()
            .stdout(" - endpoint1\n - endpoint2\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_list_endpoint_command_with_known_service_and_filter_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("foo_endpoint1")
                            .with_path("/path")
                    )
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("endpoint2")
                            .with_path("/path")
                    )
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("list")
            .arg("endpoint")
            .arg("--service")
            .arg("foo_service")
            .arg("--filter")
            .arg("foo")
            .assert()
            .success()
            .stdout(" - foo_endpoint1\n");

        clear_config(&path);
        Ok(())
    }
}
