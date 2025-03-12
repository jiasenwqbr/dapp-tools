use lapin::{options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties};
use tokio_executor_trait::Tokio;

pub struct RabbitMQ {
    uri: String,
}

impl RabbitMQ {
    pub async fn new(uri: &str) -> Self {
        Self {
            uri: uri.to_string(),
        }
    }

    pub async fn send_message(&self, queue: &str, message: &str) -> Result<(), lapin::Error> {
        let conn =
            Connection::connect(&self.uri, ConnectionProperties::default().with_tokio()).await?;
        let channel = conn.create_channel().await?;

        channel
            .queue_declare(queue, QueueDeclareOptions::default(), FieldTable::default())
            .await?;
        channel
            .basic_publish(
                "",
                queue,
                BasicPublishOptions::default(),
                message.as_bytes().to_vec(),
                BasicProperties::default(),
            )
            .await?;
        Ok(())
    }
}
