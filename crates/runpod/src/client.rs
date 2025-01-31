use crate::config::Config;
use crate::gql::gpu_types::{GpuLowestPriceInput, GpuTypeFilter};
use crate::{
    error::{Result, RunpodError},
    gql::*,
    types::*,
};
use graphql_client::GraphQLQuery;
use graphql_client::Response;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client as ReqwestClient, Url};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, error};

#[derive(Clone)]
pub struct RunpodClient {
    client: ReqwestClient,
    apikey: String,
    apiurl: Url,
}

impl std::fmt::Debug for RunpodClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RunpodClient")
            .field("base_url", &self.apiurl)
            .field(
                "api_key",
                &format!(
                    "{}...{}",
                    &self.apikey[..4],
                    &self.apikey[self.apikey.len().saturating_sub(4)..]
                ),
            )
            .finish()
    }
}

impl RunpodClient {
    pub fn from_config() -> Result<Self> {
        let config = Config::try_from_env()?;
        Ok(Self::new(config))
    }

    pub fn new(Config { apikey, apiurl }: Config) -> Self {
        let client = ReqwestClient::builder()
            .user_agent("graphql-rust/0.10.0")
            .default_headers(HeaderMap::from_iter([(
                HeaderName::from_static("authorization"),
                HeaderValue::from_str(&format!("Bearer {}", apikey))
                    .expect("invalid api key header"),
            )]))
            .build()
            .expect("Failed to create HTTP client");
        Self {
            client,
            apikey,
            apiurl: apiurl.parse().expect("invalid api url in config"),
        }
    }

    async fn request<Req, Res>(&self, params: &Req) -> Result<Res>
    where
        Req: Serialize + std::fmt::Debug,
        Res: DeserializeOwned,
    {
        debug!(
            "Making request to {} with params: {:?}",
            &self.apiurl, params
        );
        let request = self.client.post(self.apiurl.clone()).json(&params);

        let response = request.send().await?;
        let status = response.status();
        let body = response.bytes().await?;

        if !status.is_success() {
            error!("Request failed: {}", status);
            return Err(match status.as_u16() {
                401 => RunpodError::AuthenticationFailed("Invalid API key".into()),
                404 => RunpodError::NotFound("Resource not found".into()),
                429 => RunpodError::RateLimited,
                _ => {
                    error!(body = ?body, "Server returned {}", status);
                    RunpodError::ServerError(format!("Server returned {}", status))
                }
            });
        }

        let jd = &mut serde_json::Deserializer::from_slice(&body);
        let response: Response<Res> = match serde_path_to_error::deserialize(jd) {
            Ok(response) => response,
            Err(err) => {
                let json_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&body);
                error!(
                    "Failed to deserialize response at path {}: {}. Raw response: {}",
                    err.path(),
                    err,
                    json_str
                );
                return Err(RunpodError::DeserializationError(err.into_inner()));
            }
        };

