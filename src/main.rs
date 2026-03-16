use clap::Parser;

fn main() {
    let args = thin::cli::Args::parse();
    std::process::exit(thin::cli::run(args));
}
