use clap::Parser;

/// Simple command like a echo
#[derive(Parser)]
struct Args {
    /// value to display
    value: Vec<String>,

    ///no new line
    #[arg(short, long)]
    no_newline: bool,
}

fn main() {
    let args = Args::parse();

    let output: String = args.value.join(" ");

    if args.no_newline {
        print!("{}", output);
    }
    else {
        println!("{}", output);
    }
}
