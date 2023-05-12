use clap::{arg, Command};

fn cli() -> Command {
    Command::new("rcsvd")
        .about("Parse CSVs into DB Tables")
        .arg_required_else_help(true)
        .subcommand(Command::new("test").about("testing this out"))
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("test", matches)) => {
            println!("WEEEEE");
        }
        _ => unreachable!(),
    }
}
