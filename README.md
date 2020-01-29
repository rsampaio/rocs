# Rust API client for openapi

The Open Service Broker API defines an HTTP(S) interface between Platforms and Service Brokers.

## Overview

For more information, please visit [https://www.openservicebrokerapi.org/](https://www.openservicebrokerapi.org/)

## Installation

### Using `cargo`

`cargo install rocs`

### Manually

```SHELL
git clone https://github.com/rsampaio/rocs.git && cd rocs
cargo install
```

## Usage

```SHELL
rocs 0.1
Rodrigo Vaz
Rust OSB Client 'Super'

USAGE:
    rocs [FLAGS] --password <broker_pass> --broker <broker_url> --username <broker_user> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
        --json       Prints result in JSON format
    -V, --version    Prints version information

OPTIONS:
    -a, --password <broker_pass>     [env: ROCS_BROKER_PASSWORD=]
    -b, --broker <broker_url>        [env: ROCS_BROKER_URL=]
    -u, --username <broker_user>     [env: ROCS_BROKER_USERNAME=]

SUBCOMMANDS:
    bind           Service Binding request
    catalog        Catalog request
    credentials    Binding credentials
    deprovision    Service Instance deprovisioning
    help           Prints this message or the help of the given subcommand(s)
    provision      Service Instance provisioning
    unbind         Service Binding removal

```

## Documentation for API Endpoints

All URIs are relative to *http://example.com*

| Class                 | Method                                                                                                     | HTTP request                                                                             | Description                                                 |
| --------------------- | ---------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| *CatalogApi*          | [**catalog_get**](docs/CatalogApi.md#catalog_get)                                                          | **get** /v2/catalog                                                                      | get the catalog of services that the service broker offers  |
| *ServiceBindingsApi*  | [**service_binding_binding**](docs/ServiceBindingsApi.md#service_binding_binding)                          | **put** /v2/service_instances/{instance_id}/service_bindings/{binding_id}                | generate a service binding                                  |
| *ServiceBindingsApi*  | [**service_binding_get**](docs/ServiceBindingsApi.md#service_binding_get)                                  | **get** /v2/service_instances/{instance_id}/service_bindings/{binding_id}                | get a service binding                                       |
| *ServiceBindingsApi*  | [**service_binding_last_operation_get**](docs/ServiceBindingsApi.md#service_binding_last_operation_get)    | **get** /v2/service_instances/{instance_id}/service_bindings/{binding_id}/last_operation | get the last requested operation state for service binding  |
| *ServiceBindingsApi*  | [**service_binding_unbinding**](docs/ServiceBindingsApi.md#service_binding_unbinding)                      | **delete** /v2/service_instances/{instance_id}/service_bindings/{binding_id}             | deprovision a service binding                               |
| *ServiceInstancesApi* | [**service_instance_deprovision**](docs/ServiceInstancesApi.md#service_instance_deprovision)               | **delete** /v2/service_instances/{instance_id}                                           | deprovision a service instance                              |
| *ServiceInstancesApi* | [**service_instance_get**](docs/ServiceInstancesApi.md#service_instance_get)                               | **get** /v2/service_instances/{instance_id}                                              | get a service instance                                      |
| *ServiceInstancesApi* | [**service_instance_last_operation_get**](docs/ServiceInstancesApi.md#service_instance_last_operation_get) | **get** /v2/service_instances/{instance_id}/last_operation                               | get the last requested operation state for service instance |
| *ServiceInstancesApi* | [**service_instance_provision**](docs/ServiceInstancesApi.md#service_instance_provision)                   | **put** /v2/service_instances/{instance_id}                                              | provision a service instance                                |
| *ServiceInstancesApi* | [**service_instance_update**](docs/ServiceInstancesApi.md#service_instance_update)                         | **patch** /v2/service_instances/{instance_id}                                            | update a service instance                                   |


## Documentation For Models

 - [AsyncOperation](docs/AsyncOperation.md)
 - [Catalog](docs/Catalog.md)
 - [DashboardClient](docs/DashboardClient.md)
 - [Error](docs/Error.md)
 - [JsonSchema](docs/JsonSchema.md)
 - [LastOperationResource](docs/LastOperationResource.md)
 - [MaintenanceInfo](docs/MaintenanceInfo.md)
 - [Plan](docs/Plan.md)
 - [PositiveIntegerDefault0](docs/PositiveIntegerDefault0.md)
 - [Schema](docs/Schema.md)
 - [SchemaParameters](docs/SchemaParameters.md)
 - [Schemas](docs/Schemas.md)
 - [Service](docs/Service.md)
 - [ServiceBindingEndpoint](docs/ServiceBindingEndpoint.md)
 - [ServiceBindingRequest](docs/ServiceBindingRequest.md)
 - [ServiceBindingResouceObject](docs/ServiceBindingResouceObject.md)
 - [ServiceBindingResource](docs/ServiceBindingResource.md)
 - [ServiceBindingResponse](docs/ServiceBindingResponse.md)
 - [ServiceBindingSchema](docs/ServiceBindingSchema.md)
 - [ServiceBindingVolumeMount](docs/ServiceBindingVolumeMount.md)
 - [ServiceBindingVolumeMountDevice](docs/ServiceBindingVolumeMountDevice.md)
 - [ServiceInstanceAsyncOperation](docs/ServiceInstanceAsyncOperation.md)
 - [ServiceInstancePreviousValues](docs/ServiceInstancePreviousValues.md)
 - [ServiceInstanceProvisionRequest](docs/ServiceInstanceProvisionRequest.md)
 - [ServiceInstanceProvisionResponse](docs/ServiceInstanceProvisionResponse.md)
 - [ServiceInstanceResource](docs/ServiceInstanceResource.md)
 - [ServiceInstanceSchema](docs/ServiceInstanceSchema.md)
 - [ServiceInstanceUpdateRequest](docs/ServiceInstanceUpdateRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

open-service-broker-api@googlegroups.com

