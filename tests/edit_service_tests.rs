mod common;

mod edit_service_tests {
    use crate::common::{get_config, setup, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_edit_service_command_without_name_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("service")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_edit_service_command_with_unknown_service_then_should_succeed() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("service")
            .arg("unknown-service")
            .assert()
            .failure();

        Ok(())
    }

    #[test]
    fn given_edit_command_with_existing_service_when_edit_name_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("existing_name")
                    .with_alias("existing_alias")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("service")
            .arg("existing_name")
            .arg("--new-name")
            .arg("new_name")
            .assert()
            .success();

        let config = get_config();
        assert_eq!(config.services.len(), 1);
        let service = &config.services[0];
        assert_eq!(service.name, "new_name");
        assert_eq!(service.alias, Some("existing_alias".to_string()));
        Ok(())
    }

    #[test]
    fn given_edit_command_with_existing_service_when_edit_alias_should_succeed() -> Result<(), Box<dyn Error>>{
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("existing_name")
                    .with_alias("existing_alias")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("edit")
            .arg("service")
            .arg("existing_name")
            .arg("--new-alias")
            .arg("new_alias")
            .assert()
            .success();

        let config = get_config();
        assert_eq!(config.services.len(), 1);
        let service = &config.services[0];
        assert_eq!(service.name, "existing_name");
        assert_eq!(service.alias, Some("new_alias".to_string()));
        Ok(())
    }
}
