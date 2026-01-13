
#[cfg(test)]
mod view_service_tests {
    use crate::common::builders::{EndpointBuilder, EnvironmentBuilder, HtrsConfigBuilder, ServiceBuilder};
    use crate::common::test_helpers::{clear_config, setup};
    use assert_cmd::Command;
    use predicates::boolean::PredicateBooleanExt;
    use predicates::str::contains;
    use std::error::Error;

    #[test]
    fn given_view_service_command_with_unknown_service_then_should_error() -> Result<(), Box<dyn Error>> {
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("view")
            .arg("service")
            .arg("unknown_service")
            .assert()
            .failure()
            .stdout("No service could be found with name or alias `unknown_service`\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_view_service_command_with_known_service_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_name")
                    .with_alias("foo_alias")
                    .with_environment(
                        EnvironmentBuilder::new()
                            .with_name("foo_environment")
                            .with_alias("foo_env_alias")
                            .with_host("foo.com")
                    )
                    .with_endpoint(
                        EndpointBuilder::new()
                            .with_name("foo_endpoint")
                            .with_path("/my/{path_param}/path")
                            .with_query_param("required_param", true)
                            .with_query_param("optional_param", false)
                    )
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("view")
            .arg("service")
            .arg("foo_name")
            .assert()
            .success()
            .stdout(
                contains("Name: foo_name")
                    .and(contains("Alias: foo_alias"))
                    .and(contains(" - foo_environment (foo_env_alias) ~ foo.com"))
                    .and(contains("foo_endpoint ~ /my/{path_param}/path"))
                    .and(contains(" - *required_param"))
                    .and(contains(" - optional_param"))
            );

        clear_config(&path);
        Ok(())
    }
}
