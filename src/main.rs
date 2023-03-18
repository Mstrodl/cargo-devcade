use clap::Parser;
use std::env;
mod cmd_build;
mod cmd_package;
mod cmd_publish;
mod path_finder;
use cmd_build::build;
use cmd_package::package;
use cmd_publish::publish;
use path_finder::find_package;
use std::iter::once;

/// Build tooling for Rusty devcade software
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
  /// Binary to run
  #[arg(long)]
  pub bin: Option<String>,

  #[command(subcommand)]
  action: Action,
}

#[derive(clap::Subcommand, Debug, PartialEq)]
enum Action {
  /// Produces a zip
  Package,
  /// Pushes a zip to the API
  Publish,
  /// Builds the underlying executable
  Build,
}

fn main() {
  env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("cargo_devcade=info"))
    .format_timestamp(None)
    .init();

  let args = match env::args().nth(1).map(|s| s == "devcade") {
    Some(true) => Args::parse_from(once("cargo devcade".to_owned()).chain(env::args().skip(2))),
    _ => Args::parse(),
  };
  let package_info = find_package(&args);

  build(&package_info);
  if args.action != Action::Build {
    package(&package_info);
  }
  if Action::Publish == args.action {
    publish(&args);
  }
}
