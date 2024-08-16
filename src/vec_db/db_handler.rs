use qdrant_client::{
    qdrant::{CreateCollectionBuilder, ListCollectionsResponse, VectorParamsBuilder},
    Qdrant,
};

use crate::environment::Environment;
use anyhow::{Context, Result};

use super::{vector::DbVector, DB_COLLECTION_NAME, DB_VEC_LENGTH};

pub struct VdbHandler {
    client: Qdrant,
}

impl VdbHandler {
    pub async fn new(env: &Environment) -> Result<Self> {
        let client = Qdrant::from_url(&env.vdb.base_url)
            .build()
            .context("Failed to build qdrant client")?;

        Self::initialise_collection(&client).await?;

        Ok(Self { client })
    }

    async fn initialise_collection(client: &Qdrant) -> Result<()> {
        match client.collection_exists(DB_COLLECTION_NAME).await? {
            true => Ok(()),
            false => {
                let vectors_config = VectorParamsBuilder::new(
                    DB_VEC_LENGTH,
                    qdrant_client::qdrant::Distance::Euclid,
                );
                let collection =
                    CreateCollectionBuilder::new(DB_COLLECTION_NAME).vectors_config(vectors_config);
                client.create_collection(collection).await?;
                Ok(())
            }
        }
    }

    pub async fn add_vector(
        &self,
        vector: Vec<f32>,
        message: impl ToString,
        message_id: u64,
    ) -> Result<()> {
        let db_vec = DbVector::new(vector, message, message_id)?;
        self.client
            .upsert_points(db_vec)
            .await
            .context("Failed to insert vector into database")
            .map(|_| ())
    }
}
