use crate::command_args::{RootCommands, ServiceCommands};
use clap::{Arg, ArgMatches, Command};

pub fn map_command(args: ArgMatches) -> RootCommands {
    match args.subcommand() {
        Some(("service", service_matches)) => {
            match service_matches.subcommand() {
                Some(("add", add_service_matches)) => {
                    let Some(name) = add_service_matches.get_one::<String>("name") else {
                        panic!("bad add command")
                    };
                    RootCommands::Service(
                        ServiceCommands::Add {
                            name: name.to_string(),
                        }
                    )
                },
                Some(("remove" | "rm", remove_service_matches)) => {
                    let Some(name) = remove_service_matches.get_one::<String>("name") else {
                        panic!("bad remove command")
                    };
                    RootCommands::Service(
                        ServiceCommands::Remove {
                            name: name.to_string(),
                        }
                    )
                },
                Some(("list" | "ls", _)) => {
                    RootCommands::Service(
                        ServiceCommands::List
                    )
                }
                _ => panic!("Bad service command")
            }
        },

        _ => panic!("scrEEEEch")
    }
}

pub fn get_root_command() -> Command {
    let command = Command::new("htrs")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A flexible http cli client")
        .subcommand(get_service_command());

    command
}

fn get_service_command() -> Command {
    let command = Command::new("service")
        .about("Service configuration commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Create a new service")
                .arg(
                    Arg::new("name")
                        .help("Unique name of the service to create")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("remove")
                .visible_alias("rm")
                .about("Remove a service")
                .arg(
                    Arg::new("name")
                        .help("Service name to remove")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("list")
                .visible_alias("ls")
                .about("List all services")
        );

    command
}

#[cfg(test)]
mod command_builder_tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn given_valid_add_service_command_then_should_parse_and_map() {
        let args = vec!["htrs", "service", "add", "foo"];

        let result = get_root_command().try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(err) => panic!("Failed to get matches - {err}")
        };
        let mapped_command = map_command(matches);

        let RootCommands::Service(service_command) = mapped_command else {
            panic!("Command was not service command");
        };
        let ServiceCommands::Add { name } = service_command else {
            panic!("Command was not add service command");
        };
        assert_eq!(name, "foo")
    }

    #[rstest]
    #[case("remove")]
    #[case("rm")]
    fn given_valid_remove_service_command_then_should_parse_and_map(#[case] remove_alias: &str) {
        let args = vec!["htrs", "service", remove_alias, "foo"];

        let result = get_root_command().try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(e) => panic!("Failed to get matches - {e}")
        };
        let mapped_command = map_command(matches);

        let RootCommands::Service(service_command) = mapped_command else {
            panic!("Command was not RootCommands::Service");
        };
        let ServiceCommands::Remove { name } = service_command else {
            panic!("Command was not ServiceCommands::Remove")
        };
        assert_eq!(name, "foo");
    }

    #[rstest]
    #[case("list")]
    #[case("ls")]
    fn given_valid_list_services_command_then_should_parse_and_map(#[case] list_alias: &str) {
        let args = vec!["htrs", "service", list_alias];

        let result = get_root_command().try_get_matches_from(args);
        let matches = match result {
            Ok(res) => res,
            Err(e) => panic!("Failed to get matches - {e}")
        };

        let RootCommands::Service(service_command) = map_command(matches) else {
            panic!("Command was not RootCommands::Service")
        };
        assert!(matches!(service_command, ServiceCommands::List));
    }
}

