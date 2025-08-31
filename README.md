# htrs

HTTP CLI for calling services across multiple environments

## Contents

- [Quickstack](#quickstart)
- [Services](#services)
- [Environments](#environments)

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
htrs service add <name> [--alias <alias>]
```

Note: Both the name & alias of the service must be unique with any other services that have been created

### Removing Services

Remove _(delete)_ an existing service

Command:
```
htrs service remove <service>
```

Note: `<service>` in the above command can be the name or alias of the service to be removed

### Listing Services

List all currently defined services

Command:
```
htrs service list
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
htrs service environment add <service> <name> <host> [--alias <alias>] [--default]
```

### Removing Environments

Remove _(delete)_ an existing environment from a service

```
htrs service environment remove <service> <environment>
```

Note: The `<service>` and `<environment>` arguments can be the name or alias of the respective service/environment

### List Environments

List all defined environments for a service

```
htrs service environment list <service>
```
