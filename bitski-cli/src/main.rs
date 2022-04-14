use bitski::Bitski;
use clap::{Parser, Subcommand};
use tracing_log::LogTracer;
use web3::Transport;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Action,
}

#[derive(Subcommand)]
enum Action {
    #[clap(author, version, about, long_about = None)]
    Execute {
        #[clap(short, long, default_value = "mainnet")]
        network: String,

        #[clap(short, long)]
        method: String,

        /// Number of times to greet
        #[clap(short, long)]
        params: Vec<serde_json::Value>,
    },
}

/// The main entry point for the Bitski CLI. Just gets a list of accounts for now and prints them.
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let _ = LogTracer::init();

    let args = Cli::parse();

    match args.command {
        Action::Execute {
            network,
            method,
            params,
        } => {
            let bitski = Bitski::from_env().expect("Could not initialize");
            let provider = bitski
                .get_provider(network.as_ref())
                .expect("Could not get provider");

            let result = provider.execute(&method, params).await;

            match result {
                Ok(result) => println!("{}", result),
                Err(err) => println!("Error: {:?}", err),
            }
        }
    }
}
