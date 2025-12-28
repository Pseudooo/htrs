use crate::config::current_config::{Endpoint, Environment, HtrsConfig, Preset, QueryParameter, Service};
use crate::config::versioned_config::versions::v1::v1config::{EndpointV1, EnvironmentV1, HtrsConfigV1, PresetV1, QueryParameterV1, ServiceV1};

pub fn migrate_v1_to_v2(v1_config: HtrsConfigV1) -> HtrsConfig {
    HtrsConfig {
        services: v1_config.services.into_iter()
            .map(migrate_v1_service)
            .collect(),
        presets: v1_config.presets.into_iter()
            .map(migrate_v1_preset)
            .collect(),
        headers: v1_config.headers,
    }
}

fn migrate_v1_service(service: ServiceV1) -> Service {
    Service {
        name: service.name,
        alias: service.alias,
        environments: service.environments.into_iter()
            .map(migrate_v1_environment)
            .collect(),
        endpoints: service.endpoints.into_iter()
            .map(migrate_v1_endpoint)
            .collect(),
        headers: service.headers,
    }
}

fn migrate_v1_environment(environment: EnvironmentV1) -> Environment {
    Environment {
        name: environment.name,
        alias: environment.alias,
        host: environment.host,
        default: environment.default,
        headers: environment.headers,
    }
}

fn migrate_v1_endpoint(endpoint: EndpointV1) -> Endpoint {
    Endpoint {
        name: endpoint.name,
        path_template: endpoint.path_template,
        query_parameters: endpoint.query_parameters.into_iter()
            .map(migrate_v1_query_parameter)
            .collect(),
    }
}

fn migrate_v1_query_parameter(parameter: QueryParameterV1) -> QueryParameter {
    QueryParameter {
        name: parameter.name,
        required: parameter.required,
    }
}

fn migrate_v1_preset(preset: PresetV1) -> Preset {
    Preset {
        name: preset.name,
        alias: None,
        values: preset.values,
    }
}