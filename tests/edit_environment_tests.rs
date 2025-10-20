mod common;

#[cfg(test)]
mod edit_environment_tests {
    use crate::common::test_helpers::{get_config, setup, EnvironmentBuilder, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_edit_environment_command_without_name_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("environment")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_edit_environment_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("environment")
            .arg("foo_environment")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No service could be found with name or alias `foo_service`\n");
        Ok(())
    }

    #[test]
    fn given_edit_environment_command_with_unknown_environment_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("environment")
            .arg("foo_environment")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No environment could be found with name or alias `foo_environment`\n");
        Ok(())
    }

    #[test]
    fn given_edit_environment_command_with_known_environment_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("original_name")
                            .with_host("google.com")
                            .with_alias("original_alias")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("environment")
            .arg("original_name")
            .arg("--service")
            .arg("foo_service")
            .arg("--new-name")
            .arg("new_name")
            .arg("--new-alias")
            .arg("new_alias")
            .arg("--new-host")
            .arg("newhost.com")
            .arg("--is-default")
            .arg("true")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        let environment = &service.environments[0];
        assert_eq!(environment.name, "new_name");
        assert_eq!(environment.alias, Some("new_alias".to_string()));
        assert_eq!(environment.host, "newhost.com");
        assert_eq!(environment.default, true);
        Ok(())
    }

    #[test]
    fn given_edit_environment_command_with_known_environment_and_existing_name_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("existing_environment")
                            .with_alias("existing_alias")
                            .with_host("host.com")
                    )
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_alias("foo_alias")
                            .with_host("host.com")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("environment")
            .arg("foo_environment")
            .arg("--service")
            .arg("foo_service")
            .arg("--new-name")
            .arg("existing_environment")
            .assert()
            .failure()
            .stdout("Service `foo_service` already has an environment with name or alias `existing_environment`\n");
        Ok(())
    }

    #[test]
    fn given_edit_environment_command_with_known_environment_and_existing_alias_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("existing_environment")
                            .with_alias("existing_alias")
                            .with_host("host.com")
                    )
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_alias("foo_alias")
                            .with_host("host.com")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("environment")
            .arg("foo_environment")
            .arg("--service")
            .arg("foo_service")
            .arg("--new-alias")
            .arg("existing_alias")
            .assert()
            .failure()
            .stdout("Service `foo_service` already has an environment with name or alias `existing_alias`\n");
        Ok(())
    }

    #[test]
    fn given_edit_environment_command_with_known_environment_and_existing_default_when_set_default_then_should_replace_existing() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("existing_default")
                            .with_host("host.com")
                            .with_default()
                    )
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_host("host.com")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("environment")
            .arg("foo_environment")
            .arg("--service")
            .arg("foo_service")
            .arg("--is-default")
            .arg("true")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        let existing_default_environment = &service.environments[0];
        assert_eq!(existing_default_environment.name, "existing_default");
        assert_eq!(existing_default_environment.default, false);
        let new_default_environment = &service.environments[1];
        assert_eq!(new_default_environment.name, "foo_environment");
        assert_eq!(new_default_environment.default, true);
        Ok(())
    }

    #[test]
    fn given_edit_environment_command_with_non_boolean_default_value_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("environment")
            .arg("foo_environment")
            .arg("--service")
            .arg("foo_service")
            .arg("--is-default")
            .arg("foo")
            .assert()
            .failure();
        Ok(())
    }
}