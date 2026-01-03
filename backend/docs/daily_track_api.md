# daily_track_api

All URIs are relative to *http://localhost:8080/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**createDailyTrack**](daily_track_api.md#createDailyTrack) | **POST** /daily-tracks | Create a new daily track record
**getDailyTracks**](daily_track_api.md#getDailyTracks) | **GET** /daily-tracks | Get daily track records
**getDailyTrackById**](daily_track_api.md#getDailyTrackById) | **GET** /daily-tracks/{id} | Get a daily track record by ID


# **createDailyTrack**
> models::DailyTrack createDailyTrack(body)
Create a new daily track record

Creates a new daily track record. The start_time must be at :00 or :30 minutes of an hour.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **body** | [**CreateDailyTrackRequest**](CreateDailyTrackRequest.md)| Daily track record to be created | 

### Return type

[**models::DailyTrack**](DailyTrack.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getDailyTracks**
> Vec<models::DailyTrack> getDailyTracks(optional)
Get daily track records

Retrieves a list of daily track records. Supports filtering by date range and topic.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **start_date** | **chrono::naive::NaiveDate**| Filter records starting from this date (inclusive). Format: YYYY-MM-DD | 
 **end_date** | **chrono::naive::NaiveDate**| Filter records up to this date (inclusive). Format: YYYY-MM-DD | 
 **topic_id** | **i64**| Filter records by topic ID | 

### Return type

[**Vec<models::DailyTrack>**](DailyTrack.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getDailyTrackById**
> models::DailyTrack getDailyTrackById(id)
Get a daily track record by ID

Retrieves a single daily track record by its ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **i64**| ID of the daily track record to retrieve | 

### Return type

[**models::DailyTrack**](DailyTrack.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

