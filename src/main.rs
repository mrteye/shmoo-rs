use clap::{Arg, Command};

fn main() {
    let matches = Command::new("myapp")
        .version("1.0")
        .author("Tyler Gillispie <mrteye@gmail.com>")
        .about("Does something...")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Sets level of verbosity.")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");

    if verbose {
        println!("Verbose enabled.");
    } else {
        println!("Verbose not enabled");
    }

    println!("Hello, world!");
}
