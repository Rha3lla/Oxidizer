use clap::Parser;
use oxidizer_core::config::Config;

#[derive(Parser)]
#[command(name = "cargo-sast", about = "High-Assurance Static Analysis for Rust")]
struct Args {
    #[arg(long, default_value = "auto-detect")]
    config_path: String, 

    #[arg(long, default_value = "sarif")]
    format: String,

    #[arg(long, default_value = "error")]
    fail_on: String,

    #[arg(long)]
    profile: Option<String>,

    #[arg(short, long)]
    verbose: bool,
}



fn main() {
    let _args = Args::parse();
    // Here we will load the configuration, run the analysis, and output results.
    let _config = Config::load(std::path::Path::new("rust-sa.toml"));

    //stub message that needs to print for the user
   println!("rust-sa v0.1.0 - analysis pipeline not yet wired (M0 scaffold)");
}