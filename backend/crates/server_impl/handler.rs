/// Get all topics
async fn get_topics(
    parent_topic_id: Option<i64>,
) -> Result<GetTopicsResponse, ApiError> {
    info!(
        "get_topics({:?}) - X-Span-ID: {:?}",
        parent_topic_id,
        context.get().0.clone()
    );
    Err(ApiError("Api-Error: Operation is NOT implemented".into()))
}
