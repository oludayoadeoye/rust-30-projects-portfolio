use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::PgPool;
use sha2::{Sha256, Digest};
use hex;
use crate::models::{Block, VoteRequest, BlockchainStats};

fn calculate_hash(index: i32, voter_id: &str, candidate: &str, prev_hash: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}{}{}{}", index, voter_id, candidate, prev_hash));
    hex::encode(hasher.finalize())
}

#[utoipa::path(
    get,
    path = "/blocks",
    responses(
        (status = 200, description = "List all voting blocks", body = [Block])
    )
)]
pub async fn list_blocks(State(pool): State<PgPool>) -> impl IntoResponse {
    let blocks = sqlx::query_as::<_, Block>("SELECT * FROM blocks ORDER BY block_index ASC")
        .fetch_all(&pool)
        .await;

    match blocks {
        Ok(blocks) => (StatusCode::OK, Json(blocks)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching blocks").into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/vote",
    request_body = VoteRequest,
    responses(
        (status = 201, description = "Vote recorded", body = Block)
    )
)]
pub async fn cast_vote(
    State(pool): State<PgPool>,
    Json(payload): Json<VoteRequest>,
) -> impl IntoResponse {
    // 1. Get last block to get index and prev_hash
    let last_block = sqlx::query_as::<_, Block>("SELECT * FROM blocks ORDER BY block_index DESC LIMIT 1")
        .fetch_optional(&pool)
        .await;

    let (next_index, prev_hash) = match last_block {
        Ok(Some(b)) => (b.block_index + 1, b.block_hash),
        _ => (0, "0".to_string()), // Genesis block
    };

    // 2. Calculate new hash
    let block_hash = calculate_hash(next_index, &payload.voter_id, &payload.candidate, &prev_hash);

    // 3. Insert block
    let block = sqlx::query_as::<_, Block>(
        "INSERT INTO blocks (block_index, voter_id, candidate, prev_hash, block_hash) \
         VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(next_index)
    .bind(payload.voter_id)
    .bind(payload.candidate)
    .bind(prev_hash)
    .bind(block_hash)
    .fetch_one(&pool)
    .await;

    match block {
        Ok(b) => (StatusCode::CREATED, Json(b)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Error recording vote").into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/stats",
    responses(
        (status = 200, description = "Get blockchain stats", body = BlockchainStats)
    )
)]
pub async fn get_stats(State(pool): State<PgPool>) -> impl IntoResponse {
    let blocks = sqlx::query_as::<_, Block>("SELECT * FROM blocks ORDER BY block_index ASC")
        .fetch_all(&pool)
        .await;

    let mut is_valid = true;
    let mut total_votes = 0;

    if let Ok(blocks) = blocks {
        total_votes = blocks.len() as i64;
        let mut prev_hash = "0".to_string();

        for block in blocks {
            // 1. Check prev_hash matches
            if block.prev_hash != prev_hash {
                is_valid = false;
                break;
            }

            // 2. Re-calculate hash and check it matches block_hash
            let calculated = calculate_hash(
                block.block_index,
                &block.voter_id,
                &block.candidate,
                &block.prev_hash
            );

            if calculated != block.block_hash {
                is_valid = false;
                break;
            }

            prev_hash = block.block_hash;
        }
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching blockchain").into_response();
    }

    let stats = BlockchainStats {
        total_votes,
        is_valid,
    };

    (StatusCode::OK, Json(stats)).into_response()
}
