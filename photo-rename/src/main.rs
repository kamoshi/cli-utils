mod photo_rename;

use clap::Parser;


/// CLI util for mass renaming photos
#[derive(Parser, Debug)]
struct Args {
    /// Path to the input directory
    #[arg(short, long)]
    input: String,

    /// Path to the output directory
    #[arg(short, long)]
    output: String,
}


fn main() {
    let args = Args::parse();

    let input: String = args.input;
    let output: String = args.output;
    match photo_rename::run_for(input.as_str(), output.as_str()) {
        Ok(_) => println!("Task finished!"),
        Err(err) => eprintln!("{}", err),
    }
}
