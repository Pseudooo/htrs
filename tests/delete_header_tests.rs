
mod common;

#[cfg(test)]
mod delete_header_tests {
    use crate::common::{get_config, setup, EnvironmentBuilder, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_delete_header_command_with_no_args_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("header")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_delete_global_header_command_with_unknown_header_then_should_succeed() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("header")
            .arg("foo_header")
            .assert()
            .success();
        Ok(())
    }

    #[test]
    fn given_delete_global_header_command_with_known_header_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_header("foo_header", "foo_value")
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("header")
            .arg("foo_header")
            .assert()
            .success();

        let config = get_config();
        assert_eq!(config.headers.len(), 0);
        Ok(())
    }

    #[test]
    fn given_delete_service_header_command_with_known_service_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_header("foo_header", "foo_value")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("header")
            .arg("foo_header")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.headers.len(), 0);
        Ok(())
    }

    #[test]
    fn given_delete_service_header_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("header")
            .arg("foo_header")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("Unable to find service with name or alias `foo_service`\n");
        Ok(())
    }

    #[test]
    fn given_delete_environment_header_with_known_environment_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_host("google.com")
                            .with_header("foo_header", "foo_value")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("header")
            .arg("foo_header")
            .arg("--service")
            .arg("foo_service")
            .arg("--environment")
            .arg("foo_environment")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        let environment = &service.environments[0];
        assert_eq!(environment.headers.len(), 0);
        Ok(())
    }

    #[test]
    fn given_delete_environment_header_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("header")
            .arg("foo_header")
            .arg("--service")
            .arg("foo_service")
            .arg("--environment")
            .arg("foo_environment")
            .assert()
            .failure()
            .stdout("Unable to find service with name or alias `foo_service`\n");
        Ok(())
    }

    #[test]
    fn given_delete_environment_header_command_with_unknown_environment_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("header")
            .arg("foo_header")
            .arg("--service")
            .arg("foo_service")
            .arg("--environment")
            .arg("foo_environment")
            .assert()
            .failure()
            .stdout("Unable to find environment with name or alias `foo_environment` for service `foo_service`\n");
        Ok(())
    }

    #[test]
    fn given_delete_header_command_with_invalid_arguments_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("header")
            .arg("foo_header")
            .arg("--environment")
            .arg("foo_environment")
            .assert()
            .failure()
            .stdout("Invalid combination of arguments used\n");
        Ok(())
    }
}