        match response {
            Response {
                data: Some(res), ..
            } => Ok(res),
            Response {
                errors: Some(errors),
                ..
            } => {
                error!("GraphQL errors: {:#?}", errors);
                Err(errors.into_iter().next().unwrap().into())
            }
            _ => {
                error!("Response is missing data");
                Err(RunpodError::ServerError("Response is missing data".into()))
            }
        }
    }

    pub async fn list_pods(&self) -> Result<Vec<Pod>> {
        let variables = myself_query::Variables {};
        let request_body = MyselfQuery::build_query(variables);
        let response: myself_query::ResponseData = self.request(&request_body).await?;
        Ok(response.myself.pods.into_iter().map(Into::into).collect())
    }

    pub async fn stop_pod(&self, pod_id: &str) -> Result<PodStatus> {
        let variables = stop_pod::Variables {
            input: stop_pod::PodStopInput {
                pod_id: pod_id.to_string(),
                increment_version: Some(false),
            },
        };
        let request_body = StopPod::build_query(variables);
        let response: stop_pod::ResponseData = self.request(&request_body).await?;
        let status = response.pod_stop.desired_status;
        if status == stop_pod::PodStatus::EXITED {
            Ok(PodStatus::Terminated)
        } else {
            Err(RunpodError::ServerError(format!(
                "Pod did not stop: {:?}",
                status
            )))
        }
    }

    pub async fn terminate_pod(&self, pod_id: &str) -> Result<()> {
        let variables = terminate_pod::Variables {
            input: terminate_pod::PodTerminateInput {
                pod_id: pod_id.to_string(),
            },
        };
        let request_body = TerminatePod::build_query(variables);
        let _: terminate_pod::ResponseData = self.request(&request_body).await?;
        Ok(())
    }

    pub async fn get_pod(&self, pod_id: &str) -> Result<Option<Pod>> {
        let variables = get_pod::Variables {
            input: get_pod::PodFilter {
                pod_id: pod_id.to_string(),
            },
        };
        let request_body = GetPod::build_query(variables);
        let response: get_pod::ResponseData = self.request(&request_body).await?;
        Ok(response.pod.map(Into::into))
    }

    async fn request_list_gpus(
        &self,
        gpu_type_filter: GpuTypeFilter,
        lowest_price_input: GpuLowestPriceInput,
    ) -> Result<Vec<GpuOffer>> {
        let variables = gpu_types::Variables {
            input: Some(gpu_type_filter),
            lowest_price_input: Some(lowest_price_input),
        };
        let request_body = GpuTypes::build_query(variables);
        let response: gpu_types::ResponseData = self.request(&request_body).await?;
        Ok(response
            .gpu_types
            .into_iter()
            .flatten()
            .map(Into::into)
            .collect())
    }

    pub async fn list_gpus(&self, secure_cloud: Option<bool>) -> Result<Vec<GpuOffer>> {
        match secure_cloud {
            Some(_) => {
                let gpu_type_filter = gpu_types::GpuTypeFilter {
                    ..Default::default()
                };
                let lowest_price_input = gpu_types::GpuLowestPriceInput {
                    secure_cloud,
                    ..Default::default()
                };
                self.request_list_gpus(gpu_type_filter, lowest_price_input)
                    .await
            }
            None => {
                let gpu_type_filter = gpu_types::GpuTypeFilter {
                    ..Default::default()
                };
                let secure_gpus = self
                    .request_list_gpus(
                        gpu_type_filter.clone(),
                        gpu_types::GpuLowestPriceInput {
                            secure_cloud: Some(true),
                            ..Default::default()
                        },
                    )
                    .await?;
                let community_gpus = self
                    .request_list_gpus(
                        gpu_type_filter,
                        gpu_types::GpuLowestPriceInput {
                            secure_cloud: Some(false),
                            ..Default::default()
                        },
                    )
                    .await?;
                Ok(aggregate_secure_community(secure_gpus, community_gpus))
            }
        }
    }

    pub async fn spawn_pod(
        &self,
        name: String,
        gpu_type_id: String,
        gpu_count: i64,
        spot: bool,
        bid_per_gpu: Option<f64>,
        container_disk_in_gb: Option<i64>,
    ) -> Result<String> {
        if spot {
            let variables = spawn_pod_spot::Variables {
                input: spawn_pod_spot::PodRentInterruptableInput {
                    name: Some(name),
                    gpu_type_id: Some(gpu_type_id),
                    gpu_count: Some(gpu_count),
                    bid_per_gpu,
                    container_disk_in_gb,
                    ..Default::default()
                },
            };
            let request_body = SpawnPodSpot::build_query(variables);
            let response: spawn_pod_spot::ResponseData = self.request(&request_body).await?;
            let id = response
                .pod_rent_interruptable
                .ok_or(RunpodError::GraphQLError("Pod not created".to_string()))?
                .id;
            Ok(id)
        } else {
            let variables = spawn_pod_on_demand::Variables {
                input: spawn_pod_on_demand::PodFindAndDeployOnDemandInput {
                    name: Some(name),
                    gpu_type_id: Some(gpu_type_id),
                    gpu_count: Some(gpu_count),
                    container_disk_in_gb,
                    ..Default::default()
                },
            };
            let request_body = SpawnPodOnDemand::build_query(variables);
            let response: spawn_pod_on_demand::ResponseData = self.request(&request_body).await?;
            let id = response
                .pod_find_and_deploy_on_demand
                .ok_or(RunpodError::GraphQLError("Pod not created".to_string()))?
                .id;
            Ok(id)
        }
    }
}

fn min_option<T: PartialOrd>(a: Option<T>, b: Option<T>) -> Option<T> {
    match (a, b) {
        (Some(x), Some(y)) => Some(if x <= y { x } else { y }),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

fn add_option<T: std::ops::Add<Output = T>>(a: Option<T>, b: Option<T>) -> Option<T> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x + y),
        (Some(x), None) => Some(x),
        (None, Some(y)) => Some(y),
        (None, None) => None,
    }
}

