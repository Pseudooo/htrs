#[cfg(test)]
mod delete_environment_tests {
    use crate::common::builders::{EnvironmentBuilder, HtrsConfigBuilder, ServiceBuilder};
    use crate::common::test_helpers::{clear_config, get_config, setup};
    use assert_cmd::Command;
    use rstest::rstest;
    use std::error::Error;

    #[test]
    fn given_delete_environment_command_without_args_then_should_fail() -> Result<(), Box<dyn Error>> {
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("delete")
            .arg("environment")
            .assert()
            .failure();

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_delete_environment_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("delete")
            .arg("environment")
            .arg("foo_environment")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No service could be found with name or alias `foo_service`\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_delete_environment_command_with_unknown_environment_then_should_fail() -> Result<(), Box<dyn Error>> {
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
            .arg("environment")
            .arg("foo_environment")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No environment could be found with name or alias `foo_environment`\n");

        clear_config(&path);
        Ok(())
    }

    #[rstest]
    #[case("environment")]
    #[case("env")]
    fn given_delete_environment_command_with_known_environment_name_then_should_succeed(
        #[case] env_cmd: &str
    ) -> Result<(), Box<dyn Error>> {
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
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("delete")
            .arg(env_cmd)
            .arg("foo_environment")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config(&path);
        let service = &config.services[0];
        assert_eq!(service.environments.len(), 0);

        clear_config(&path);
        Ok(())
    }

    #[rstest]
    #[case("environment")]
    #[case("env")]
    fn given_delete_environment_command_with_known_environment_alias_then_should_succeed(
        #[case] env_cmd: &str
    ) -> Result<(), Box<dyn Error>> {
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
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("delete")
            .arg(env_cmd)
            .arg("foo_alias")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config(&path);
        let service = &config.services[0];
        assert_eq!(service.environments.len(), 0);

        clear_config(&path);
        Ok(())
    }
}