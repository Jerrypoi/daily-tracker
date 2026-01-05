# Topic

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | Unique identifier for the topic | 
**topic_name** | **String** | Name of the topic (e.g., 'playing', 'working') | 
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Timestamp when the topic was created | 
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Timestamp when the topic was last updated | 
**parent_topic_id** | **i64** | ID of the parent topic (null for root-level topics) | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


