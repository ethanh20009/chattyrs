use qdrant_client::{
    qdrant::{CreateCollectionBuilder, ListCollectionsResponse, VectorParamsBuilder},
    Qdrant,
};

use crate::environment::Environment;
use anyhow::{Context, Result};

pub struct VdbHandler {
    client: Qdrant,
}

const DB_COLLECTION_NAME: &str = "messages";
const DB_VEC_LENGTH: u64 = 1024;

impl VdbHandler {
    pub async fn new(env: &Environment) -> Result<Self> {
        let client = Qdrant::from_url(&env.vdb.base_url)
            .build()
            .context("Failed to build qdrant client")?;

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
}
