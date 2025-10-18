
mod common;

#[cfg(test)]
mod list_services_tests {
    use crate::common::{setup, HtrsConfigBuilder, ServiceBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_list_services_command_without_services_then_should_succeed() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("list")
            .arg("service")
            .assert()
            .success()
            .stdout("No services found\n");
        Ok(())
    }

    #[test]
    fn given_list_services_command_with_filter_and_no_services_then_should_succeed() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("list")
            .arg("service")
            .arg("--filter")
            .arg("foo")
            .assert()
            .success()
            .stdout("No services found with name or alias containing `foo`\n");
        Ok(())
    }

    #[test]
    fn given_list_services_command_with_multiple_services_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("service1")
                    .with_alias("alias1")
            )
            .with_service(
                ServiceBuilder::new()
                    .with_name("service2")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("list")
            .arg("service")
            .assert()
            .success()
            .stdout(" - service1 (alias1)\n - service2\n");
        Ok(())
    }

    #[test]
    fn given_list_services_command_with_filter_and_multiple_services_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_service(
                ServiceBuilder::new()
                    .with_name("service1")
                    .with_alias("alias1")
            )
            .with_service(
                ServiceBuilder::new()
                    .with_name("foo_service2")
                    .with_alias("alias2")
            )
            .with_service(
                ServiceBuilder::new()
                    .with_name("service3")
                    .with_alias("foo_alias3")
            )
            .with_service(
                ServiceBuilder::new()
                    .with_name("service5")
                    .with_alias("alias5")
            )
            .build();
        setup(Some(config));

        Command::cargo_bin("htrs")?
            .arg("list")
            .arg("service")
            .arg("--filter")
            .arg("foo")
            .assert()
            .success()
            .stdout(" - foo_service2 (alias2)\n - service3 (foo_alias3)\n");
        Ok(())
    }
}
