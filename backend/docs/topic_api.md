# topic_api

All URIs are relative to *http://localhost:8080/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**createTopic**](topic_api.md#createTopic) | **POST** /topics | Create a new topic
**getTopics**](topic_api.md#getTopics) | **GET** /topics | Get all topics
**getTopicById**](topic_api.md#getTopicById) | **GET** /topics/{id} | Get a topic by ID


# **createTopic**
> models::Topic createTopic(body)
Create a new topic

Creates a new topic with the given name and optional parent topic.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**CreateTopicRequest**](CreateTopicRequest.md)| Topic object to be created | 

### Return type

[**models::Topic**](Topic.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTopics**
> Vec<models::Topic> getTopics(optional)
Get all topics

Retrieves a list of all topics. Supports optional filtering by parent_topic_id.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **parent_topic_id** | **i64**| Filter topics by parent topic ID. | 

### Return type

[**Vec<models::Topic>**](Topic.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getTopicById**
> models::Topic getTopicById(id)
Get a topic by ID

Retrieves a single topic by its ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **i64**| ID of the topic to retrieve | 

### Return type

[**models::Topic**](Topic.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

