
mod process;

use clap::{arg, command, Parser};
use process::process_blackboard_zip;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to zip file
    #[arg()]
    source: String,

    /// path to output folder (will create subfolder inside)
    #[arg(default_value=".")]
    dest: String,

    ///Whether to replace user names with numbers
    #[arg(short='a', long="anon", default_value_t=false)]
    anonymize: bool
}


fn main() {

    let args = Args::parse();

    process_blackboard_zip(&args.source, &args.dest, args.anonymize).unwrap();

}