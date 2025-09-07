# htrs

HTTP CLI for calling services across multiple environments

## Contents

- [Quickstack](#quickstart)
- [Services](#services)
- [Environments](#environments)
- [Endpoints](#endpoints)
- [Headers](#headers)

## Quickstart

Following example will be using the AviationStack API

### Define a service

A service represents an api you want to call across multiple environments

Command:
```
> htrs service add aviation
```

### Define an environment

An environment is an instance of the service with a host

Command:
```
> htrs service environment add aviation prod api.aviationstack.com --default
```

### Define an endpoint

An endpoint is consistent across all environments in a service with only the host varying

Command:
```
> htrs service endpoint aviation add get-flights /v1/flights
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

## Services

A service represents an api that might exist across multiple environments

To see full detail about service commands run:
```
htrs service --help
```

### Creating Services

Create a new service with a unique name, and optionally a unique alias. The name or alias can be used to reference the service in the future

Command:
```
Usage: htrs.exe service add [OPTIONS] <name>

Arguments:
  <name>  Unique name of the service to create

Options:
  -a, --alias <alias>  Unique alias for the service
  -h, --help           Print help
```

Note: Both the name & alias of the service must be unique with any other services that have been created

### Removing Services

Remove an existing service

Command:
```
Usage: htrs.exe service remove <name>

Arguments:
  <name>  Service name or alias to remove

Options:
  -h, --help  Print help
```

Note: `<service>` in the above command can be the name or alias of the service to be removed

### Listing Services

List all currently defined services

Command:
```
Usage: htrs.exe service list

Options:
  -h, --help  Print help
```

## Environments

An environment is an instance of a service, companies typically have the same service hosted in QA/Staging/Production environments
these environments can be created each with their own host but sharing the same endpoints.

To see full detail about service environment commands run:
```
htrs service environment --help
```

### Creating Environments

Create a new environment under an existing service, with a unique name and optionally a unique alias. The name or alias can
be used to reference the environment in the future.

```
Usage: htrs.exe service environment add [OPTIONS] <service name> <environment name> <host>

Arguments:
  <service name>      The name or alias of the service
  <environment name>  Unique name of the environment to create
  <host>              Hostname for the service in the environment

Options:
  -a, --alias <alias>  Alias for the environment
      --default        Set as the default environment for the service
  -h, --help           Print help
```

### Removing Environments

Remove an existing environment from a service

```
Usage: htrs.exe service environment remove <service name> <environment name>

Arguments:
  <service name>      The name or alias of the service
  <environment name>  The environment name or alias to remove

Options:
  -h, --help  Print help
```

Note: The `<service>` and `<environment>` arguments can use the name or alias of the respective service/environment

### List Environments

List all defined environments for a service

```
Usage: htrs.exe service environment list <service name>

Arguments:
  <service name>  The name or alias of the service

Options:
  -h, --help  Print help
```

## Endpoints

### Creating Endpoints

Create an endpoint for a service

```
Usage: htrs.exe service endpoint <service name> add [OPTIONS] <endpoint name> <path template>

Arguments:
  <endpoint name>  The unique endpoint name
  <path template>  The templated path of endpoint

Options:
  -q, --query-param <query_parameters>  Query parameter for endpoint
  -h, --help                            Print help
```

Within the path template for the url variables can be declared using `{}`

If the path template `/my/{variable}/path` is used then a parameter `variable` will be used which will be a required
argument when calling the endpoint

Similarly, query parameters can be provided using the `--query-param` argument, these will also be required when calling
the created endpoint

### Removing Endpoints

Remove an endpoint from a service

```
Usage: htrs.exe service endpoint <service name> remove <endpoint name>

Arguments:
  <endpoint name>  The endpoint name to remove

Options:
  -h, --help  Print help
```

### List Endpoints

List all endpoints for a service

```
Usage: htrs.exe service endpoint <service name> list

Options:
  -h, --help  Print help
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
Usage: htrs.exe call [environment name] [COMMAND]

Commands:
  example  
  help     Print this message or the help of the given subcommand(s)

Arguments:
  [environment name]  Environment to target, will use default environment if none specified

Options:
  -h, --help  Print help
```

And for the endpoint will show:
```
Usage: htrs.exe call example endpoint --path <path> --query <query>

Options:
      --path <path>    
      --query <query>  
  -h, --help           Print help
```

## Headers

Headers can be applied to all requests globally or for a given service

### Global Headers

By default, htrs will only set the `User-Agent` header to `htrs/{version}` 

Set a header to be applied globally for any requests made

Headers can be set for a new header, or overwrite an existing header:
```
Usage: htrs.exe header set <header name> <header value>

Arguments:
  <header name>   The header name
  <header value>  The header value

Options:
  -h, --help  Print help
```

Or cleared:
```
Usage: htrs.exe header clear <header name>

Arguments:
  <header name>  The header name

Options:
  -h, --help  Print help
```

### Service Headers

Set a header for a specific service for any requests made to that service

If the same header is set globally, the service-level header will overwrite it

Headers can be set:
```
Usage: htrs.exe service header <service> set <header name> <header value>

Arguments:
  <header name>   The header name
  <header value>  The header value

Options:
  -h, --help  Print help
```

Or cleared:
```
Usage: htrs.exe service header <service> clear <header name>

Arguments:
  <header name>  The header name

Options:
  -h, --help  Print help
```
