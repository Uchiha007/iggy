use crate::quic::sender::Sender;
use anyhow::Result;
use shared::error::Error;
use shared::topics::create_topic::CreateTopic;
use std::sync::Arc;
use streaming::system::System;
use tokio::sync::RwLock;
use tracing::trace;

pub async fn handle(
    command: CreateTopic,
    sender: &mut Sender,
    system: Arc<RwLock<System>>,
) -> Result<(), Error> {
    trace!("{}", command);
    let mut system = system.write().await;
    system
        .get_stream_mut(command.stream_id)?
        .create_topic(command.topic_id, &command.name, command.partitions_count)
        .await?;
    sender.send_empty_ok_response().await?;
    Ok(())
}
