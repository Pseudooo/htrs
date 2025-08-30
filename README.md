# htrs

HTTP CLI for calling services across multiple environments conveniently

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
200 OK | GET | https://api.aviationstack.com//v1/flights
{
  "success": false,
  "error": {
    "code": 101,
    "type": "missing_access_key",
    "info": "You have not supplied an API Access Key. [Required format: access_key=YOUR_ACCESS_KEY]"
  }
}
```