use clap::{Arg, Command};
use shmoo::stuff::hey;

fn main() {
  let version = env!("CARGO_PKG_VERSION");
  let about = env!("CARGO_PKG_DESCRIPTION");

  let matches = Command::new("shmoo")
    .version(version)
    .about(about)
    .arg(
      Arg::new("hey")
        .short('e')
        .long("hey")
        .value_parser(clap::value_parser!(String))
        .help("Identifies a Shmoo: accepts single arg - noun"),
    )
    .get_matches();

  if let Some(val) = matches.get_one::<String>("hey") {
    println!("{}", hey(val))
  } else {
    println!("Hello, shmoo!");
  }
}
