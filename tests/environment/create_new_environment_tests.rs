#[cfg(test)]
mod create_new_environment_tests {
    use crate::common::test_helpers::{clear_config, get_config2, setup2, EnvironmentBuilder, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_create_environment_command_without_args_then_should_fail() -> Result<(), Box<dyn Error>> {
        let path = setup2(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("environment")
            .assert()
            .failure();

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_create_environment_command_without_service_arg_then_should_fail() -> Result<(), Box<dyn Error>> {
        let path = setup2(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("environment")
            .arg("foo_environment")
            .arg("google.com")
            .assert()
            .failure();

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_create_environment_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        let path = setup2(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("environment")
            .arg("foo_environment")
            .arg("google.com")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No service found with name or alias `foo_service`\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_create_environment_command_with_known_service_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        let path = setup2(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("environment")
            .arg("foo_environment")
            .arg("google.com")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config2(&path);
        let service = &config.services[0];
        assert_eq!(service.environments.len(), 1);
        let environment = &service.environments[0];
        assert_eq!(environment.name, "foo_environment");
        assert_eq!(environment.alias, None);
        assert_eq!(environment.host, "google.com");
        assert_eq!(environment.default, false);

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_create_environment_command_with_alias_and_known_service_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        let path = setup2(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("environment")
            .arg("foo_environment")
            .arg("google.com")
            .arg("--alias")
            .arg("foo_alias")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config2(&path);
        let service = &config.services[0];
        assert_eq!(service.environments.len(), 1);
        let environment = &service.environments[0];
        assert_eq!(environment.name, "foo_environment");
        assert_eq!(environment.alias, Some("foo_alias".to_string()));
        assert_eq!(environment.host, "google.com");
        assert_eq!(environment.default, false);

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_create_environment_command_with_default_flag_and_known_service_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        let path = setup2(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("environment")
            .arg("foo_environment")
            .arg("google.com")
            .arg("--service")
            .arg("foo_service")
            .arg("--default")
            .assert()
            .success();

        let config = get_config2(&path);
        let service = &config.services[0];
        assert_eq!(service.environments.len(), 1);
        let environment = &service.environments[0];
        assert_eq!(environment.name, "foo_environment");
        assert_eq!(environment.alias, None);
        assert_eq!(environment.host, "google.com");
        assert_eq!(environment.default, true);

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_create_environment_command_with_existing_name_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_alias("foo_alias")
                            .with_host("google.com")
                    )
            )
            .build();
        let path = setup2(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("environment")
            .arg("foo_environment")
            .arg("google.com")
            .arg("--alias")
            .arg("new_alias")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("Service `foo_service` already has an environment with name or alias `foo_environment`\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_create_environment_command_with_existing_alias_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_alias("foo_alias")
                            .with_host("google.com")
                    )
            )
            .build();
        let path = setup2(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("environment")
            .arg("new_environment")
            .arg("google.com")
            .arg("--alias")
            .arg("foo_alias")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("Service `foo_service` already has an environment with name or alias `foo_alias`\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_create_environment_command_with_existing_default_then_should_succeed_and_replace_default() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_alias("foo_alias")
                            .with_host("google.com")
                            .with_default()
                    )
            )
            .build();
        let path = setup2(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("environment")
            .arg("new_environment")
            .arg("google.com")
            .arg("--default")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config2(&path);
        let service = &config.services[0];
        assert_eq!(service.environments.len(), 2);
        let existing_environment = &service.environments[0];
        assert_eq!(existing_environment.default, false);
        let new_environment = &service.environments[1];
        assert_eq!(new_environment.name, "new_environment");
        assert_eq!(new_environment.alias, None);
        assert_eq!(new_environment.host, "google.com");
        assert_eq!(new_environment.default, true);

        clear_config(&path);
        Ok(())
    }
}
