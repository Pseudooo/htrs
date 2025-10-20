
mod common;

#[cfg(test)]
mod set_header_tests {
    use crate::common::test_helpers::{get_config, setup, EnvironmentBuilder, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_set_header_command_with_no_args_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("set")
            .arg("header")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_set_global_header_command_then_should_succeed() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("set")
            .arg("header")
            .arg("header_name")
            .arg("header_value")
            .assert()
            .success();

        let config = get_config();
        assert_eq!(config.headers.len(), 1);
        assert!(config.headers.contains_key("header_name"));
        assert_eq!(config.headers["header_name"], "header_value");
        Ok(())
    }

    #[test]
    fn given_set_global_header_command_with_existing_header_then_should_succeed_and_overwrite() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_header("header_name", "header_value")
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("set")
            .arg("header")
            .arg("header_name")
            .arg("new_header_value")
            .assert()
            .success();

        let config = get_config();
        assert_eq!(config.headers.len(), 1);
        assert!(config.headers.contains_key("header_name"));
        assert_eq!(config.headers["header_name"], "new_header_value");
        Ok(())
    }

    #[test]
    fn given_set_service_header_command_with_known_service_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("set")
            .arg("header")
            .arg("header_name")
            .arg("header_value")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.headers.len(), 1);
        assert!(service.headers.contains_key("header_name"));
        assert_eq!(service.headers["header_name"], "header_value");
        Ok(())
    }

    #[test]
    fn given_set_service_header_with_known_service_and_existing_header_then_should_succeed_and_overwrite() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_header("header_name", "old_header_value")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("set")
            .arg("header")
            .arg("header_name")
            .arg("new_header_value")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.headers.len(), 1);
        assert!(service.headers.contains_key("header_name"));
        assert_eq!(service.headers["header_name"], "new_header_value");
        Ok(())
    }

    #[test]
    fn given_set_service_header_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("set")
            .arg("header")
            .arg("header_name")
            .arg("new_header_value")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("Unable to find service with name or alias `foo_service`\n");
        Ok(())
    }

    #[test]
    fn given_set_environment_header_with_known_environment_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_host("google.com")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("set")
            .arg("header")
            .arg("header_name")
            .arg("header_value")
            .arg("--service")
            .arg("foo_service")
            .arg("--environment")
            .arg("foo_environment")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        let environment = &service.environments[0];
        assert_eq!(environment.headers.len(), 1);
        assert!(environment.headers.contains_key("header_name"));
        assert_eq!(environment.headers["header_name"], "header_value");
        Ok(())
    }

    #[test]
    fn given_set_environment_header_with_existing_header_then_should_succeed_and_overwrite() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_host("google.com")
                            .with_header("header_name", "old_header_value")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("set")
            .arg("header")
            .arg("header_name")
            .arg("new_header_value")
            .arg("--service")
            .arg("foo_service")
            .arg("--environment")
            .arg("foo_environment")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        let environment = &service.environments[0];
        assert_eq!(environment.headers.len(), 1);
        assert!(environment.headers.contains_key("header_name"));
        assert_eq!(environment.headers["header_name"], "new_header_value");
        Ok(())
    }

    #[test]
    fn given_set_environment_header_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("set")
            .arg("header")
            .arg("header_name")
            .arg("header_value")
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
    fn given_set_environment_header_with_unknown_environment_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("set")
            .arg("header")
            .arg("header_name")
            .arg("header_value")
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
    fn given_set_environment_header_with_no_service_arg_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("set")
            .arg("header")
            .arg("header_name")
            .arg("header_value")
            .arg("--environment")
            .arg("foo_environment")
            .assert()
            .failure()
            .stdout("Invalid combination of arguments used\n");
        Ok(())
    }
}
