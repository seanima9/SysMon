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

        let min_refresh_rate = 500; // TODO: Load this from a config file
        let mut refresh_rate = matches
            .get_one::<u64>("refresh_rate")
            .map(|value| *value)
            .unwrap_or(1000);

        if refresh_rate > min_refresh_rate {
            // Sysinfo CPU minimum update interval is 200ms
            eprintln!(
                "Warning: The refresh rate is too low for sysinfo. Setting it to {}ms",
                min_refresh_rate
            );
            refresh_rate = min_refresh_rate;
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
                    .default_value("1000")
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
