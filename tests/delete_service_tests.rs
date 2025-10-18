mod common;

mod delete_service_tests {
    use crate::common::{get_config, setup, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_delete_service_command_without_name_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("service")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_delete_service_command_with_unknown_service_then_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("service")
            .arg("foo")
            .assert()
            .failure();
        Ok(())
    }

    #[test]
    fn given_delete_service_command_with_known_service_name_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("service_name")
                    .with_alias("service_alias")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("service")
            .arg("service_name")
            .assert()
            .success();

        let config = get_config();
        assert_eq!(config.services.len(), 0);
        Ok(())
    }

    #[test]
    fn given_delete_service_command_with_known_service_alias_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("service_name")
                    .with_alias("service_alias")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("delete")
            .arg("service")
            .arg("service_alias")
            .assert()
            .success();

        let config = get_config();
        assert_eq!(config.services.len(), 0);
        Ok(())
    }
}
