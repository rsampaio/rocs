# \CatalogApi

All URIs are relative to *http://example.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**catalog_get**](CatalogApi.md#catalog_get) | **get** /v2/catalog | get the catalog of services that the service broker offers



## catalog_get

> crate::models::Catalog catalog_get(x_broker_api_version)
get the catalog of services that the service broker offers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_broker_api_version** | **String** | version number of the Service Broker API that the Platform will use | Required | [default to 2.13]

### Return type

[**crate::models::Catalog**](Catalog.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

