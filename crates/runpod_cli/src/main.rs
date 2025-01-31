use clap::Parser;
use runpod::config::Config;
use runpod::RunpodClient;
use std::error::Error;
use tabled::Table;
use tracing::error;

mod opts;
use opts::{Cli, Commands, GpuCommands, PodCommands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Get API key from CLI arg or environment variable
    // let api_key = cli
    //     .api_key
    //     .expect("API key must be provided either via --api-key or RUNPOD_API_KEY env variable");
    let config = Config::try_from_env().expect("Failed to load config");
    let client = RunpodClient::new(config);

    match cli.command {
        Commands::Pod { command } => match command {
            PodCommands::List { verbose } => match client.list_pods().await {
                Ok(pods) => {
                    println!("{}", Table::new(pods).to_string());
                }
                Err(e) => {
                    error!("Failed to list pods: {}", e);
                    std::process::exit(1);
                }
            },
            PodCommands::Stop { id } => match client.stop_pod(&id).await {
                Ok(_) => {
                    println!("Pod {} stopped successfully", id);
                }
                Err(e) => {
                    error!("Failed to stop pod: {}", e);
                    std::process::exit(1);
                }
            },
            PodCommands::Terminate { id } => match client.terminate_pod(&id).await {
                Ok(_) => {
                    println!("Pod {} terminated successfully", id);
                }
                Err(e) => {
                    error!("Failed to terminate pod: {}", e);
                    std::process::exit(1);
                }
            },
            PodCommands::Get { id } => match client.get_pod(&id).await {
                Ok(Some(pod)) => {
                    println!("Pod: {pod:#?}");
                }
                Ok(None) => {
                    println!("Pod with id {id} not found");
                    std::process::exit(1);
                }
                Err(e) => {
                    error!("Failed to get pod: {}", e);
                    std::process::exit(1);
                }
            },
            PodCommands::Spawn {
                name,
                gpu,
                count,
                spot,
                bid,
                disk,
            } => {
                if spot && bid.is_none() {
                    error!("Must specify --bid when using --spot");
                    std::process::exit(1);
                }

                match client.spawn_pod(name, gpu, count, spot, bid, disk).await {
                    Ok(pod) => {
                        println!("Successfully spawned pod:");
                        println!("{}", Table::new(vec![pod]).to_string());
                    }
                    Err(e) => {
                        error!("Failed to spawn pod: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        },
        Commands::Gpu { command } => match command {
            GpuCommands::List {
                lowest_price,
                secure,
            } => match client.list_gpus(secure).await {
                Ok(gpus) => {
                    if lowest_price {
                        let gpus: Vec<_> = gpus
                            .into_iter()
                            .filter_map(|gpu| gpu.lowest_price)
                            .collect();
                        println!("{}", Table::new(gpus).to_string());
                    } else {
                        println!("{}", Table::new(gpus).to_string());
                    }
                }
                Err(e) => {
                    error!("Failed to get GPUs: {}", e);
                    std::process::exit(1);
                }
            },
        },
    }

    Ok(())
}
