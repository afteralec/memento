use super::traits::Resolver;
use anyhow::Result;
use futures::{stream::SplitStream, StreamExt};
use std::fmt;
use tokio::{macros::support::Future, net::TcpStream, sync::mpsc};
use tokio_util::codec::{Framed, LinesCodec};

pub async fn resolve_receiver<T, R>(
    mut receiver: mpsc::UnboundedReceiver<T>,
    mut resolver: R,
) -> Result<()>
where
    T: 'static + Send + Sync + fmt::Debug,
    R: 'static + Send + Sync + fmt::Debug + Resolver<T>,
{
    while let Some(event) = receiver.recv().await {
        resolver.resolve_on(event)?;
    }

    // @TODO: Figure out how to reattach this receiver to the spawning struct
    Ok(())
}

pub async fn resolve_stream_and_receiver<T, R, S>(
    mut stream: SplitStream<Framed<TcpStream, LinesCodec>>,
    mut stream_resolver: S,
    mut receiver: mpsc::UnboundedReceiver<T>,
    mut resolver: R,
) -> Result<()>
where
    T: 'static + Send + Sync + fmt::Debug,
    R: 'static + Send + Sync + fmt::Debug + Resolver<T>,
    S: 'static + Send + Sync + fmt::Debug + Resolver<String>,
{
    loop {
        tokio::select!(
             Some(event) = receiver.recv() => {
                 resolver.resolve_on(event)?;
             }
             input = stream.next() => match input {
                 Some(Ok(input)) => {
                     stream_resolver.resolve_on(input)?;
                 },
                 Some(Err(_err)) => {
                     // @TODO: Here, there is a LinesCodecError from the underlying SplitStream - handle it gracefully
                     break;
                 },
                 None => {
                     // @TODO: Here, the stream is now closed - the player is disconnected. Handle this gracefully.
                     break;
                 }
             }
        );
    }

    Ok(())
}

pub fn spawn_and_trace<F>(f: F) -> tokio::task::JoinHandle<()>
where
    F: Future<Output = Result<()>> + Send + 'static,
{
    tokio::spawn(async move {
        if let Err(err) = f.await {
            tracing::error!("{:#?}", err);
        }
    })
}
