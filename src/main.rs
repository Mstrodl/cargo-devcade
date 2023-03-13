use clap::Parser;
mod cmd_build;
mod cmd_package;
mod path_finder;
use cmd_build::build;
use cmd_package::package;
use path_finder::find_package;

/// Simple program to greet a person
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
  let args = Args::parse();
  let package_info = find_package(&args);

  build(&package_info);
  if args.action != Action::Build {
    package(&package_info);
  }
  if Action::Publish == args.action {
    // publish(&args);
  }
}
