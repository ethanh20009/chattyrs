use super::{DB_COLLECTION_NAME, DB_VEC_LENGTH};
use anyhow::{anyhow, Context, Result};
use qdrant_client::qdrant::{PointStruct, ScoredPoint, UpsertPoints, UpsertPointsBuilder};

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

impl TryFrom<ScoredPoint> for DbVector {
    type Error = anyhow::Error;
    fn try_from(value: ScoredPoint) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(Self {
            guild_id: value
                .payload
                .get("guild_id")
                .context("No Guild ID attached to vector payload")?
                .as_integer()
                .context("Guild ID on vector not a number")? as u64,
            message_id: match value
                .id
                .context("Vector missing ID")?
                .point_id_options
                .context("Cannot get point ID Options")?
            {
                qdrant_client::qdrant::point_id::PointIdOptions::Num(id) => id,
                _ => Err(anyhow!("Point id is not a number"))?,
            },
            vector: match value
                .vectors
                .context("Vector missing vectors")?
                .vectors_options
                .context("Vector missing vector options")?
            {
                qdrant_client::qdrant::vectors::VectorsOptions::Vector(vec) => vec.data,
                _ => Err(anyhow!(
                    "Qdrant vector options does not contain a single vector"
                ))?,
            },
            message: value
                .payload
                .get("message")
                .context("Vector missing message field")?
                .to_string(),
        })
    }
}
