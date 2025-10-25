use crate::config::QueryParameter;

pub fn map_query_param_shorthand(query_param_str: &str) -> QueryParameter {
    match query_param_str.starts_with('*') {
        true => QueryParameter {
            name: query_param_str[1..].to_string(),
            required: true,
        },
        false => QueryParameter {
            name: query_param_str.to_string(),
            required: false
        },
    }
}