mod edit_preset_tests {
    use crate::common::test_helpers::{clear_config, get_config, setup, HtrsConfigBuilder, PresetBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    pub fn given_unknown_preset_when_edit_then_should_fail() -> Result<(), Box<dyn Error>> {
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("edit")
            .arg("preset")
            .arg("unknown_preset")
            .assert()
            .failure()
            .stdout("No preset found with name `unknown_preset`\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    pub fn given_known_preset_when_edit_name_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_preset(
                PresetBuilder::new()
                    .with_name("old_name")
                    .with_value("foo", "bar")
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("edit")
            .arg("preset")
            .arg("old_name")
            .arg("--new-name")
            .arg("new_name")
            .assert()
            .success();

        let config = get_config(&path);
        assert_eq!(config.presets.len(), 1);
        assert_eq!(config.presets[0].name, "new_name");
        assert_eq!(config.presets[0].values.len(), 1);
        assert_eq!(config.presets[0].values["foo"], "bar");

        clear_config(&path);
        Ok(())
    }

    #[test]
    pub fn given_known_preset_when_edit_name_to_existing_then_should_fail() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_preset(
                PresetBuilder::new()
                    .with_name("old_name")
                    .with_value("foo", "bar")
            )
            .with_preset(
                PresetBuilder::new()
                    .with_name("existing_preset")
                    .with_value("foo", "bar")
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("edit")
            .arg("preset")
            .arg("old_name")
            .arg("--new-name")
            .arg("existing_preset")
            .assert()
            .failure()
            .stdout("A preset already exists with name `existing_preset`\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    pub fn given_known_preset_when_set_new_param_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_preset(
                PresetBuilder::new()
                    .with_name("foo_preset")
                    .with_value("foo", "bar")
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("edit")
            .arg("preset")
            .arg("foo_preset")
            .arg("--set")
            .arg("test=kek")
            .assert()
            .success();

        let config = get_config(&path);
        assert_eq!(config.presets.len(), 1);
        assert_eq!(config.presets[0].name, "foo_preset");
        assert_eq!(config.presets[0].values.len(), 2);
        assert_eq!(config.presets[0].values["foo"], "bar");
        assert_eq!(config.presets[0].values["test"], "kek");

        clear_config(&path);
        Ok(())
    }

    #[test]
    pub fn given_known_preset_when_set_existing_param_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_preset(
                PresetBuilder::new()
                    .with_name("foo_preset")
                    .with_value("foo", "bar")
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("edit")
            .arg("preset")
            .arg("foo_preset")
            .arg("--set")
            .arg("foo=kek")
            .assert()
            .success();

        let config = get_config(&path);
        assert_eq!(config.presets.len(), 1);
        assert_eq!(config.presets[0].name, "foo_preset");
        assert_eq!(config.presets[0].values.len(), 1);
        assert_eq!(config.presets[0].values["foo"], "kek");

        clear_config(&path);
        Ok(())
    }

    #[test]
    pub fn given_known_preset_when_clear_existing_param_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_preset(
                PresetBuilder::new()
                    .with_name("foo_preset")
                    .with_value("foo", "bar")
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("edit")
            .arg("preset")
            .arg("foo_preset")
            .arg("--clear")
            .arg("foo")
            .assert()
            .success();

        let config = get_config(&path);
        assert_eq!(config.presets.len(), 1);
        assert_eq!(config.presets[0].name, "foo_preset");
        assert_eq!(config.presets[0].values.len(), 0);

        clear_config(&path);
        Ok(())
    }
}