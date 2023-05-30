use crate::quic::sender::Sender;
use anyhow::Result;
use shared::error::Error;
use shared::streams::delete_stream::DeleteStream;
use std::sync::Arc;
use streaming::system::System;
use tokio::sync::RwLock;
use tracing::trace;

pub async fn handle(
    command: DeleteStream,
    sender: &mut Sender,
    system: Arc<RwLock<System>>,
) -> Result<(), Error> {
    trace!("{}", command);
    let mut system = system.write().await;
    system.delete_stream(command.stream_id).await?;
    sender.send_empty_ok_response().await?;
    Ok(())
}
