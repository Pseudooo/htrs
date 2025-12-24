#[cfg(test)]
mod delete_preset_tests {
    use crate::common::test_helpers::{clear_config, get_config, setup, HtrsConfigBuilder, PresetBuilder};
    use assert_cmd::Command;
    use std::error::Error;

    #[test]
    fn given_unknown_preset_then_should_fail() -> Result<(), Box<dyn Error>> {
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("delete")
            .arg("preset")
            .arg("unknown_preset")
            .assert()
            .failure()
            .stdout("Unable to find preset with name `unknown_preset`\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    fn given_known_preset_then_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_preset(
                PresetBuilder::new()
                    .with_name("existing_preset")
                    .with_value("key", "value")
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("delete")
            .arg("preset")
            .arg("existing_preset")
            .assert()
            .success();

        let config = get_config(&path);
        assert_eq!(config.presets.len(), 0);

        clear_config(&path);
        Ok(())
    }
}