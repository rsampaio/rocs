# \ServiceInstancesApi

All URIs are relative to *http://example.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**service_instance_deprovision**](ServiceInstancesApi.md#service_instance_deprovision) | **delete** /v2/service_instances/{instance_id} | deprovision a service instance
[**service_instance_get**](ServiceInstancesApi.md#service_instance_get) | **get** /v2/service_instances/{instance_id} | get a service instance
[**service_instance_last_operation_get**](ServiceInstancesApi.md#service_instance_last_operation_get) | **get** /v2/service_instances/{instance_id}/last_operation | get the last requested operation state for service instance
[**service_instance_provision**](ServiceInstancesApi.md#service_instance_provision) | **put** /v2/service_instances/{instance_id} | provision a service instance
[**service_instance_update**](ServiceInstancesApi.md#service_instance_update) | **patch** /v2/service_instances/{instance_id} | update a service instance



## service_instance_deprovision

> serde_json::Value service_instance_deprovision(x_broker_api_version, instance_id, service_id, plan_id, x_broker_api_originating_identity, accepts_incomplete)
deprovision a service instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_broker_api_version** | **String** | version number of the Service Broker API that the Platform will use | Required | [default to 2.13]
**instance_id** | **String** | id of instance being deleted | Required | 
**service_id** | **String** | id of the service associated with the instance being deleted | Required | 
**plan_id** | **String** | id of the plan associated with the instance being deleted | Required | 
**x_broker_api_originating_identity** | **String** | identity of the user that initiated the request from the Platform |  | 
**accepts_incomplete** | **bool** | asynchronous deprovision supported |  | 

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_instance_get

> crate::models::ServiceInstanceResource service_instance_get(x_broker_api_version, instance_id, x_broker_api_originating_identity, service_id, plan_id)
get a service instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_broker_api_version** | **String** | version number of the Service Broker API that the Platform will use | Required | [default to 2.13]
**instance_id** | **String** | instance id of instance to fetch | Required | 
**x_broker_api_originating_identity** | **String** | identity of the user that initiated the request from the Platform |  | 
**service_id** | **String** | id of the service associated with the instance |  | 
**plan_id** | **String** | id of the plan associated with the instance |  | 

### Return type

[**crate::models::ServiceInstanceResource**](ServiceInstanceResource.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_instance_last_operation_get

> crate::models::LastOperationResource service_instance_last_operation_get(x_broker_api_version, instance_id, service_id, plan_id, operation)
get the last requested operation state for service instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_broker_api_version** | **String** | version number of the Service Broker API that the Platform will use | Required | [default to 2.13]
**instance_id** | **String** | instance id of instance to find last operation applied to it | Required | 
**service_id** | **String** | id of the service associated with the instance |  | 
**plan_id** | **String** | id of the plan associated with the instance |  | 
**operation** | **String** | a provided identifier for the operation |  | 

### Return type

[**crate::models::LastOperationResource**](LastOperationResource.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_instance_provision

> crate::models::ServiceInstanceProvisionResponse service_instance_provision(x_broker_api_version, instance_id, service_instance_provision_request, x_broker_api_originating_identity, accepts_incomplete)
provision a service instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_broker_api_version** | **String** | version number of the Service Broker API that the Platform will use | Required | [default to 2.13]
**instance_id** | **String** | instance id of instance to provision | Required | 
**service_instance_provision_request** | [**ServiceInstanceProvisionRequest**](ServiceInstanceProvisionRequest.md) | parameters for the requested service instance provision | Required | 
**x_broker_api_originating_identity** | **String** | identity of the user that initiated the request from the Platform |  | 
**accepts_incomplete** | **bool** | asynchronous operations supported |  | 

### Return type

[**crate::models::ServiceInstanceProvisionResponse**](ServiceInstanceProvisionResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_instance_update

> serde_json::Value service_instance_update(x_broker_api_version, instance_id, service_instance_update_request, x_broker_api_originating_identity, accepts_incomplete)
update a service instance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_broker_api_version** | **String** | version number of the Service Broker API that the Platform will use | Required | [default to 2.13]
**instance_id** | **String** | instance id of instance to update | Required | 
**service_instance_update_request** | [**ServiceInstanceUpdateRequest**](ServiceInstanceUpdateRequest.md) | parameters for the requested service instance update | Required | 
**x_broker_api_originating_identity** | **String** | identity of the user that initiated the request from the Platform |  | 
**accepts_incomplete** | **bool** | asynchronous operations supported |  | 

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

