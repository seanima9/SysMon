use clap::{Arg, ArgMatches, Command};

/// Struct to hold command line arguments
pub struct CommandLineArgs {
    pub refresh_rate: u64,
    pub show_graphs: bool,
}

impl CommandLineArgs {
    /// Parse command line arguments
    pub fn parse_args() -> CommandLineArgs {
        let matches = CommandLineArgs::get_command_matches();

        let max_refresh_rate = 300; // TODO: Load this from a config file
        let mut refresh_rate = matches
            .get_one::<u64>("refresh_rate")
            .map(|value| *value)
            .unwrap_or(144);

        if refresh_rate > max_refresh_rate {
            // Don't want users to crash their systems
            eprintln!(
                "Warning: Refresh rate is too high, setting to {}",
                max_refresh_rate
            );
            refresh_rate = max_refresh_rate;
        }

        CommandLineArgs {
            refresh_rate,
            show_graphs: matches.get_flag("show_graphs"),
        }
    }

    /// Define and get command line arguments
    fn get_command_matches() -> ArgMatches {
        Command::new("Linux Process Viewer")
            .version("1.0")
            .author("Sean Imani")
            .about("Monitors and displays system process information")
            .arg(
                Arg::new("refresh_rate")
                    .short('r')
                    .long("refresh")
                    .value_name("MILLISECONDS")
                    .help("Sets the refresh rate of the UI updates in milliseconds")
                    .default_value("144")
                    .value_parser(clap::value_parser!(u64)), // Ensure correct type parsing
            )
            .arg(
                Arg::new("show_graphs")
                    .short('g')
                    .long("graphs")
                    .help("Enables detailed graphical display of system stats")
                    .action(clap::ArgAction::SetTrue), // Change to SetTrue to handle flag presence
            )
            .get_matches()
    }
}
