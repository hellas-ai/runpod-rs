use crate::gql::*;
use std::fmt::Display;
use tracing::error;

#[cfg_attr(
    feature = "tabled",
    derive(tabled::Tabled),
    tabled(display(Option, "display_option", ""))
)]
#[derive(Debug, Clone)]
pub struct Pod {
    pub id: String,
    pub name: String,
    pub pod_type: Option<PodType>,
    pub desired_status: PodStatus,
    pub image_name: String,
    pub gpu_count: i64,
    pub vcpu_count: f64,
    pub memory_in_gb: f64,
    pub volume_in_gb: Option<f64>,
    pub container_disk_in_gb: i64,
    pub adjusted_cost_per_hr: f64,
    pub lowest_bid_price_to_resume: Option<f64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub docker_args: Option<String>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub env: Vec<String>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub runtime: Option<PodRuntime>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub machine: PodMachineInfo,
}

#[derive(Debug, Clone)]
pub struct PodRuntime {
    pub uptime_in_seconds: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct PodMachineInfo {
    pub id: String,
    pub gpu_type: Option<GpuType>,
    pub location: String,
    pub machine_system: MachineSystem,
}

#[derive(Debug, Clone)]
pub enum PodType {
    Interruptable,
    Reserved,
    Bid,
    Background,
}

impl Display for PodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PodType::Interruptable => write!(f, "Interruptable"),
            PodType::Reserved => write!(f, "Reserved"),
            PodType::Bid => write!(f, "Bid"),
            PodType::Background => write!(f, "Background"),
        }
    }
}

impl From<myself_query::PodType> for PodType {
    fn from(pod_type: myself_query::PodType) -> Self {
        match pod_type {
            myself_query::PodType::INTERRUPTABLE => PodType::Interruptable,
            myself_query::PodType::RESERVED => PodType::Reserved,
            myself_query::PodType::BID => PodType::Bid,
            myself_query::PodType::BACKGROUND => PodType::Background,
            _ => {
                error! {"Unknown pod type: {:?}", pod_type};
                PodType::Background
            }
        }
    }
}

impl From<get_pod::PodType> for PodType {
    fn from(pod_type: get_pod::PodType) -> Self {
        match pod_type {
            get_pod::PodType::INTERRUPTABLE => PodType::Interruptable,
            get_pod::PodType::RESERVED => PodType::Reserved,
            get_pod::PodType::BID => PodType::Bid,
            get_pod::PodType::BACKGROUND => PodType::Background,
            _ => {
                error! {"Unknown pod type: {:?}", pod_type};
                PodType::Background
            }
        }
    }
}

