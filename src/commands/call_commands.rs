use crate::command_args::CallServiceOptions;
use crate::config::{HtrsConfig, ServiceEnvironmentConfig};
use crate::outcomes::HtrsAction::MakeRequest;
use crate::outcomes::{HtrsAction, HtrsError};
use reqwest::{Method, Url};
use std::collections::HashMap;
use std::str::FromStr;

pub fn execute_call_command(config: &HtrsConfig, cmd: CallServiceOptions) -> Result<HtrsAction, HtrsError> {
    let service = match config.find_service_config(&cmd.service) {
        Some(service) => service,
        None => return Err(HtrsError::new(&format!("Service {} does not exist", cmd.service))),
    };

    let environment: &ServiceEnvironmentConfig;
    if let Some(environment_name) = cmd.environment {
        environment = match service.find_environment(&environment_name) {
            Some(environment) => environment,
            None => return Err(HtrsError::new(&format!("Environment {} does not exist", environment_name))),
        }
    } else if let Some(default_environment) = service.find_default_environment() {
        environment = default_environment;
    } else {
        return Err(HtrsError::new(&format!("No default environment defined for {}", cmd.service)));
    }

    let path = cmd.path;
    let query = cmd.query;
    let display_options = cmd.display_options;

    let mut method = Method::GET;
    if let Some(method_str) = cmd.method {
        method = match Method::from_str(&method_str.to_uppercase()) {
            Ok(method) => method,
            Err(_) => return Err(HtrsError::new(&format!("Invalid method: {}", method_str))),
        }
    }

    let mut query_values = Vec::new();
    for q in query {
        match q.split("=").collect::<Vec<&str>>().as_slice() {
            [key, value] if key.len() > 0 && value.len() > 0 => query_values.push((key.to_string(), value.to_string())),
            _ => return Err(HtrsError::new(&format!("Invalid query: {}", q))),
        }
    }

    let url = build_url(&environment.host, path, query_values)?;
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("User-Agent".to_string(), format!("htrs/{}", env!("CARGO_PKG_VERSION")));
    for (key, value) in &config.headers {
        headers.insert(key.clone(), value.clone());
    }
    for (key, value) in &service.headers {
        headers.insert(key.clone(), value.clone());
    }

    for kvp in cmd.header {
        match kvp.split("=").collect::<Vec<&str>>().as_slice() {
            [key, value] => {


                headers.insert(key.to_string(), value.to_string());
            }
            _ => return Err(HtrsError::new(&format!("Invalid header value {}", kvp))),
        };
    }

    let action = MakeRequest {
        url, headers, method, display_options,
    };
    Ok(action)
}

fn build_url(host: &str, path: Option<String>, query: Vec<(String, String)>) -> Result<Url, HtrsError> {
    let mut url = match Url::parse(&format!("https://{host}")) {
        Ok(uri) => uri,
        Err(e) => return Err(HtrsError::new(&e.to_string())),
    };

    url = match path {
        Some(path) => match url.join(&path) {
            Ok(uri) => uri,
            Err(e) => return Err(HtrsError::new(&e.to_string())),
        },
        None => url,
    };

    if query.len() > 0 {
        let query_string = query.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&");
        url.set_query(Some(&query_string));
    }

    Ok(url)
}

#[cfg(test)]
mod call_command_tests {
    use super::*;
    use crate::command_args::CallOutputOptions;
    use crate::config::ServiceConfig;
    use rstest::rstest;

    struct CallServiceOptionsBuilder {
        service: Option<String>,
        environment: Option<String>,
        path: Option<String>,
        query: Vec<String>,
        header: Vec<String>,
        method: Option<String>,
    }

    impl CallServiceOptions {
        fn build() -> CallServiceOptionsBuilder {
            CallServiceOptionsBuilder {
                service: None,
                environment: None,
                path: None,
                query: vec![],
                header: vec![],
                method: None,
            }
        }
    }

    impl CallServiceOptionsBuilder {
        fn service(&mut self, service: &str) -> &mut CallServiceOptionsBuilder {
            self.service = Some(service.to_string());
            self
        }

        fn environment(&mut self, environment: &str) -> &mut CallServiceOptionsBuilder {
            self.environment = Some(environment.to_string());
            self
        }

