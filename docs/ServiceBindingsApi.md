# \ServiceBindingsApi

All URIs are relative to *http://example.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**service_binding_binding**](ServiceBindingsApi.md#service_binding_binding) | **put** /v2/service_instances/{instance_id}/service_bindings/{binding_id} | generate a service binding
[**service_binding_get**](ServiceBindingsApi.md#service_binding_get) | **get** /v2/service_instances/{instance_id}/service_bindings/{binding_id} | get a service binding
[**service_binding_last_operation_get**](ServiceBindingsApi.md#service_binding_last_operation_get) | **get** /v2/service_instances/{instance_id}/service_bindings/{binding_id}/last_operation | get the last requested operation state for service binding
[**service_binding_unbinding**](ServiceBindingsApi.md#service_binding_unbinding) | **delete** /v2/service_instances/{instance_id}/service_bindings/{binding_id} | deprovision a service binding



## service_binding_binding

> crate::models::ServiceBindingResponse service_binding_binding(x_broker_api_version, instance_id, binding_id, service_binding_request, x_broker_api_originating_identity, accepts_incomplete)
generate a service binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_broker_api_version** | **String** | version number of the Service Broker API that the Platform will use | Required | [default to 2.13]
**instance_id** | **String** | instance id of instance to create a binding on | Required | 
**binding_id** | **String** | binding id of binding to create | Required | 
**service_binding_request** | [**ServiceBindingRequest**](ServiceBindingRequest.md) | parameters for the requested service binding | Required | 
**x_broker_api_originating_identity** | **String** | identity of the user that initiated the request from the Platform |  | 
**accepts_incomplete** | **bool** | asynchronous operations supported |  | 

### Return type

[**crate::models::ServiceBindingResponse**](ServiceBindingResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_binding_get

> crate::models::ServiceBindingResource service_binding_get(x_broker_api_version, instance_id, binding_id, x_broker_api_originating_identity, service_id, plan_id)
get a service binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_broker_api_version** | **String** | version number of the Service Broker API that the Platform will use | Required | [default to 2.13]
**instance_id** | **String** | instance id of instance associated with the binding | Required | 
**binding_id** | **String** | binding id of binding to fetch | Required | 
**x_broker_api_originating_identity** | **String** | identity of the user that initiated the request from the Platform |  | 
**service_id** | **String** | id of the service associated with the instance |  | 
**plan_id** | **String** | id of the plan associated with the instance |  | 

### Return type

[**crate::models::ServiceBindingResource**](ServiceBindingResource.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_binding_last_operation_get

> crate::models::LastOperationResource service_binding_last_operation_get(x_broker_api_version, instance_id, binding_id, service_id, plan_id, operation)
get the last requested operation state for service binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_broker_api_version** | **String** | version number of the Service Broker API that the Platform will use | Required | [default to 2.13]
**instance_id** | **String** | instance id of instance to find last operation applied to it | Required | 
**binding_id** | **String** | binding id of service binding to find last operation applied to it | Required | 
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


## service_binding_unbinding

> serde_json::Value service_binding_unbinding(x_broker_api_version, instance_id, binding_id, service_id, plan_id, x_broker_api_originating_identity, accepts_incomplete)
deprovision a service binding

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_broker_api_version** | **String** | version number of the Service Broker API that the Platform will use | Required | [default to 2.13]
**instance_id** | **String** | id of the instance associated with the binding being deleted | Required | 
**binding_id** | **String** | id of the binding being deleted | Required | 
**service_id** | **String** | id of the service associated with the instance for which a binding is being deleted | Required | 
**plan_id** | **String** | id of the plan associated with the instance for which a binding is being deleted | Required | 
**x_broker_api_originating_identity** | **String** | identity of the user that initiated the request from the Platform |  | 
**accepts_incomplete** | **bool** | asynchronous operations supported |  | 

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

