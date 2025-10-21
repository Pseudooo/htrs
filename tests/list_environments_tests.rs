
mod common;

#[cfg(test)]
mod list_environments_tests {
    use crate::common::test_helpers::{setup, EnvironmentBuilder, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use rstest::rstest;
    use std::error::Error;

    #[test]
    fn given_list_environments_command_without_arguments_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("list")
            .arg("environment")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_list_environments_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("list")
            .arg("environment")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .failure()
            .stdout("No service could be found with name or alias `foo_service`\n");
        Ok(())
    }

    #[test]
    fn given_list_environments_command_with_known_service_and_no_environments_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("list")
            .arg("environment")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success()
            .stdout("No environments defined\n");
        Ok(())
    }

    #[rstest]
    #[case("list")]
    #[case("ls")]
    fn given_list_environments_command_with_known_environments_then_should_succeed(
        #[case] list_cmd: &str,
    ) -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("environment1")
                            .with_alias("alias1")
                            .with_host("google.com")
                    )
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("environment2")
                            .with_host("google.com")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg(list_cmd)
            .arg("environment")
            .arg("--service")
            .arg("foo_service")
            .assert()
            .success()
            .stdout(" - environment1 (alias1)\n - environment2\n");
        Ok(())
    }

    #[test]
    fn given_list_environments_command_with_filter_and_no_matching_environments_then_should_succeed() -> Result<(), Box<dyn Error>> {
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
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("list")
            .arg("environment")
            .arg("--service")
            .arg("foo_service")
            .arg("--filter")
            .arg("unknown")
            .assert()
            .success()
            .stdout("No environments found for service `foo_service` with name or alias containing `unknown`\n");
        Ok(())
    }

    #[test]
    fn given_list_environments_command_with_filter_and_matching_environments_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("environment1")
                            .with_alias("alias1")
                            .with_host("google.com")
                    )
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment2")
                            .with_host("google.com")
                    )
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("environment3")
                            .with_alias("foo_alias3")
                            .with_host("google.com")
                    )
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("list")
            .arg("environment")
            .arg("--service")
            .arg("foo_service")
            .arg("--filter")
            .arg("foo")
            .assert()
            .success()
            .stdout(" - foo_environment2\n - environment3 (foo_alias3)\n");
        Ok(())
    }
}