fn combine_lowest_price(secure: LowestPrice, community: LowestPrice) -> Result<LowestPrice> {
    if secure.gpu_name != community.gpu_name {
        return Err(RunpodError::InvalidInput(
            "GPU names do not match".to_string(),
        ));
    }
    if secure.gpu_type_id != community.gpu_type_id {
        return Err(RunpodError::InvalidInput(
            "GPU types do not match".to_string(),
        ));
    }
    let uninterruptable_price = min_option(
        secure.uninterruptable_price,
        community.uninterruptable_price,
    );
    let minimum_bid_price = min_option(secure.minimum_bid_price, community.minimum_bid_price);
    let min_memory = min_option(secure.min_memory, community.min_memory);
    let min_vcpu = min_option(secure.min_vcpu, community.min_vcpu);
    let rental_percentage = min_option(secure.rental_percentage, community.rental_percentage);
    let rented_count = add_option(secure.rented_count, community.rented_count);
    let total_count = add_option(secure.total_count, community.total_count);
    let stock_status = min_option(secure.stock_status, community.stock_status);
    let min_download = min_option(secure.min_download, community.min_download);
    let min_disk = min_option(secure.min_disk, community.min_disk);
    let min_upload = min_option(secure.min_upload, community.min_upload);
    let support_public_ip = secure.support_public_ip.or(community.support_public_ip);
    let country_code = secure.country_code;
    Ok(LowestPrice {
        gpu_name: secure.gpu_name,
        gpu_type_id: secure.gpu_type_id,
        minimum_bid_price,
        uninterruptable_price,
        min_memory,
        min_vcpu,
        rental_percentage,
        rented_count,
        total_count,
        stock_status,
        min_download,
        min_disk,
        min_upload,
        country_code,
        support_public_ip,
        compliance: secure.compliance,
    })
}

fn aggregate_secure_community(secure: Vec<GpuOffer>, community: Vec<GpuOffer>) -> Vec<GpuOffer> {
    let mut seen_ids = std::collections::HashSet::new();
    let mut result = Vec::with_capacity(secure.len() + community.len());

    for secure_gpu in secure {
        seen_ids.insert(secure_gpu.id.clone());
        result.push(secure_gpu);
    }

    for community_gpu in community {
        if seen_ids.contains(&community_gpu.id) {
            if let Some(secure_gpu) = result.iter_mut().find(|g| g.id == community_gpu.id) {
                secure_gpu.lowest_price =
                    match (secure_gpu.lowest_price.take(), community_gpu.lowest_price) {
                        (Some(secure_price), Some(community_price)) => Some(
                            combine_lowest_price(secure_price, community_price)
                                .expect("Failed to combine lowest prices"),
                        ),
                        (None, community_price) => community_price,
                        (secure_price, None) => secure_price,
                    };

                secure_gpu.manufacturer = secure_gpu
                    .manufacturer
                    .take()
                    .or(community_gpu.manufacturer);
                secure_gpu.memory_in_gb = secure_gpu.memory_in_gb.or(community_gpu.memory_in_gb);
                secure_gpu.community_price =
                    secure_gpu.community_price.or(community_gpu.community_price);
                secure_gpu.community_spot_price = secure_gpu
                    .community_spot_price
                    .or(community_gpu.community_spot_price);
                secure_gpu.secure_price = secure_gpu.secure_price.or(community_gpu.secure_price);
                secure_gpu.secure_spot_price = secure_gpu
                    .secure_spot_price
                    .or(community_gpu.secure_spot_price);
            }
        } else {
            seen_ids.insert(community_gpu.id.clone());
            result.push(community_gpu);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> RunpodClient {
        RunpodClient::from_config().expect("no runpod login")
    }

    #[ignore]
    #[test_log::test(tokio::test)]
    async fn test_list_pods() {
        let client = setup();
        let pods = client.list_pods().await.expect("failed to list pods");
        println!("Found {} pods", pods.len());
        for pod in pods {
            println!(
                "Pod {} ({}): {} GPU(s), {} vCPU(s), {} GB RAM",
                pod.name, pod.id, pod.gpu_count, pod.vcpu_count, pod.memory_in_gb
            );
        }
    }
}
