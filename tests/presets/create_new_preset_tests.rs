#[cfg(test)]
mod create_new_preset_tests {
    use crate::common::builders::{HtrsConfigBuilder, PresetBuilder};
    use crate::common::test_helpers::{clear_config, get_config, setup};
    use assert_cmd::Command;
    use rstest::rstest;
    use std::error::Error;

    #[test]
    fn given_create_new_preset_command_without_args_then_should_fail() -> Result<(), Box<dyn Error>> {
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("preset")
            .assert()
            .failure();

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_create_new_preset_command_with_name_and_no_values_then_should_fail() -> Result<(), Box<dyn Error>> {
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("preset")
            .arg("foo_preset")
            .assert()
            .failure();

        clear_config(&path);
        Ok(())
    }

    #[rstest]
    #[case("foo")]
    #[case("foo=")]
    #[case("=")]
    #[case("=foo")]
    fn given_create_new_preset_command_with_invalid_value_then_should_fail(
        #[case] param: &str
    ) -> Result<(), Box<dyn Error>> {
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("preset")
            .arg("foo_preset")
            .arg("--value")
            .arg(param)
            .assert()
            .failure()
            .stdout(format!("Invalid preset value `{}`, should be in format `key=value`\n", param));

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_create_new_preset_command_with_args_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("preset")
            .arg("foo_preset")
            .arg("--alias")
            .arg("foo_alias")
            .arg("--value")
            .arg("key=value")
            .assert()
            .success();

        let config = get_config(&path);
        assert_eq!(config.presets.len(), 1);
        assert_eq!(config.presets[0].name, "foo_preset");
        assert_eq!(config.presets[0].alias, Some("foo_alias".to_string()));
        assert_eq!(config.presets[0].values.len(), 1);
        assert_eq!(config.presets[0].values["key"], "value");

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_create_new_preset_command_with_existing_name_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_preset(
                PresetBuilder::new()
                    .with_name("existing_preset")
                    .with_value("foo", "bar")
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("new")
            .arg("preset")
            .arg("existing_preset")
            .arg("--value")
            .arg("foo=bar")
            .assert()
            .failure()
            .stdout("A preset with name or alias `existing_preset` already exists\n");

        clear_config(&path);
        Ok(())
    }
}