# DailyTrack

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **i64** | Unique identifier for the daily track record | 
**start_time** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Start time of the 30-minute period (must be at :00 or :30) | 
**created_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Timestamp when the record was created | 
**updated_at** | [**chrono::DateTime::<chrono::Utc>**](DateTime.md) | Timestamp when the record was last updated | 
**topic_id** | **i64** | ID of the associated topic | 
**comment** | **String** | Optional notes or comments for this time period | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


