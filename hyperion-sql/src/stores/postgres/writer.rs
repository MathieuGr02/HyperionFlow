use hyperion_core::{components::writer_trait::StoreWriter, message::DeserializedMessage};

use crate::stores::postgres::client::PostgreSQLClient;

pub struct PostgreSQLWriter {
    pub client: PostgreSQLClient,
    pub query: String,
}

impl StoreWriter for PostgreSQLWriter {
    fn write(
        &mut self,
        item: DeserializedMessage,
    ) -> Result<(), hyperion_core::components::writer_trait::WriteError> {
        todo!()
    }

    fn write_batch(
        &mut self,
        items: Vec<DeserializedMessage>,
    ) -> Result<(), hyperion_core::components::writer_trait::WriteError> {
        todo!()
    }
}
