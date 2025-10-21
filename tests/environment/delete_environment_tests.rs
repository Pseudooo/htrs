#[cfg(test)]
mod delete_environment_tests {
    use crate::common::test_helpers::{get_config, setup, EnvironmentBuilder, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use rstest::rstest;
    use std::error::Error;

    #[test]
    fn given_delete_environment_command_without_args_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("environment")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_delete_environment_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("delete")
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
    fn given_delete_environment_command_with_unknown_environment_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("environment")
            .arg("foo_environment")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No environment could be found with name or alias `foo_environment`\n");
        Ok(())
    }

    #[rstest]
    #[case("environment")]
    #[case("env")]
    fn given_delete_environment_command_with_known_environment_then_should_succeed(
        #[case] env_cmd: &str
    ) -> Result<(), Box<dyn Error>> {
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
            .arg("delete")
            .arg(env_cmd)
            .arg("foo_environment")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success();

        let config = get_config();
        let service = &config.services[0];
        assert_eq!(service.environments.len(), 0);
        Ok(())
    }
}