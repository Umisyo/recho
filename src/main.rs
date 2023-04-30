use clap::Parser;

/// Simple command like a echo
#[derive(Parser)]
struct Args {
    /// value to display
    value: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let output: String = args.value.join(" ");

    println!("{}", output);
}
