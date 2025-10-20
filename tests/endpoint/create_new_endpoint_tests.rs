mod create_new_endpoint_tests {
    use crate::common::test_helpers::setup;
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_new_endpoint_command_with_no_args_should_fail() -> Result<(), Box<dyn Error>> {
        setup(None);

        Command::cargo_bin("htrs")?
            .arg("new")
            .arg("endpoint")
            .assert()
            .failure();
        Ok(())
    }
}
