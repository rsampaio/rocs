# Service

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**id** | **String** |  | 
**description** | **String** |  | 
**tags** | **Vec<String>** |  | [optional] 
**requires** | **Vec<String>** |  | [optional] 
**bindable** | **bool** |  | 
**metadata** | [***serde_json::Value**](.md) | See [Service Metadata Conventions](https://github.com/openservicebrokerapi/servicebroker/blob/master/profile.md#service-metadata) for more details. | [optional] 
**dashboard_client** | [***crate::models::DashboardClient**](DashboardClient.md) |  | [optional] 
**plan_updateable** | **bool** |  | [optional] 
**plans** | [**Vec<crate::models::Plan>**](Plan.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


