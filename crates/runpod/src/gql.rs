use graphql_client::GraphQLQuery;

pub type DateTime = String;
pub type Port = u16;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.graphql",
    query_path = "gql/myself_query.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug, Clone"
)]
pub struct MyselfQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.graphql",
    query_path = "gql/stop_pod.graphql",
    response_derives = "Debug, Clone, PartialEq",
    variables_derives = "Debug, Clone"
)]
pub struct StopPod;

type Void = ();
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.graphql",
    query_path = "gql/terminate_pod.graphql",
    variables_derives = "Debug"
)]
pub struct TerminatePod;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.graphql",
    query_path = "gql/get_pod.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug, Clone"
)]
pub struct GetPod;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.graphql",
    query_path = "gql/gpu_types.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug, Default, Clone"
)]
pub struct GpuTypes;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.graphql",
    query_path = "gql/spawn_pod.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug, Default"
)]
pub struct SpawnPodOnDemand;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "gql/schema.graphql",
    query_path = "gql/spawn_pod.graphql",
    response_derives = "Debug, Clone",
    variables_derives = "Debug, Default"
)]
pub struct SpawnPodSpot;
