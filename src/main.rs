use clap::{arg, Command};

fn cli() -> Command {
    Command::new("rcsvd")
        .about("Parse CSVs into DB Tables")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("read")
                .about("Read and print out csv")
                .arg(arg!(<PATH> "The path to the csv file"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("read", matches)) => {
            let path = matches.get_one::<String>("PATH").expect("Path is missing");
            println!("Path: {}", path);
        }
        _ => unreachable!(),
    }
}
