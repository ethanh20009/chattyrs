use super::{DB_COLLECTION_NAME, DB_VEC_LENGTH};
use anyhow::{anyhow, Result};
use qdrant_client::qdrant::{PointStruct, UpsertPoints, UpsertPointsBuilder};

pub struct DbVector {
    pub vector: Vec<f32>,
    pub message: String,
    pub message_id: u64,
    pub guild_id: u64,
}

impl DbVector {
    pub fn new(
        vector: Vec<f32>,
        message: impl ToString,
        message_id: u64,
        guild_id: u64,
    ) -> Result<Self> {
        let message = message.to_string();
        if vector.len() != DB_VEC_LENGTH as usize {
            return Err(anyhow!(
                "Wrong vector size, expected: {} found {}",
                DB_VEC_LENGTH,
                vector.len()
            ));
        }
        Ok(Self {
            vector,
            message,
            message_id,
            guild_id,
        })
    }
}

impl From<DbVector> for UpsertPoints {
    fn from(value: DbVector) -> Self {
        let point_struct = PointStruct::new(
            value.message_id,
            value.vector,
            [
                ("message", value.message.into()),
                ("guild_id", value.guild_id.to_string().into()),
            ],
        );
        UpsertPointsBuilder::new(DB_COLLECTION_NAME, vec![point_struct]).build()
    }
}
