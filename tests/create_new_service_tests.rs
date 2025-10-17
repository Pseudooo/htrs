mod helpers;

use assert_cmd::prelude::*;
use crate::helpers::{get_config, setup};
// Add methods on commands
use predicates::prelude::*;
use std::error::Error;
use std::process::Command;

#[test]
fn given_create_new_service_when_no_arguments_then_should_err() -> Result<(), Box<dyn Error>> {
    setup();

    let mut cmd = Command::cargo_bin("htrs")?;

    cmd.arg("new")
        .arg("service");
    cmd.assert()
        .failure();

    Ok(())
}

#[test]
fn given_valid_create_new_service_command_then_should_succeed() -> Result<(), Box<dyn Error>> {
    setup();

    let mut cmd = Command::cargo_bin("htrs")?;

    cmd.arg("new")
        .arg("service")
        .arg("foo")
        .arg("--alias").arg("bar");
    cmd.assert()
        .success();

    let config = get_config();
    let service = &config["services"][0];
    assert_eq!(service["name"], "foo");
    assert_eq!(service["alias"], "bar");

    Ok(())
}