        fn path(&mut self, path: &str) -> &mut CallServiceOptionsBuilder {
            self.path = Some(path.to_string());
            self
        }

        fn query_pair(&mut self, key: &str, value: &str) -> &mut CallServiceOptionsBuilder {
            self.query.push(format!("{}={}", key, value));
            self
        }

        fn query(&mut self, query: &str) -> &mut CallServiceOptionsBuilder {
            self.query.push(query.to_string());
            self
        }

        fn header(&mut self, header: &str) -> &mut CallServiceOptionsBuilder {
            self.header.push(header.to_string());
            self
        }

        fn build(&self) -> CallServiceOptions {
            CallServiceOptions {
                service: self.service.clone().unwrap(),
                environment: self.environment.clone(),
                path: self.path.clone(),
                query: self.query.clone(),
                header: self.header.clone(),
                method: self.method.clone(),
                display_options: CallOutputOptions {
                    hide_url: false,
                    hide_request_headers: false,
                    hide_response_status: false,
                    hide_response_headers: false,
                    hide_response_body: false,
                }
            }
        }
    }

    #[test]
    fn given_unknown_service_when_call_service_then_error_returned() {
        // Arrange
        let mut service = ServiceConfig::new("something".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "foo".to_string(),
            "bar.com".to_string(),
            true,
        ));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("bar")
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Err(_)))
    }

    #[test]
    fn given_known_service_with_no_environments_when_call_service_then_error_returned() {
        // Arrange
        let mut config = HtrsConfig::new();
        config.services.push(ServiceConfig::new("something".to_string()));
        let command = CallServiceOptions::build()
            .service("something")
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn given_known_service_with_no_default_environment_when_call_service_then_error_returned() {
        // Arrange
        let mut service = ServiceConfig::new("something".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "foo".to_string(),
            "bar.com".to_string(),
            false,
        ));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("something")
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn given_known_service_with_default_environment_when_call_service_then_result_returned() {
        // Arrange
        let mut service = ServiceConfig::new("google".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "dev".to_string(),
            "google.com".to_string(),
            true,
        ));
        service.environments.push(ServiceEnvironmentConfig::new(
            "staging".to_string(),
            "not-google.com".to_string(),
            false,
        ));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("google")
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Ok(_)));
        let MakeRequest { url, method, .. } = result.unwrap() else {
            panic!("Returned action was not MakeRequest");
        };
        assert_eq!(url.host().unwrap().to_string(), "google.com");
        assert_eq!(method, "GET");
    }

    #[test]
    fn given_known_service_with_unknown_environment_when_call_then_error_returned() {
        // Arrange
        let mut service = ServiceConfig::new("something".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "foo".to_string(),
            "foo.com".to_string(),
            false,
        ));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("something")
            .environment("something")
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn given_known_service_with_known_environment_when_call_then_result_returned() {
        // Arrange
        let mut service = ServiceConfig::new("something".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "foo".to_string(),
            "foo.com".to_string(),
            false,
        ));
        service.environments.push(ServiceEnvironmentConfig::new(
            "bar".to_string(),
            "bar.com".to_string(),
            false,
        ));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("something")
            .environment("foo")
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Ok(_)));
        let MakeRequest { url, method, .. } = result.unwrap() else {
            panic!("Returned action was not MakeRequest");
        };
        assert_eq!(url.host().unwrap().to_string(), "foo.com");
        assert_eq!(method, "GET");
    }

    #[test]
    fn given_known_service_with_default_environment_when_call_with_path_then_result_returned() {
        // Arrange
        let mut service = ServiceConfig::new("something".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "foo".to_string(),
            "foo.com".to_string(),
            true,
        ));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("something")
            .path("/my/path")
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Ok(_)));
        let MakeRequest { url, .. } = result.unwrap() else {
            panic!("Returned action was not MakeRequest");
        };
        assert_eq!(url.host().unwrap().to_string(), "foo.com");
        assert_eq!(url.path(), "/my/path");
    }

    #[rstest]
    #[case("")]
    #[case("foo")]
    #[case("a=")]
    #[case("=a")]
    #[case("=")]
    fn given_known_service_with_default_environment_when_call_with_invalid_query_values_then_error_returned(#[case] query: &str) {
        // Arrange
        let mut service = ServiceConfig::new("something".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "foo".to_string(),
            "foo.com".to_string(),
            true,
        ));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("something")
            .query(query)
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn given_known_service_with_default_environment_when_call_with_query_values_then_result_returned() {
        // Arrange
        let mut service = ServiceConfig::new("something".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "foo".to_string(),
            "foo.com".to_string(),
            true,
        ));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("something")
            .query_pair("fieldA", "valueA")
            .query_pair("fieldB", "valueB")
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Ok(_)));
        let MakeRequest { url, .. } = result.unwrap() else {
            panic!("Returned action was not MakeRequest");
        };
        assert_eq!(url.host().unwrap().to_string(), "foo.com");
        assert!(matches!(url.query(), Some("fieldA=valueA&fieldB=valueB")));
    }

    #[rstest]
    #[case("")]
    #[case("foo")]
    #[case("a=")]
    #[case("=a")]
    #[case("=")]
    fn given_known_service_with_default_environment_when_call_with_invalid_headers_then_error_returned(#[case] header: &str) {
        // Arrange
        let mut service = ServiceConfig::new("something".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "foo".to_string(),
            "foo.com".to_string(),
            true,
        ));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("foo")
            .header(header)
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn given_known_service_with_default_environment_when_call_with_headers_then_result_returned() {
        // Arrange
        let mut service = ServiceConfig::new("something".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "foo".to_string(),
            "foo.com".to_string(),
            true,
        ));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("something")
            .header("foo=bar")
            .header("kek=lol")
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Ok(_)));
        let MakeRequest { headers, .. } = result.unwrap() else {
            panic!("Returned action was not MakeRequest");
        };
        let mut expected_headers = Vec::new();
        expected_headers.push(("foo", "bar"));
        expected_headers.push(("kek", "lol"));
        for (key, value) in expected_headers {
            let header_value = headers.get(key);
            assert!(matches!(header_value, Some(_)));
            assert_eq!(value, header_value.unwrap())
        }
    }

    #[test]
    fn given_known_header_configured_and_known_service_with_default_environment_when_call_then_result_returned_with_headers() {
        // Arrange
        let mut service = ServiceConfig::new("something".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "foo".to_string(),
            "foo.com".to_string(),
            true,
        ));
        let mut config = HtrsConfig::new();
        config.headers.insert("foo".to_string(), "bar".to_string());
        config.headers.insert("kek".to_string(), "lol".to_string());
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("something")
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Ok(_)));
        let MakeRequest { headers, .. } = result.unwrap() else {
            panic!("Returned action was not MakeRequest");
        };
        let mut expected_headers = Vec::new();
        expected_headers.push(("foo", "bar"));
        expected_headers.push(("kek", "lol"));
        for (key, value) in expected_headers {
            let header_value = headers.get(key);
            assert!(matches!(header_value, Some(_)));
            assert_eq!(value, header_value.unwrap())
        }
    }

    #[test]
    fn given_known_service_with_default_environment_and_headers_configured_when_call_then_result_returned_with_headers() {
        // Arrange
        let mut service = ServiceConfig::new("something".to_string());
        service.headers.insert("foo".to_string(), "bar".to_string());
        service.headers.insert("kek".to_string(), "lol".to_string());
        service.environments.push(ServiceEnvironmentConfig::new(
            "foo".to_string(),
            "foo.com".to_string(),
            true,
        ));
        let mut config = HtrsConfig::new();
        config.services.push(service);
        let command = CallServiceOptions::build()
            .service("something")
            .build();

        // Act
        let result = execute_call_command(&config, command);

        // Assert
        assert!(matches!(result, Ok(_)));
        let MakeRequest { headers, .. } = result.unwrap() else {
            panic!("Returned action was not MakeRequest");
        };
        let mut expected_headers = Vec::new();
        expected_headers.push(("foo", "bar"));
        expected_headers.push(("kek", "lol"));
        for (key, value) in expected_headers {
            let header_value = headers.get(key);
            assert!(matches!(header_value, Some(_)));
            assert_eq!(value, header_value.unwrap())
        }
    }
}