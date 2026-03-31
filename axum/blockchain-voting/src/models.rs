use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Block {
    pub id: i32,
    pub block_index: i32,
    pub voter_id: String,
    pub candidate: String,
    pub prev_hash: String,
    pub block_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VoteRequest {
    pub voter_id: String,
    pub candidate: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BlockchainStats {
    pub total_votes: i64,
    pub is_valid: bool,
}
