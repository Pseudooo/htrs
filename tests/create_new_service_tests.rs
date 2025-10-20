mod common;

#[cfg(test)]
mod create_new_service_tests {
    use crate::common::test_helpers::{get_config, setup, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
    use rstest::rstest;
    use std::error::Error;
    use std::process::Command;

    #[test]
    fn given_create_service_command_without_name_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("service")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_create_service_command_with_name_then_should_succeed() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("service")
            .arg("service_name")
            .assert()
            .success();

        let config = get_config();
        assert_eq!(config.services.len(), 1);
        let service = &config.services[0];
        assert_eq!(service.name, "service_name");
        assert_eq!(service.alias, None);
        Ok(())
    }

    #[test]
    fn given_create_service_command_with_name_and_alias_then_should_succeed() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("service")
            .arg("service_name")
            .arg("--alias")
            .arg("service_alias")
            .assert()
            .success();

        let config = get_config();
        assert_eq!(config.services.len(), 1);
        let service = &config.services[0];
        assert_eq!(service.name, "service_name");
        assert_eq!(service.alias, Some("service_alias".to_string()));
        Ok(())
    }

    #[rstest]
    #[case("existing_service", "new_alias")]
    #[case("new_service", "existing_alias")]
    fn given_create_service_command_when_existing_service_with_name_or_alias_then_should_fail(
        #[case] service_name: String,
        #[case] service_alias: String,
    ) -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("existing_service")
                    .with_alias("existing_alias")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("service")
            .arg(service_name)
            .arg("--alias")
            .arg(service_alias)
            .assert()
            .failure();

        let config = get_config();
        assert_eq!(config.services.len(), 1);
        assert_eq!(config.services[0].name, "existing_service");
        assert_eq!(config.services[0].alias, Some("existing_alias".to_string()));
        Ok(())
    }
}
