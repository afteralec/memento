use super::resolver::StreamResolver;
use crate::{messaging::traits::Resolver, stream::types::Stream};
use anyhow::Result;
use futures::StreamExt;

pub async fn resolve_stream(mut stream: Stream, mut resolver: StreamResolver) -> Result<()> {
    while let Some(input) = stream.next().await {
        match input {
            Ok(input) => {
                resolver.resolve_on(input)?;
            }
            Err(_err) => {
                // @TODO: Here, there is a LinesCodecError from the underlying SplitStream - handle it gracefully
                break;
            }
        }
    }

    // @TODO: Here, the stream is now closed - the player is disconnected. Handle this gracefully.
    Ok(())
}
