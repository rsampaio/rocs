# Schema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | [optional] 
**schema** | **String** |  | [optional] 
**title** | **String** |  | [optional] 
**description** | **String** |  | [optional] 
**default** | [***serde_json::Value**](.md) |  | [optional] 
**multiple_of** | **f32** |  | [optional] 
**maximum** | **f32** |  | [optional] 
**exclusive_maximum** | **bool** |  | [optional] [default to false]
**minimum** | **f32** |  | [optional] 
**exclusive_minimum** | **bool** |  | [optional] [default to false]
**max_length** | **i32** |  | [optional] 
**min_length** | [***crate::models::PositiveIntegerDefault0**](positiveIntegerDefault0.md) |  | [optional] 
**pattern** | **String** |  | [optional] 
**additional_items** | [***crate::models::AnyOfboolean**](anyOf<boolean,#>.md) |  | [optional] [default to {}]
**items** | [***crate::models::AnyOfschemaArray**](anyOf<#,schemaArray>.md) |  | [optional] [default to {}]
**max_items** | **i32** |  | [optional] 
**min_items** | [***crate::models::PositiveIntegerDefault0**](positiveIntegerDefault0.md) |  | [optional] 
**unique_items** | **bool** |  | [optional] [default to false]
**max_properties** | **i32** |  | [optional] 
**min_properties** | [***crate::models::PositiveIntegerDefault0**](positiveIntegerDefault0.md) |  | [optional] 
**required** | [***crate::models::StringArray**](stringArray.md) |  | [optional] 
**additional_properties** | [***crate::models::AnyOfboolean**](anyOf<boolean,#>.md) |  | [optional] [default to {}]
**definitions** | [**::std::collections::HashMap<String, crate::models::>**](#.md) |  | [optional] [default to {}]
**properties** | [**::std::collections::HashMap<String, crate::models::>**](#.md) |  | [optional] [default to {}]
**pattern_properties** | [**::std::collections::HashMap<String, crate::models::>**](#.md) |  | [optional] [default to {}]
**dependencies** | [**::std::collections::HashMap<String, crate::models::AnyOfstringArray>**](anyOf<#,stringArray>.md) |  | [optional] 
**_enum** | **Vec<String>** |  | [optional] 
**_type** | [***crate::models::AnyOfsimpleTypesarray**](anyOf<simpleTypes,array>.md) |  | [optional] 
**format** | **String** |  | [optional] 
**all_of** | [***crate::models::SchemaArray**](schemaArray.md) |  | [optional] 
**any_of** | [***crate::models::SchemaArray**](schemaArray.md) |  | [optional] 
**one_of** | [***crate::models::SchemaArray**](schemaArray.md) |  | [optional] 
**not** | [***crate::models::**](#.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


