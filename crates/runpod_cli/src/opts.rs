use clap::{Parser, Subcommand};
// use runpod::types::SaveTemplateInput;

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
    /// List all templates
    Template {
        /// List all templates
        #[command(subcommand)]
        command: TemplateCommands,
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
    List {},

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

        /// Template ID
        #[arg(long)]
        template: String,
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
pub enum TemplateCommands {
    /// List all templates
    List {},

    /// Save or update a template
    SaveTemplate {
        /// Template ID (optional for new templates)
        #[arg(long)]
        id: Option<String>,

        /// Template name
        #[arg(long)]
        name: String,

        /// Docker image name
        #[arg(long)]
        image_name: String,

        /// Container disk size in GB
        #[arg(long)]
        #[arg(long, default_value = "20")]
        container_disk_in_gb: i64,

        /// Volume size in GB
        #[arg(long)]
        #[arg(long, default_value = "60.0")]
        volume_in_gb: Option<f64>,

        /// Volume mount path
        #[arg(long)]
        volume_mount_path: Option<String>,

        /// Container ports (e.g. "8080/tcp,8081/udp")
        #[arg(long)]
        ports: Option<String>,

        /// Environment variables (e.g. "KEY1=value1,KEY2=value2")
        #[arg(long)]
        env: Option<String>,

        /// Number of GPUs
        #[arg(long, default_value = "1")]
        gpu_count: Option<i64>,

        /// Number of vCPUs
        #[arg(long, default_value = "4.0")]
        vcpu_count: Option<f64>,

        /// Memory in GB
        #[arg(long)]
        #[arg(long, default_value = "16.0")]
        memory_in_gb: Option<f64>,

        /// Docker arguments
        #[arg(long)]
        docker_args: Option<String>,

        /// Container registry auth ID
        #[arg(long)]
        container_registry_auth_id: Option<String>,
    },

    /// Remove a template
    RemoveTemplate {
        /// Template ID
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

        /// Minimum VRAM required in GB (can be satisfied by multiple GPUs)
        #[arg(long)]
        vram: Option<i64>,
    },
}
