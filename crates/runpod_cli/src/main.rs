use clap::Parser;
use runpod::RunpodClient;
use std::error::Error;
use tabled::Table;
use tracing::error;

mod opts;
use opts::{Cli, Commands, GpuCommands, PodCommands, TemplateCommands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let opts = Cli::parse();
    let client = RunpodClient::from_config()?;

    match opts.command {
        Commands::Pod { command } => match command {
            PodCommands::List {} => match client.list_pods().await {
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
                template,
            } => {
                if spot && bid.is_none() {
                    error!("Must specify --bid when using --spot");
                    std::process::exit(1);
                }

                match client
                    .spawn_pod(name, gpu, count, spot, bid, disk, template)
                    .await
                {
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
                vram,
            } => match client.list_gpus(secure).await {
                Ok(mut gpus) => {
                    // Filter by VRAM if specified
                    if let Some(min_vram) = vram {
                        gpus = gpus
                            .into_iter()
                            .filter(|gpu| {
                                let max_gpus = gpu.max_gpu_count.unwrap_or(1);
                                let gpu_vram = gpu.memory_in_gb.unwrap_or(0);
                                gpu_vram * max_gpus >= min_vram
                            })
                            .collect();
                    }

                    if lowest_price {
                        let lowest_price_gpus: Vec<_> = gpus
                            .into_iter()
                            .filter_map(|gpu| gpu.lowest_price)
                            .collect();
                        if lowest_price_gpus.is_empty() {
                            println!("No GPUs found matching the criteria");
                        } else {
                            println!("{}", Table::new(lowest_price_gpus).to_string());
                        }
                    } else {
                        if gpus.is_empty() {
                            println!("No GPUs found matching the criteria");
                        } else {
                            println!("{}", Table::new(gpus).to_string());
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to get GPUs: {}", e);
                    std::process::exit(1);
                }
            },
        },
        Commands::Template { command } => match command {
            TemplateCommands::List {} => {
                let templates = client.get_templates().await?;
                let table = Table::new(templates);
                println!("{table}");
            }
            _ => unimplemented!(),
        },
    }

    Ok(())
}
