pub struct GetTopicsResponse {
    pub topics: Vec<Topic>,
}

pub struct Topic {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
