mod photo_rename;

use clap::Parser;


/// CLI util for mass renaming photos
#[derive(Parser, Debug)]
struct Args {
    /// Path to the directory
    #[arg(short, long)]
    directory: String,
}


fn main() {
    let args = Args::parse();
    let dir: String = args.directory;

    match photo_rename::run_for(dir.as_str()) {
        Ok(_) => println!("Task finished!"),
        Err(err) => eprintln!("{}", err),
    }
}
