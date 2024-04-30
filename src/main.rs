mod app;
mod cli;
mod config;

fn main() {
    let args = cli::CommandLineArgs::parse_args();
    println!("Refresh rate: {}", args.refresh_rate);
    println!("Show graphs: {}", args.show_graphs);
}
