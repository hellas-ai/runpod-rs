use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// List or show a pod
    #[command(subcommand)]
    pub command: Commands,

    /// API key for Runpod.ai (can also be set via RUNPOD_API_KEY environment variable)
    #[arg(global = true, short, long, env = "Runpod_API_KEY")]
    pub api_key: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage pods
    Pod {
        #[command(subcommand)]
        command: PodCommands,
    },
    /// Manage GPUs
    Gpu {
        #[command(subcommand)]
        command: GpuCommands,
    },
}

#[derive(Subcommand)]
pub enum PodCommands {
    /// Get details of a specific pod
    Get {
        /// Pod ID to get details for
        id: String,
    },
    /// List all pods
    List {
        /// Show all fields
        #[arg(short, long)]
        verbose: bool,
    },
    /// Spawn a new pod
    Spawn {
        /// Name for the new pod
        #[arg(short, long)]
        name: String,

        /// GPU type ID to use
        #[arg(short, long)]
        gpu: String,

        /// Number of GPUs to request
        #[arg(short, long, default_value = "1")]
        count: i64,

        /// Use spot/interruptible instances (cheaper but can be terminated)
        #[arg(long)]
        spot: bool,

        /// Maximum bid price per GPU (only valid with --spot)
        #[arg(long)]
        bid: Option<f64>,

        /// Container disk size in GB
        #[arg(long)]
        disk: Option<i64>,
    },
    /// Stop a pod
    Stop {
        /// Pod ID to stop
        id: String,
    },
    /// Terminate a pod
    Terminate {
        /// Pod ID to terminate
        id: String,
    },
}

#[derive(Subcommand)]
pub enum GpuCommands {
    /// List available GPUs
    List {
        /// Show the 'Lowest Price' GPUs
        #[arg(short, long)]
        lowest_price: bool,

        /// Show only secure cloud GPUs
        #[arg(short, long)]
        secure: Option<bool>,
    },
}
