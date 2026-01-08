
mod view_preset_tests {
    use crate::common::builders::{HtrsConfigBuilder, PresetBuilder};
    use crate::common::test_helpers::{clear_config, setup};
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::error::Error;

    #[test]
    pub fn given_unknown_preset_when_view_then_should_fail() -> Result<(), Box<dyn Error>> {
        let path = setup(None);

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("view")
            .arg("preset")
            .arg("unknown_preset")
            .assert()
            .failure()
            .stdout("No preset could be found with name or alias `unknown_preset`\n");

        clear_config(&path);
        Ok(())
    }

    #[test]
    pub fn given_known_preset_name_when_view_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_preset(
                PresetBuilder::new()
                    .with_name("foo_name")
                    .with_value("key", "value")
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("view")
            .arg("preset")
            .arg("foo_name")
            .assert()
            .success()
            .stdout(
                predicate::str::contains("foo_name")
                    .and(predicate::str::contains(" - key: value"))
            );

        clear_config(&path);
        Ok(())
    }

    #[test]
    pub fn given_known_preset_alias_when_view_should_succeed() -> Result<(), Box<dyn Error>> {
        let config = HtrsConfigBuilder::new()
            .with_preset(
                PresetBuilder::new()
                    .with_name("foo_name")
                    .with_alias("foo_alias")
                    .with_value("key", "value")
            )
            .build();
        let path = setup(Some(config));

        Command::cargo_bin("htrs")?
            .env("HTRS_CONFIG_PATH", &path)
            .arg("view")
            .arg("preset")
            .arg("foo_alias")
            .assert()
            .success()
            .stdout(
                predicate::str::contains("foo_name (foo_alias)")
                    .and(predicate::str::contains(" - key: value"))
            );

        clear_config(&path);
        Ok(())
    }
}
