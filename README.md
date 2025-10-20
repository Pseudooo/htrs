# htrs

HTTP CLI for calling services across multiple environments

## Contents

- [Quickstack](#quickstart)
- [Services](#services)
- [Environments](#environments)
- [Endpoints](#endpoints)
- [Calling a Service](#calling-a-service)
- [Headers](#headers)

## Quickstart

Following example will be using the AviationStack API

### Define a service

A service represents an api you want to call across multiple environments

Command:
```
> htrs new service aviation
```

### Define an environment

An environment is an instance of the service with an associated host

Command:
```
> htrs new environment prod api.aviationstack.com --service aviation --default
```

### Define an endpoint

An endpoint is consistent across all environments in a service with only the host varying

Command:
```
> htrs new endpoint get-flights /v1/flights --service aviation
```

### Call the endpoint

We've specified our environment `prod` as the default so we don't need to specify it, but we can specifiy specific environments with the `--environment` option

Command:
```
> htrs call aviation get-flights
401 Unauthorized | GET | https://api.aviationstack.com/v1/flights
{
  "error": {
    "code": "missing_access_key",
    "message": "You have not supplied an API Access Key. [Required format: access_key=YOUR_ACCESS_KEY]"
  }
}
```

For more details on parameterising the endpoints see [Creating Endpoint](#creating-endpoints) & [Calling a Service](#calling-a-service)

## Commands

Most of the commands follow the same structure of:
```
htrs <new|edit|delete|set|clear> ...
```

Which can be applied to the respective items that can be configured by the cli

## Services

A service represents an api that is hosted across multiple environments, such as most companies typically have hosts for:
- production
- staging
- development

A service will encapsulate all of the above.

### Creating A Service

New services can be created with a unique name & _(optional)_ alias

```
Create a new service

Usage: htrs.exe new service [OPTIONS] <name>

Arguments:
  <name>  The unique name of the service to create

Options:
  -a, --alias <alias>  The unique alias for the new service
  -h, --help           Print help
```

Note: Both the name & alias of the service must be unique with any other services that have been created

### Removing Services

Remove an existing service

Command:
```
Delete an existing service from config

Usage: htrs.exe delete service <name>

Arguments:
  <name>  The name or alias of the service to delete

Options:
  -h, --help  Print help
```

### Listing Services

List all currently defined services

```
List all services

Usage: htrs.exe list service [OPTIONS]

Options:
  -f, --filter <filter>  Filter for service name or alias
  -h, --help             Print help
```

## Environments

An environment is a hosted instance of a service, a service in each environment will share the same endpoint(s) but have different
hosts

### Creating Environments

An environment is defined under a service, so there must be an existing service in order to create an environment

```
Usage: htrs.exe new environment [OPTIONS] --service <service> <name> <host>

Arguments:
  <name>  The unique name for the new environment
  <host>  The host for the environment

Options:
      --default            Flag to determine if the new environment should be the default
  -a, --alias <alias>      The unique alias for the new environment
  -s, --service <service>  The service that the environment will be created for
  -h, --help               Print help
```

### Removing Environments

Remove an existing environment from a service

```
Delete an existing environment from config

Usage: htrs.exe delete environment --service <service> <name>

Arguments:
  <name>  The name or alias of the environment to delete

Options:
  -s, --service <service>  The service name or alias that environment is defined in
  -h, --help               Print help
```

### List Environments

List all defined environments for a service

```
List environments for a service

Usage: htrs.exe list environment [OPTIONS] --service <service>

Options:
  -s, --service <service>  Service to list environments for
  -f, --filter <filter>    Filter for environment name or alias
  -h, --help               Print help
```

## Endpoints

### Creating Endpoints

Create an endpoint for a service

```
Usage: htrs.exe new endpoint [OPTIONS] --service <service> <name> <path>

Arguments:
  <name>  Name of the endpoint to create
  <path>  The path of the endpoint

Options:
  -q, --query <query>      Query parameter for endpoint
  -s, --service <service>  The service endpoint will be created for
  -h, --help               Print help
```

Within the path variables can be declared using `{}`

If the path template `/my/{variable}/path` is used then a parameter `variable` will be used which will be a required
argument when calling the endpoint

### Removing Endpoints

Remove an endpoint from a service

```
Delete an existing endpoint from config

Usage: htrs.exe delete endpoint --service <service> <name>

Arguments:
  <name>  The name of the endpoint

Options:
  -s, --service <service>  The service name or alias that the endpoint is defined for
  -h, --help               Print help
```

### List Endpoints

List all endpoints for a service

```
List endpoints for a service

Usage: htrs.exe list endpoint [OPTIONS] --service <service>

Options:
  -s, --service <service>  Service to list environments for
  -f, --filter <filter>    Filter for endpoint name
  -h, --help               Print help
```

## Calling a Service

Calling a service requires:

1. A service has been defined
2. An environment has been defined for that service
3. An endpoint has been defined for that service
   1. The endpoint will determine what parameters are required for calling the endpoint - see [Creating Endpoints](#creating-endpoints)

The above configuration will determine what commands are available to be called, if a service has been created `"example"`
with an endpoint `"endpoint"` with path=`/my/{path}/path` and a single query parameter `"query"` then the help menu will
show the following for `htrs call --help`
```
Usage: htrs.exe call [OPTIONS] [COMMAND]

Commands:
  example  
  help     Print this message or the help of the given subcommand(s)

Options:
  -e, --environment <environment name>  Environment to target, will use default environment if none specified
  -h, --help                            Print help
```

And for the endpoint will show:
```
Usage: htrs.exe call example endpoint [OPTIONS] --path <path> --query <query>

Options:
  -e, --environment <environment name>  Environment to target, will use default environment if none specified
  -q, --query-param <query param>       Set a query parameter for the request in the format `name=value`
      --body                            Print the response body
      --path <path>                     
      --query <query>                   
  -h, --help                            Print help
```

The `-q` or `--query-param` argument can be used to provide additional query parameters that aren't included in the template.
If a query parameter that's provided with this argument has the same name as any defined in the endpoint it will override
the value provided directly from the endpoint's corresponding argument.


## Headers

Headers can be defined to be added to requests at the following scopes:
- global
- service
- environment

If a header is present in two scopes then their precedence will follow environment > service > global

Meaning if the same header is defined for an environment & the global scope, then when calling the given environment it will
override the value from the global scope.

```
Set a header for a service or environment

Usage: htrs.exe set header [OPTIONS] <name> <value>

Arguments:
  <name>   The header name to set
  <value>  The header value to set

Options:
  -s, --service <service>          Service to target
  -e, --environment <environment>  Environment to target
  -h, --help                       Print help
```
