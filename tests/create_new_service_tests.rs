mod common;

#[cfg(test)]
mod create_new_service_tests {
    use crate::common::{get_config, setup, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
    use std::error::Error;
    use std::process::Command;

    #[test]
    fn given_create_new_service_when_no_arguments_then_should_err() -> Result<(), Box<dyn Error>> {
        setup(None);

        let mut cmd = Command::cargo_bin("htrs")?;

        cmd.arg("new")
            .arg("service");
        cmd.assert()
            .failure();

        Ok(())
    }

    #[test]
    pub fn given_valid_create_new_service_command_without_alias_then_should_succeed() -> Result<(), Box<dyn Error>> {
        setup(None);

        let mut cmd = Command::cargo_bin("htrs")?;

        cmd.arg("new")
            .arg("service")
            .arg("foo");
        cmd.assert()
            .success();

        let config = get_config();
        let service = config.get_service("foo").unwrap();
        assert_eq!(service.name, "foo");
        assert_eq!(service.alias, None);

        Ok(())
    }

    #[test]
    fn given_valid_create_new_service_command_then_should_succeed() -> Result<(), Box<dyn Error>> {
        setup(None);

        let mut cmd = Command::cargo_bin("htrs")?;

        cmd.arg("new")
            .arg("service")
            .arg("foo")
            .arg("--alias").arg("bar");
        cmd.assert()
            .success();

        let config = get_config();
        let service = config.get_service("foo").unwrap();
        assert_eq!(service.name, "foo");
        assert_eq!(service.alias, Some("bar".to_string()));

        Ok(())
    }

    #[test]
    fn given_valid_create_service_command_when_existing_service_name_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("existing_service_name")
                    .with_alias("existing_service_alias")
            )
            .build();
        setup(Some(config));

        let mut cmd = Command::cargo_bin("htrs")?;
        cmd.arg("new")
            .arg("service")
            .arg("existing_service_name");
        cmd.assert()
            .success();

        Ok(())
    }
}