impl From<myself_query::MyselfQueryMyselfPods> for Pod {
    fn from(pod: myself_query::MyselfQueryMyselfPods) -> Self {
        Self {
            id: pod.id,
            name: pod.name,
            pod_type: pod.pod_type.map(Into::into),
            desired_status: pod.desired_status.into(),
            image_name: pod.image_name,
            gpu_count: pod.gpu_count,
            vcpu_count: pod.vcpu_count,
            memory_in_gb: pod.memory_in_gb,
            volume_in_gb: pod.volume_in_gb,
            container_disk_in_gb: pod.container_disk_in_gb,
            adjusted_cost_per_hr: pod.adjusted_cost_per_hr,
            lowest_bid_price_to_resume: pod.lowest_bid_price_to_resume,
            runtime: pod.runtime.map(Into::into),
            machine: pod.machine.into(),
            docker_args: pod.docker_args,
            env: pod
                .env
                .map(|env_vec| {
                    env_vec
                        .into_iter()
                        .filter_map(|opt_env| opt_env.map(Into::into))
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

impl From<get_pod::GetPodPod> for Pod {
    fn from(pod: get_pod::GetPodPod) -> Self {
        Self {
            id: pod.id,
            name: pod.name,
            docker_args: pod.docker_args,
            pod_type: pod.pod_type.map(Into::into),
            desired_status: pod.desired_status.into(),
            image_name: pod.image_name,
            gpu_count: pod.gpu_count,
            vcpu_count: pod.vcpu_count,
            memory_in_gb: pod.memory_in_gb,
            volume_in_gb: pod.volume_in_gb,
            container_disk_in_gb: pod.container_disk_in_gb,
            adjusted_cost_per_hr: pod.adjusted_cost_per_hr,
            lowest_bid_price_to_resume: pod.lowest_bid_price_to_resume,
            runtime: pod.runtime.map(Into::into),
            machine: pod.machine.into(),
            env: pod
                .env
                .map(|env_vec| env_vec.into_iter().filter_map(|opt_env| opt_env).collect())
                .unwrap_or_default(),
        }
    }
}

impl From<get_pod::GetPodPodRuntime> for PodRuntime {
    fn from(runtime: get_pod::GetPodPodRuntime) -> Self {
        Self {
            uptime_in_seconds: runtime.uptime_in_seconds,
        }
    }
}

impl From<get_pod::GetPodPodMachine> for PodMachineInfo {
    fn from(machine: get_pod::GetPodPodMachine) -> Self {
        Self {
            id: machine.id,
            gpu_type: machine.gpu_type.map(Into::into),
            location: machine.location,
            machine_system: machine.machine_system.into(),
        }
    }
}

impl From<get_pod::GetPodPodMachineMachineSystem> for MachineSystem {
    fn from(machine_system: get_pod::GetPodPodMachineMachineSystem) -> Self {
        Self {
            cuda_version: machine_system.cuda_version,
            kernel_version: machine_system.kernel_version,
        }
    }
}

impl From<get_pod::GetPodPodMachineGpuType> for GpuType {
    fn from(gpu_type: get_pod::GetPodPodMachineGpuType) -> Self {
        Self {
            id: gpu_type.id,
            memory_in_gb: gpu_type.memory_in_gb,
            cuda_cores: gpu_type.cuda_cores,
        }
    }
}

impl From<myself_query::MyselfQueryMyselfPodsRuntime> for PodRuntime {
    fn from(runtime: myself_query::MyselfQueryMyselfPodsRuntime) -> Self {
        Self {
            uptime_in_seconds: runtime.uptime_in_seconds,
        }
    }
}

impl From<myself_query::MyselfQueryMyselfPodsMachine> for PodMachineInfo {
    fn from(machine: myself_query::MyselfQueryMyselfPodsMachine) -> Self {
        Self {
            id: machine.id,
            gpu_type: machine.gpu_type.map(Into::into),
            location: machine.location,
            machine_system: machine.machine_system.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PodStatus {
    Created,
    Running,
    Restarting,
    Exited,
    Paused,
    Dead,
    Terminated,
}

impl From<myself_query::PodStatus> for PodStatus {
    fn from(status: myself_query::PodStatus) -> Self {
        match status {
            myself_query::PodStatus::CREATED => PodStatus::Created,
            myself_query::PodStatus::RUNNING => PodStatus::Running,
            myself_query::PodStatus::RESTARTING => PodStatus::Restarting,
            myself_query::PodStatus::EXITED => PodStatus::Exited,
            myself_query::PodStatus::PAUSED => PodStatus::Paused,
            myself_query::PodStatus::DEAD => PodStatus::Dead,
            myself_query::PodStatus::TERMINATED => PodStatus::Terminated,
            // This should never happen as we've covered all variants
            _ => PodStatus::Dead,
        }
    }
}

impl From<get_pod::PodStatus> for PodStatus {
    fn from(status: get_pod::PodStatus) -> Self {
        match status {
            get_pod::PodStatus::CREATED => PodStatus::Created,
            get_pod::PodStatus::RUNNING => PodStatus::Running,
            get_pod::PodStatus::RESTARTING => PodStatus::Restarting,
            get_pod::PodStatus::EXITED => PodStatus::Exited,
            get_pod::PodStatus::PAUSED => PodStatus::Paused,
            get_pod::PodStatus::DEAD => PodStatus::Dead,
            get_pod::PodStatus::TERMINATED => PodStatus::Terminated,
            // This should never happen as we've covered all variants
            _ => PodStatus::Dead,
        }
    }
}

impl std::fmt::Display for PodStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for PodRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for myself_query::PodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct MachineSystem {
    pub cuda_version: String,
    pub kernel_version: String,
}

impl From<myself_query::MyselfQueryMyselfPodsMachineMachineSystem> for MachineSystem {
    fn from(machine_system: myself_query::MyselfQueryMyselfPodsMachineMachineSystem) -> Self {
        Self {
            cuda_version: machine_system.cuda_version,
            kernel_version: machine_system.kernel_version,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GpuType {
    pub id: String,
    pub memory_in_gb: Option<i64>,
    pub cuda_cores: Option<i64>,
}

impl From<myself_query::MyselfQueryMyselfPodsMachineGpuType> for GpuType {
    fn from(gpu_type: myself_query::MyselfQueryMyselfPodsMachineGpuType) -> Self {
        Self {
            id: gpu_type.id,
            memory_in_gb: gpu_type.memory_in_gb,
            cuda_cores: gpu_type.cuda_cores,
        }
    }
}

#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[derive(Debug, Clone)]
pub enum Compliance {
    Gdpr,
    IsoIec27001,
    Iso14001,
    PciDss,
    Hitrust,
    Soc1Type2,
    Soc2Type2,
    Soc3Type2,
    Itar,
    FismaHigh,
}

impl From<gpu_types::Compliance> for Compliance {
    fn from(c: gpu_types::Compliance) -> Self {
        match c {
            gpu_types::Compliance::GDPR => Compliance::Gdpr,
            gpu_types::Compliance::ISO_IEC_27001 => Compliance::IsoIec27001,
            gpu_types::Compliance::ISO_14001 => Compliance::Iso14001,
            gpu_types::Compliance::PCI_DSS => Compliance::PciDss,
            gpu_types::Compliance::HITRUST => Compliance::Hitrust,
            gpu_types::Compliance::SOC_1_TYPE_2 => Compliance::Soc1Type2,
            gpu_types::Compliance::SOC_2_TYPE_2 => Compliance::Soc2Type2,
            gpu_types::Compliance::SOC_3_TYPE_2 => Compliance::Soc3Type2,
            gpu_types::Compliance::ITAR => Compliance::Itar,
            gpu_types::Compliance::FISMA_HIGH => Compliance::FismaHigh,
            other => {
                error! {"Unknown compliance type: {:?}", other};
                Compliance::Gdpr
            }
        }
    }
}

fn display_option<T>(opt: &Option<T>, default: &str) -> String
where
    T: ToString,
{
    match opt {
        Some(val) => val.to_string(),
        None => default.to_string(),
    }
}

#[cfg_attr(
    feature = "tabled",
    derive(tabled::Tabled),
    tabled(display(Option, "display_option", ""))
)]
#[derive(Debug, Clone)]
pub struct GpuOffer {
    pub id: String,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub display_name: String,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub manufacturer: Option<String>,
    pub memory_in_gb: Option<i64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub secure_cloud: Option<bool>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub community_cloud: Option<bool>,
    pub secure_price: Option<f64>,
    pub community_price: Option<f64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub one_month_price: Option<f64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub three_month_price: Option<f64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub six_month_price: Option<f64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub one_week_price: Option<f64>,
    pub community_spot_price: Option<f64>,
    pub secure_spot_price: Option<f64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub max_gpu_count: Option<i64>,
    pub max_gpu_count_community_cloud: Option<i64>,
    pub max_gpu_count_secure_cloud: Option<i64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub min_pod_gpu_count: Option<i64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub lowest_price: Option<LowestPrice>,
}

#[cfg_attr(
    feature = "tabled",
    derive(tabled::Tabled),
    tabled(display(Option, "display_option", ""))
)]
#[derive(Debug, Clone)]
pub struct LowestPrice {
    // pub valid: Option<bool>,
    pub gpu_name: Option<String>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub gpu_type_id: Option<String>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub minimum_bid_price: Option<f64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub uninterruptable_price: Option<f64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub min_memory: Option<i64>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub min_vcpu: Option<i64>,
    pub rental_percentage: Option<f64>,
    pub rented_count: Option<i64>,
    pub total_count: Option<i64>,
    pub stock_status: Option<String>,
    pub min_download: Option<i64>,
    pub min_disk: Option<i64>,
    pub min_upload: Option<i64>,
    pub country_code: Option<String>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub support_public_ip: Option<bool>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    pub compliance: Option<Vec<Compliance>>,
}

impl From<gpu_types::GpuTypesGpuTypes> for GpuOffer {
    fn from(gpu: gpu_types::GpuTypesGpuTypes) -> Self {
        Self {
            id: gpu.id,
            display_name: gpu.display_name.unwrap_or_default(),
            manufacturer: gpu.manufacturer,
            memory_in_gb: gpu.memory_in_gb,
            secure_cloud: gpu.secure_cloud,
            community_cloud: gpu.community_cloud,
            secure_price: gpu.secure_price,
            community_price: gpu.community_price,
            one_month_price: gpu.one_month_price,
            three_month_price: gpu.three_month_price,
            six_month_price: gpu.six_month_price,
            one_week_price: gpu.one_week_price,
            community_spot_price: gpu.community_spot_price,
            secure_spot_price: gpu.secure_spot_price,
            max_gpu_count: gpu.max_gpu_count,
            max_gpu_count_community_cloud: gpu.max_gpu_count_community_cloud,
            max_gpu_count_secure_cloud: gpu.max_gpu_count_secure_cloud,
            min_pod_gpu_count: gpu.min_pod_gpu_count,
            lowest_price: gpu.lowest_price.map(Into::into),
        }
    }
}

impl From<gpu_types::GpuTypesGpuTypesLowestPrice> for LowestPrice {
    fn from(price: gpu_types::GpuTypesGpuTypesLowestPrice) -> Self {
        Self {
            gpu_name: price.gpu_name,
            gpu_type_id: price.gpu_type_id,
            minimum_bid_price: price.minimum_bid_price,
            uninterruptable_price: price.uninterruptable_price,
            min_memory: price.min_memory,
            min_vcpu: price.min_vcpu,
            rental_percentage: price.rental_percentage,
            rented_count: price.rented_count,
            total_count: price.total_count,
            stock_status: price.stock_status,
            min_download: price.min_download,
            min_disk: price.min_disk,
            min_upload: price.min_upload,
            country_code: price.country_code,
            support_public_ip: price.support_public_ip,
            compliance: price
                .compliance
                .map(|v| v.into_iter().filter_map(|c| c.map(Into::into)).collect()),
        }
    }
}

#[cfg_attr(
    feature = "tabled",
    derive(tabled::Tabled),
    tabled(display(Option, "display_option", ""))
)]
#[derive(Debug, Clone)]
pub struct EnvironmentVariable {
    pub key: String,
    pub value: String,
}

impl From<get_templates::GetTemplatesMyselfPodTemplatesEnv> for EnvironmentVariable {
    fn from(env: get_templates::GetTemplatesMyselfPodTemplatesEnv) -> Self {
        Self {
            key: env.key.expect("env var without key"),
            value: env.value.expect("env var without"),
        }
    }
}

// impl From<get_template::GetTemplateMyselfTemplateEnv> for EnvironmentVariable {
//     fn from(env: get_template::GetTemplateMyselfTemplateEnv) -> Self {
//         Self {
//             key: env.key,
//             value: env.value,
//         }
//     }
// }

// impl From<save_template::SaveTemplateSaveTemplateEnv> for EnvironmentVariable {
//     fn from(env: save_template::SaveTemplateSaveTemplateEnv) -> Self {
//         Self {
//             key: env.key,
//             value: env.value,
//         }
//     }
// }

// "advancedStart": false,
// "containerDiskInGb": 987,
// "containerRegistryAuthId": "abc123",
// "dockerArgs": "abc123",
// "earned": 123.45,
// "env": [EnvironmentVariable],
// "id": "abc123",
// "imageName": "xyz789",
// "isPublic": true,
// "isRunpod": false,
// "isServerless": true,
// "boundEndpointId": "xyz789",
// "name": "xyz789",
// "ports": "xyz789",
// "readme": "abc123",
// "runtimeInMin": 123,
// "startJupyter": false,
// "startScript": "xyz789",
// "startSsh": false,
// "volumeInGb": 123,
// "volumeMountPath": "abc123",
// "config": {},
// "category": "abc123"

fn display_environment_variable(a: ()) -> String {
    "".to_string()
}

#[cfg_attr(
    feature = "tabled",
    derive(tabled::Tabled),
    tabled(
        display(Option, "display_option", ""),
        // display(Option<Vec<EnvironmentVariable>>, "display_environment_variable"),
    )
)]
#[derive(Debug, Clone)]
pub struct Template {
    // advanced_start: Option<bool>,
    // container_disk_in_gb: i64,
    // container_registry_auth_id: Option<String>,
    docker_args: Option<String>,
    // earned: f64,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    env: Option<Vec<EnvironmentVariable>>,
    id: Option<String>,
    image_name: Option<String>,
    // is_public: bool,
    // is_runpod: bool,
    // is_serverless: bool,
    // bound_endpoint_id: Option<String>,
    // name: String,
    // ports: Option<String>,
    // readme: Option<String>,
    // runtime_in_min: i64,
    // start_jupyter: bool,
    // start_script: Option<String>,
    start_ssh: Option<bool>,
    // volume_in_gb: f64,
    // volume_mount_path: Option<String>,
    // config: serde_json::Value,
    // category: String,
}

impl From<get_templates::GetTemplatesMyselfPodTemplates> for Template {
    fn from(template: get_templates::GetTemplatesMyselfPodTemplates) -> Self {
        Self {
            // advanced_start: template.advanced_start,
            // container_disk_in_gb: template.container_disk_in_gb,
            // container_registry_auth_id: template.container_registry_auth_id,
            docker_args: template.docker_args,
            // earned: template.earned,
            // // env: template.env.map(|v| v.into_iter().map(Into::into).collect()),
            id: template.id,
            image_name: template.image_name,
            // is_public: template.is_public,
            // is_runpod: template.is_runpod,
            // is_serverless: template.is_serverless,
            // bound_endpoint_id: template.bound_endpoint_id,
            // name: template.name,
            // ports: template.ports,
            // readme: template.readme,
            // runtime_in_min: template.runtime_in_min,
            // start_jupyter: template.start_jupyter,
            // start_script: template.start_script,
            start_ssh: template.start_ssh,
            // volume_in_gb: template.volume_in_gb,
            // volume_mount_path: template.volume_mount_path,
            // config: template.config,
            // category: template.category,
            env: template.env.map(|env_vec| {
                env_vec
                    .into_iter()
                    .filter_map(|opt_env| opt_env.map(Into::into))
                    .collect()
            }),
        }
    }
}
